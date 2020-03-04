#![no_std]
#[macro_use]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub struct EnvironmentalVariable {
    pub name: String,
    pub value: String,
}

pub struct AccessibleDirectory {
    pub path: String,
    pub fd: u32,
}

pub fn print(message: &str) {
    unsafe {
        let stdout = 1;
        let data = [wasi::Ciovec {
            buf: message.as_ptr(),
            buf_len: message.len(),
        }];
        wasi::fd_write(stdout, &data).unwrap();
    }
}

pub fn println(message: &str) {
    let mut m = String::from(message);
    m.push_str("\n");
    print(&m);
}

pub fn error(message: &str) {
    let mut m = String::from(message);
    m.push_str("\n");
    unsafe {
        let stdout = 2;
        let data = [wasi::Ciovec {
            buf: m.as_ptr(),
            buf_len: m.len(),
        }];
        wasi::fd_write(stdout, &data).unwrap();
    }
}

pub fn command_line_arguments() -> Vec<String> {
    let mut cmd_args: Vec<String> = vec![];
    unsafe {
        let (ct, len) = wasi::args_sizes_get().unwrap();
        let mut cmd_arg_positions: Vec<i32> = vec![0; ct];
        let mut cmd_arg_data: Vec<u8> = vec![0; len];
        wasi::args_get(
            cmd_arg_positions.as_mut_ptr() as *mut *mut u8,
            cmd_arg_data.as_mut_ptr() as *mut u8,
        )
        .unwrap();
        for x in 0..ct {
            let base = cmd_arg_data.as_mut_ptr() as usize;
            let beg = cmd_arg_positions[x] as usize - base;
            let end = if x == ct - 1 {
                cmd_arg_data.len() - 1
            } else {
                cmd_arg_positions[x + 1] as usize - 1 - base
            };
            let cmd_arg = String::from_utf8_lossy(&cmd_arg_data[beg..end]);
            cmd_args.push(cmd_arg.to_string());
        }
    }
    cmd_args
}

pub fn environment_variables() -> Vec<EnvironmentalVariable> {
    let mut envs: Vec<EnvironmentalVariable> = vec![];
    unsafe {
        let (ct, len) = wasi::environ_sizes_get().unwrap();
        let mut env_positions: Vec<i32> = vec![0; ct];
        let mut env_data: Vec<u8> = vec![0; len];
        wasi::environ_get(
            env_positions.as_mut_ptr() as *mut *mut u8,
            env_data.as_mut_ptr() as *mut u8,
        )
        .unwrap();
        for x in 0..ct {
            let base = env_data.as_mut_ptr() as usize;
            let beg = env_positions[x] as usize - base;
            let end = if x == ct - 1 {
                env_data.len() - 1
            } else {
                env_positions[x + 1] as usize - 1 - base
            };
            let pair = String::from_utf8_lossy(&env_data[beg..end]);
            let p: Vec<&str> = pair.splitn(2, '=').collect();
            envs.push(EnvironmentalVariable {
                name: p[0].to_string(),
                value: p[1].to_string(),
            });
        }
    }
    envs
}

pub fn random_number() -> f32 {
    unsafe {
        let mut noise: Vec<u8> = vec![0; 8];
        let max = 0xFFFFFFFFFFFFFFFF as u64;
        let mut array = [0 as u8; 8];
        wasi::random_get(noise.as_mut_ptr() as *mut u8, 8).unwrap();
        array.copy_from_slice(&noise);
        let value = u64::from_be_bytes(array);
        value as f32 / max as f32
    }
}

pub fn sleep(millis: usize) {
    unsafe {
        let t = current_time();
        loop {
            wasi::sched_yield().unwrap();
            let nt = current_time();
            if nt - t >= millis as u64 {
                break;
            }
        }
    }
}

pub fn exit() {
    unsafe {
        wasi::proc_exit(0);
    }
}

pub fn high_precision_time() -> u64 {
    unsafe {
        // precision appears is ignored?
        let precision = 0;
        wasi::clock_time_get(wasi::CLOCKID_REALTIME, precision).unwrap() as u64
    }
}

pub fn current_time() -> u64 {
    unsafe {
        // precision appears is ignored?
        let precision = 0;
        wasi::clock_time_get(wasi::CLOCKID_REALTIME, precision).unwrap() as u64 / 1000000
    }
}

pub fn unix_time() -> u64 {
    unsafe {
        // precision appears is ignored?
        let precision = 0;
        wasi::clock_time_get(wasi::CLOCKID_REALTIME, precision).unwrap() as u64 / 1000000000
    }
}

pub fn read_text(path: &str) -> Result<String, String> {
    if let Some(fd) = get_owning_directory(path) {
        Ok("found".to_string())
    } else {
        Err("no access to file".to_string())
    }
    /*unsafe {

        let mut fs_rights_base = 0;
        fs_rights_base |= wasi::RIGHTS_FD_READ;
        fs_rights_base |= wasi::RIGHTS_FD_READDIR;
        fs_rights_base |= wasi::RIGHTS_PATH_FILESTAT_GET;
        fs_rights_base |= wasi::RIGHTS_PATH_OPEN;
        let mut fd = wasi::path_open(
                self.fd,
                dirflags,
                path,
                oflags,
                fs_rights_base,surrenders
                fs_rights_inheriting,
                fs_flags
            ).unwrap();
    }*/
}

pub fn write_text(path: &str, _data: &str) -> Result<(), String> {
    if let Some(fd) = get_owning_directory(path) {
        Ok(())
    } else {
        Err("no access to file".to_string())
    }
    // todo
}

pub fn accessible_directories() -> Vec<AccessibleDirectory> {
    let mut dirs = vec![];
    let mut i = 3;
    loop {
        let path = unsafe {
            let len = 1024;
            let mut path_data: Vec<u8> = vec![0; len];
            if let Ok(_) = wasi::fd_prestat_dir_name(i, path_data.as_mut_ptr() as *mut u8, len) {
                String::from_utf8_lossy(&path_data)
                    .trim_end_matches("\0")
                    .to_string()
            } else {
                break;
            }
        };
        dirs.push(AccessibleDirectory {
            fd: i,
            path: path.to_string(),
        });
        i = i + 1;
    }

    dirs
}

fn get_owning_directory(path: &str) -> Option<AccessibleDirectory> {
    let mut d = None;
    let dirs = accessible_directories();
    for dir in dirs.into_iter() {
        if path.starts_with(&dir.path) {
            d = Some(dir);
            break;
        }
    }
    d
}

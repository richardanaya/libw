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

pub fn read_line() -> String {
    let mut line = read_str(0);
    let len = line.len();
    line.truncate(len-1);
    line
}

pub fn print(message: &str) {
    write_str(1, message)
}

fn write_str(fd: u32, s: &str) {
    unsafe {
        let data = [wasi::Ciovec {
            buf: s.as_ptr(),
            buf_len: s.len(),
        }];
        wasi::fd_write(fd, &data).unwrap();
    }
}

fn read_str(fd: u32) -> String {
    unsafe {
        let len = 10240;
        let mut path_data: Vec<u8> = vec![0; len];
        let data = [wasi::Iovec {
            buf: path_data.as_mut_ptr(),
            buf_len: path_data.len(),
        }];
        wasi::fd_read(fd, &data).unwrap();
        String::from_utf8_lossy(&path_data)
            .trim_end_matches("\0")
            .to_string()
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
    write_str(2, &m)
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

pub fn exit() -> ! {
    unsafe {
        wasi::proc_exit(0);
        panic!("should not be here")
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
    if let Some(parent_dir) = get_owning_directory(path) {
        let rel_path = relative_path(&parent_dir.path, path);
        let fd = open_file(parent_dir.fd, &rel_path)?;
        let data = read_str(fd);
        close_file(fd);
        Ok(data)
    } else {
        Err("no access to file".to_string())
    }
}

fn open_file(dir_fd: u32, relative_path: &str) -> Result<u32, String> {
    let mut oflags = 0;
    oflags |= wasi::OFLAGS_CREAT;
    oflags |= wasi::OFLAGS_TRUNC;
    unsafe {
        match wasi::path_open(
            dir_fd,
            0,
            relative_path,
            oflags,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0,
        ) {
            Ok(fd) => Ok(fd),
            Err(_) => Err("Something went wrong opening file, did you give access?".to_string()),
        }
    }
}

fn close_file(fd: u32) {
    unsafe { wasi::fd_close(fd).unwrap() }
}

fn relative_path(base_dir: &str, path: &str) -> String {
    let rel_path = &path[base_dir.len()..];
    rel_path.to_string()
}

pub fn write_text(path: &str, data: &str) -> Result<(), String> {
    if let Some(parent_dir) = get_owning_directory(path) {
        let rel_path = relative_path(&parent_dir.path, path);
        let fd = open_file(parent_dir.fd, &rel_path)?;
        write_str(fd, data);
        close_file(fd);
        Ok(())
    } else {
        Err("no access to file".to_string())
    }
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
    // get the most specific folder
    let mut len = 0;
    for dir in dirs.into_iter() {
        let dir_path_len = dir.path.len();
        if path.starts_with(&dir.path) && dir_path_len > len {
            d = Some(dir);
            len = dir_path_len
        }
    }
    d
}

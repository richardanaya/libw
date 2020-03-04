#![no_std]
#[macro_use]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub struct EnvironmentalVariable {
    pub name: String,
    pub value: String,
}

pub struct File {
    pub is_directory: bool,
    pub fd: usize,
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

pub fn executing_directory() -> File {
    File {
        fd: 3,
        is_directory: true,
    }
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

pub fn yield_control() {
    unsafe {
        wasi::sched_yield().unwrap();
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

pub fn read_text(_path: &str) -> String {
    // todo
    "".to_string()
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

pub fn write_text(_path: &str, _data: &str) {
    // todo
}

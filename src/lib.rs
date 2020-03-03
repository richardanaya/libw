 #![no_std]
 #[macro_use]
 extern crate alloc;
 use alloc::string::{String,ToString};
 use alloc::vec::Vec;

 pub struct EnvironmentalVariable {
     pub name:String,
     pub value:String,
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
    let mut  m = String::from(message);
    m.push_str("\n");
    print(&m);
 }

 pub fn environment_variables() -> Vec<EnvironmentalVariable> {
    let mut envs: Vec<EnvironmentalVariable> = vec![];
    unsafe {
        let (ct,len) = wasi::environ_sizes_get().unwrap();
        let mut env_positions:Vec<i32> = vec![0; ct];
        let mut env_data:Vec<u8> = vec![0; len];
        wasi::environ_get(env_positions.as_mut_ptr() as *mut *mut u8,env_data.as_mut_ptr() as *mut u8).unwrap();
        for x in 0..ct {
            let base = env_data.as_mut_ptr() as usize;
            let beg = env_positions[x] as usize - base;
            let end = if x == ct-1 {
                env_data.len()-1
            } else {
                env_positions[x+1] as usize -1 - base
            };
            let pair = String::from_utf8_lossy(&env_data[beg..end]);
            let p:Vec<&str> = pair.splitn(2, '=').collect();
            envs.push(EnvironmentalVariable{
                name:p[0].to_string(),
                value:p[1].to_string(),
            });
        }
    }
    envs
 }

 pub fn read_text(path: &str) -> String {
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
                fs_rights_base,
                fs_rights_inheriting,
                fs_flags
            ).unwrap();
    }*/
 }

 pub fn write_text(_path: &str, _data: &str) {
    
 }
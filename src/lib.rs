 #![no_std]

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
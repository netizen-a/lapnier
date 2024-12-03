use crate::hcf;
use crate::io;
use arrayvec::ArrayString;
use core::fmt::Write;

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        let _ = io::cls();
        let _ = io::kprint(b"kernel panicked!\r\n", 0x00ff0000, 0);
    }
    if let Some(location) = info.location() {
        let mut line_astr = ArrayString::<50>::new();
        if let Ok(_) = write!(line_astr, "line: {}\r\n", location.line()) {
            unsafe {
                let _ = io::kprint(line_astr.as_str(), 0x00ff0000, 0);
            }
        } else {
            unsafe {
                let _ = io::kprint(b"error: failed convert line\r\n", 0x00ff0000, 0);
            }
        }
        unsafe {
            let _ = io::kprint(b"file: ", 0x00ff0000, 0);
            let _ = io::kprint(location.file().as_bytes(), 0x00ff0000, 0);
            let _ = io::kprint(b"\r\n", 0x00ff0000, 0);
        }
    }
    if let Some(msg) = info.message().as_str() {
        unsafe {
            let _ = io::kprint(b"msg: ", 0x00ff0000, 0);
            let _ = io::kprint(msg.as_bytes(), 0x00ff0000, 0);
            let _ = io::kprint(b"\r\n", 0x00ff0000, 0);
        }
    }
    hcf();
}

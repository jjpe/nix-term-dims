#[cfg(target_os = "macos")]
use libc::O_EVTONLY;
use libc::{O_NONBLOCK, TIOCGWINSZ, c_int, ioctl, open, winsize};
use std::{ffi::CString, str::FromStr};

#[cfg(target_os = "linux")]
pub fn get_tty_fd(tty_path: &str) -> Result<c_int, String> {
    let cpath = CString::from_str(tty_path).expect("Failed to init cpath");
    let tty_fd: c_int = unsafe { open(cpath.as_ptr(), O_NONBLOCK) };
    if tty_fd < 0 {
        Err(format!("Failed to open {tty_path}"))
    } else {
        Ok(tty_fd)
    }
}

#[cfg(target_os = "macos")]
pub fn get_tty_fd(tty_path: &str) -> Result<c_int, String> {
    let cpath = CString::from_str(tty_path).expect("Failed to init cpath");
    let tty_fd: c_int = unsafe {
        open(cpath.as_ptr(), O_EVTONLY | O_NONBLOCK) // events only, nonblocking
    };
    if tty_fd < 0 {
        Err(format!("Failed to open {tty_path}"))
    } else {
        Ok(tty_fd)
    }
}

#[rustfmt::skip]
pub fn get_term_dims(tty_fd: c_int) -> Result<winsize, &'static str> {
    let mut dims = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let result: c_int = unsafe {
        ioctl(tty_fd, TIOCGWINSZ, &mut dims as *mut winsize)
    };
    if result < 0 {
        Err("Failed to get terminal dimensions")
    } else {
        Ok(dims)
    }
}

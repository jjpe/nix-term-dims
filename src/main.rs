use clap::Parser;
#[cfg(target_os = "macos")]
use libc::O_EVTONLY;
use libc::{c_int, ioctl, open, winsize, O_NONBLOCK, TIOCGWINSZ};
use std::{ffi::CString, str::FromStr};

fn main() -> Result<(), String> {
    let cli_args = CliArgs::parse();
    let tty_fd: c_int = get_tty_fd(&cli_args.tty_filepath)?;
    let dims = get_term_dims(tty_fd)?;
    report_term_dims(&cli_args, &dims);
    Ok(())
}

fn report_term_dims(cli_args: &CliArgs, dims: &winsize) {
    if cli_args.columns {
        print!("{}", dims.ws_col);
        if cli_args.rows {
            print!(" {}", dims.ws_row);
        }
    } else if cli_args.rows {
        print!("{}", dims.ws_row);
    }
    println!();
}

#[cfg(target_os = "linux")]
fn get_tty_fd(tty_path: &str) -> Result<c_int, String> {
    let cpath = CString::from_str(tty_path).expect("Failed to init cpath");
    let tty_fd: c_int = unsafe { open(cpath.as_ptr(), O_NONBLOCK) };
    if tty_fd < 0 {
        Err(format!("Failed to open {tty_path}"))
    } else {
        Ok(tty_fd)
    }
}

#[cfg(target_os = "macos")]
fn get_tty_fd(tty_path: &str) -> Result<c_int, String> {
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
fn get_term_dims(tty_fd: c_int) -> Result<winsize, &'static str> {
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

#[derive(clap::Parser)]
struct CliArgs {
    /// Report the terminal width, in columns
    #[arg(short, long, default_value_t = false)]
    columns: bool,

    /// Report the terminal height, in rows
    #[arg(short, long, default_value_t = false)]
    rows: bool,

    /// The tty filepath to use for querying the terminal dimensions
    #[arg(short, long, default_value_t = {"/dev/tty".to_string()})]
    tty_filepath: String,
}

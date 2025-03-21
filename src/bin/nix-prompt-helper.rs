use clap::Parser;
use libc::{c_int, winsize};
use nix_term_dims::{get_term_dims, get_tty_fd};

fn main() -> Result<(), String> {
    let cli_args = CliArgs::parse();
    let tty_fd: c_int = get_tty_fd(&cli_args.tty_filepath)?;
    let winsize { ws_col, .. } = get_term_dims(tty_fd)?;
    for _ in 0..ws_col {
        print!("{}", cli_args.string);
    }
    println!();
    Ok(())
}

#[derive(clap::Parser)]
struct CliArgs {
    /// The string to use between terminal commands
    #[arg(short, long, default_value_t = { "=".to_string() })]
    string: String,

    /// The tty filepath to use for querying the terminal dimensions
    #[arg(short, long, default_value_t = { "/dev/tty".to_string() })]
    tty_filepath: String,
}

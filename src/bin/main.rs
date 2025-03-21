use clap::Parser;
use libc::{c_int, winsize};
use nix_terminal_dimensions::{get_term_dims, get_tty_fd};

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

use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Print a friendly greeting.
    Greet {
        /// Person to greet.
        #[arg(short, long, default_value = "world")]
        name: String,
    },
}

/// Execute the CLI entry point.
///
/// # Errors
///
/// Returns an error when writing to standard output fails.
pub fn run(cli: Cli) -> color_eyre::Result<()> {
    match cli.command {
        Some(Command::Greet { name }) => {
            println!("Hello, {name}!");
        }
        None => {
            let mut command = Cli::command();
            command.print_help()?;
            println!();
        }
    }

    Ok(())
}

use clap::Parser;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = {{ cookiecutter.crate_name }}::Cli::parse();
    {{ cookiecutter.crate_name }}::run(cli)
}

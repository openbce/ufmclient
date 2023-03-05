use clap::{Parser, Subcommand};

use ufmclient::{UFMError, UFM};

mod view;

#[derive(Parser)]
#[command(name = "ufm")]
#[command(author = "Klaus Ma <klaus@xflops.cn>")]
#[command(version = "0.1.0")]
#[command(about = "UFM command line", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    View {
        #[arg(short, long)]
        pkey: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), UFMError> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::View { pkey }) => view::view_run(pkey).await?,
        None => {}
    };

    Ok(())
}

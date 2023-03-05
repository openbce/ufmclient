use clap::{Parser, Subcommand};

use ufmclient::util::parse_pkey;
use ufmclient::{Partition, PartitionQoS, PortBinding, UFMError, UFM};

mod create;
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
    Create {
        #[arg(short, long)]
        pkey: String,
        #[arg(long)]
        mtu: i32,
        #[arg(long)]
        ipoib: bool,
        #[arg(long)]
        index0: bool,
        #[arg(short, long)]
        membership: String,
        #[arg(short, long)]
        service_level: i32,
        #[arg(short, long)]
        rate_limit: f64,
        #[arg(short, long)]
        guids: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), UFMError> {
    env_logger::init();

    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::View { pkey }) => view::run(pkey).await?,
        Some(Commands::Create {
            pkey,
            mtu,
            ipoib,
            index0,
            membership,
            service_level,
            rate_limit,
            guids,
        }) => {
            create::run(
                pkey,
                mtu,
                ipoib,
                index0,
                membership,
                service_level,
                rate_limit,
                guids,
            )
            .await?
        }
        None => {}
    };

    Ok(())
}

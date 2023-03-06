use clap::{Parser, Subcommand};

use ufmclient::UFMError;

mod create;
mod list;
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
    List,
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
        Some(Commands::List) => list::run().await?,
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
            let opt = create::CreateOptions {
                pkey: pkey.to_string(),
                mtu: *mtu,
                ipoib: *ipoib,
                index0: *index0,
                membership: membership.to_string(),
                service_level: *service_level,
                rate_limit: *rate_limit,
                guids: guids.to_vec(),
            };
            create::run(&opt).await?
        }
        None => {}
    };

    Ok(())
}

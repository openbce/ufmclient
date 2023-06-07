use clap::{Parser, Subcommand};

use ufmclient::{UFMError, UFMConfig};
use std::env;

mod create;
mod delete;
mod list;
mod version;
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
    Version,
    Delete {
        #[arg(short, long)]
        pkey: String,
    },
    Create {
        #[arg(short, long)]
        pkey: String,
        #[arg(long)]
        mtu: u16,
        #[arg(long)]
        ipoib: bool,
        #[arg(long)]
        index0: bool,
        #[arg(short, long)]
        membership: String,
        #[arg(short, long)]
        service_level: u8,
        #[arg(short, long)]
        rate_limit: f64,
        #[arg(short, long)]
        guids: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), UFMError> {
    env_logger::init();

    let conf = load_conf();
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Delete { pkey }) => delete::run(conf, pkey).await?,
        Some(Commands::Version) => version::run(conf).await?,
        Some(Commands::List) => list::run(conf).await?,
        Some(Commands::View { pkey }) => view::run(conf, pkey).await?,
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
            create::run(conf, &opt).await?
        }
        None => {}
    };

    Ok(())
}


fn load_conf() -> UFMConfig {
    let ufm_address = match env::var("UFM_ADDRESS").ok() {
        Some(address) => address,
        None => panic!("UFM_ADDRESS not found"),
    };

    let ufm_username = env::var("UFM_USERNAME").ok();
    let ufm_passworkd = env::var("UFM_PASSWORD").ok();
    let ufm_token = env::var("UFM_TOKEN").ok();

    UFMConfig {
        address: ufm_address,
        username: ufm_username,
        password: ufm_passworkd,
        token: ufm_token,
    }
}

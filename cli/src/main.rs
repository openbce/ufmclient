use clap::{Parser, Subcommand};

use std::env;
use ufmclient::{UFMConfig, UFMError};

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
    /// View the detail of the partition
    View {
        /// The pkey of the partition to view
        #[arg(short, long)]
        pkey: String,
    },
    /// List all partitions
    List,
    /// Get the version of UFM
    Version,
    /// Delete the partition
    Delete {
        /// The pkey of the partition to delete
        #[arg(short, long)]
        pkey: String,
    },
    /// Create a partition
    Create {
        /// The pkey for the new partition
        #[arg(short, long)]
        pkey: String,
        /// The MTU of the new partition
        #[arg(long, default_value_t = 2048)]
        mtu: u16,
        /// The IPOverIB of the new partition
        #[arg(long, default_value_t = true)]
        ipoib: bool,
        /// The Index0 of the new partition
        #[arg(long, default_value_t = true)]
        index0: bool,
        /// The Membership of the new partition
        #[arg(short, long, default_value_t = String::from("full"))]
        membership: String,
        /// The ServiceLevel of the new partition
        #[arg(short, long, default_value_t = 0)]
        service_level: u8,
        /// The RateLimit of the new partition
        #[arg(short, long, default_value_t = 100.0)]
        rate_limit: f64,
        /// The GUIDs of the new partition
        #[arg(short, long)]
        guids: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), UFMError> {
    env_logger::init();

    let cli = Cli::parse();

    let conf = load_conf();
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

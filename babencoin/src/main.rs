#![forbid(unsafe_code)]

use babencoin::node::{run, Config};

use anyhow::{Context, Result};
use log::*;
use structopt::StructOpt;

use std::{fs::File, io::Read};

const DEFAULT_LOG_VERBOSITY: usize = 3;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opts {
    /// Config path
    #[structopt(short = "c", long = "config")]
    config_path: String,
}

fn read_config(path: &str) -> Result<Config> {
    let mut file = File::open(path).context(format!("failed to open {}", path))?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .context(format!("failed to read {}", path))?;

    serde_yaml::from_slice(&buffer).context("failed to parse config")
}

fn get_verbosity() -> usize {
    let log_level = match std::env::var("LOG_LEVEL") {
        Ok(s) => s,
        _ => return DEFAULT_LOG_VERBOSITY,
    };
    match log_level.to_lowercase().as_str() {
        "trace" => 4,
        "debug" => 3,
        "info" => 2,
        "error" => 1,
        "none" => 0,
        _ => DEFAULT_LOG_VERBOSITY,
    }
}

async fn do_main() -> Result<()> {
    let opts = Opts::from_args();

    stderrlog::new()
        .verbosity(get_verbosity())
        .timestamp(stderrlog::Timestamp::Millisecond)
        .init()
        .expect("failed to initialize logging");

    let config = read_config(&opts.config_path)?;
    run(config).await
}

#[tokio::main]
async fn main() {
    if let Err(err) = do_main().await {
        error!("{:#}", err);
        std::process::exit(1);
    }
}

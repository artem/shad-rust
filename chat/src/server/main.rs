use structopt::StructOpt;

use chat::server;

#[derive(StructOpt)]
struct Opts {
    #[structopt(long)]
    admin_token: String,

    #[structopt(long)]
    addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();
    server::serve(opts.admin_token, opts.addr.parse()?).await?;
    Ok(())
}

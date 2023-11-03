use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    ip: String,

    #[clap(short, long)]
    user_name: String,

    #[clap(short, long)]
    secret: u64,
}

mod api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let _client = api::register(args.ip, args.user_name, args.secret).await?;
    Ok(())
}

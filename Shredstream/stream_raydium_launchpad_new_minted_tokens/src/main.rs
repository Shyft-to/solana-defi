
mod processor;

use {
    clap::Parser,
    std::{env, io, time::Duration},
    tokio::time::sleep,
    processor::core::connect_and_stream,
};

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, default_value_t = String::from("http://127.0.0.1:9999"))]
    shredstream_uri: String,

    #[clap(short, long)]
    x_token: Option<String>,

    #[clap(long)]
    pubkey: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    env::set_var(
        env_logger::DEFAULT_FILTER_ENV,
        env::var_os(env_logger::DEFAULT_FILTER_ENV).unwrap_or_else(|| "info".into()),
    );
    env_logger::init();

    let args = Args::parse();
    println!("Starting Stream...");
    loop {
        match connect_and_stream(
            &args.shredstream_uri,
            args.x_token.as_deref(),
            args.pubkey.as_deref(),
        )
        .await
        {
            Ok(()) => {
                println!("Stream ended gracefully. Reconnecting...");
            }
            Err(e) => {
                eprintln!("Connection or stream error: {e}. Retrying...");
            }
        }

        sleep(Duration::from_secs(1)).await;
    }
}

use rutrack::opts::Opts;
use structopt::StructOpt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let opts = Opts::from_args();
    let addr = format!("{}:{}", opts.address, opts.port);
    let listner = TcpListener::bind(addr)
        .await
        .expect("listner running error");

    while let Ok((stream, _addr)) = listner.accept().await {}
}

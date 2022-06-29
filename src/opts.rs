#[derive(Debug, structopt::StructOpt)]
pub struct Opts{
    #[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
    pub address: String, 
    #[structopt(short = "p", long = "port", default_value = "6787")]
    pub port: String, 
}

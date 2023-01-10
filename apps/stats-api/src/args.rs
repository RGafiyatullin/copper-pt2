use std::net::SocketAddr;

#[derive(Debug, structopt::StructOpt)]
pub struct ArgsDb {
    #[structopt(
        long,
        env = "DB",
        default_value = "host=127.0.0.1 port=5432 user=dev password=dev dbname=dev"
    )]
    pub db: String,
}

#[derive(Debug, structopt::StructOpt)]
pub struct ArgsHttp {
    #[structopt(long, env = "BIND_ADDR")]
    pub bind_addr: SocketAddr,
}

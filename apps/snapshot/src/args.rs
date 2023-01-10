use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct ArgDuration(pub Duration);

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
pub struct ArgsHNApi {
    #[structopt(long, env = "HN_API_CONCURRENCY", default_value = "1")]
    pub hn_api_concurrency: usize,

    #[structopt(
        long,
        env = "HN_API_BASE_URL",
        default_value = "https://hacker-news.firebaseio.com/"
    )]
    pub hn_api_base_url: String,
}

impl FromStr for ArgDuration {
    type Err = parse_duration::parse::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_duration::parse(s).map(Self)
    }
}

impl From<ArgDuration> for Duration {
    fn from(value: ArgDuration) -> Self {
        value.0
    }
}

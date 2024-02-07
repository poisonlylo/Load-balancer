// config.rs

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Load Balancer", about = "A simple load balancer in Rust")]
pub struct CmdOptions {
    #[structopt(short, long, default_value = "127.0.0.1:8080", help = "Bind address")]
    pub bind: String,

    #[structopt(short, long, help = "Upstream servers (comma-separated list)")]
    pub upstream: Vec<String>,
}

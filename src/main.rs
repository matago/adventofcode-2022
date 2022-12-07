mod cli;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let opt = cli::Opt::from_args();
    println!("{:?}", opt);
}

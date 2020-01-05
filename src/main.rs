#[macro_use]
extern crate clap;

pub mod text;
pub mod obj;

fn main() {
    use structopt::StructOpt;

    let opts = obj::Opts::from_args();
    println!("Hello, world!");
}

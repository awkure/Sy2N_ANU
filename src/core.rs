#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![feature(crate_in_paths)]

extern crate structopt;
use structopt::StructOpt;

extern crate Sy2N_ANU;
use Sy2N_ANU::proto::quantum::get_random_quantum_numbers;

fn main () {
    let config = Sy2N_ANU::Config::from_args();
    println!("{:?}", get_random_quantum_numbers(config));
}
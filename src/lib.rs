#![feature(crate_in_paths)]

extern crate curl;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

#[macro_use] extern crate log;
extern crate loggerv;

extern crate structopt;
#[macro_use] extern crate structopt_derive;

pub mod proto;
use proto::quantum::QuantumNumberType;

#[derive(StructOpt)]
#[structopt(name = "Sy2N_ANU")]
pub struct Config {
    #[structopt(long="length", short="l", default_value="1", help="Output the amount of generated numbers")]
    array_length : usize,

    #[structopt(long="block_size", short="bs", default_value="5", help="Set generator block size")]
    block_size   : usize,
      
    #[structopt(subcommand)]
    data_type    : Option<QuantumNumberType>,

    #[structopt(long="verbose", short="v", help="Switch on verbosity")]
    verbosity    : Option<u8>,
}
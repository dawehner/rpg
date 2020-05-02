extern crate docopt;
extern crate rustc_serialize;
extern crate rand;
extern crate num_bigint;
extern crate num_traits;
extern crate num;

use docopt::Docopt;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use num_bigint::{BigInt};
use num::pow::pow;

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_length: usize,
    flag_type: TargetMode,
    flag_verbose: bool
}

#[derive(Debug)]
enum TargetMode {
    String,
    Number,
}

impl rustc_serialize::Decodable for TargetMode {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<TargetMode, D::Error> {
        let str = d.read_str()?;
        let string = str.as_ref();
        Ok(match string {
            "s" => TargetMode::String,
            "string" => TargetMode::String,
            "n" => TargetMode::Number,
            "number" => TargetMode::Number,
            _ => TargetMode::String,
        })
    }
}

fn calculate_entropy(length : usize, mode : TargetMode) -> BigInt {
    match mode {
        TargetMode::String => pow(BigInt::from(62), length),
        TargetMode::Number => pow(BigInt::from(10), length),
    }
}

fn main() {

    // Write the Docopt usage string.
    static USAGE: &'static str = "
    Usage:
      rpg [--length=<length>] [--type=<type>] [--verbose]
      rpg (-h | --help)

    Options:
        -h --help             Show this screen.
        --length=<length>     Length of the random string [default: 20].
        --verbose             Shows information about the password
        --type=<type>         Target type: s, string for string, n, number for number [default: s]
    ";

    // let argv = std::env::args();

    let args: Args = Docopt::new(USAGE)
                  .and_then(|d| d.decode())
                  .unwrap_or_else(|e| e.exit());

    let ten: i64 = 10;
    let random_string: String = match args.flag_type {
        TargetMode::String => thread_rng().sample_iter(Alphanumeric).take(args.flag_length).collect(),
        // @fixme It would be better to just generate a bunch of 1 char numbers.
        TargetMode::Number => thread_rng().gen_range(0, ten.pow(args.flag_length as u32)).to_string(),
    };

    if args.flag_verbose {
        println!("Length: {}", args.flag_length);
        println!("Entropy: {}", calculate_entropy(args.flag_length, args.flag_type));
        println!("");
    }

    println!("{}", random_string);
}


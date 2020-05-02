extern crate docopt;
extern crate rustc_serialize;
extern crate rand;

use docopt::Docopt;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_length: usize,
    flag_type: TargetMode,
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
    

fn main() {

    // Write the Docopt usage string.
    static USAGE: &'static str = "
    Usage:
      rpg [--length=<length>] [--type=<type>]
      rpg (-h | --help)

    Options:
        -h --help             Show this screen.
        --length=<length>     Length of the random string [default: 20].
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

    println!("{}", random_string);
}


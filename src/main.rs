extern crate docopt;
extern crate rustc_serialize;
extern crate rand;

use docopt::Docopt;
use rand::{thread_rng, Rng};

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_length: usize,
}

fn main() {

    // Write the Docopt usage string.
    static USAGE: &'static str = "
    Usage:
      rpg [--length=<length>]
      rpg (-h | --help)

    Options:
        -h --help     Show this screen.
        --length=<length>     Length of the random string [default: 20].
    ";

    // let argv = std::env::args();

    let args: Args = Docopt::new(USAGE)
                  .and_then(|d| d.decode())
                  .unwrap_or_else(|e| e.exit());

    let random_string: String = thread_rng().gen_ascii_chars().take(args.flag_length).collect();
    println!("{}", random_string);
}


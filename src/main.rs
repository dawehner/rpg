extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_length: i32,
}

fn main() {

    // Write the Docopt usage string.
    static USAGE: &'static str = "
    Usage: rpg [--length=<length>]

    Options:
        --length=<length>     Length of the random string.
    ";

    // let argv = std::env::args();

    let args: Args = Docopt::new(USAGE)
                  .and_then(|d| d.decode())
                  .unwrap_or_else(|e| e.exit());

    println!("{}", args.flag_length);
}

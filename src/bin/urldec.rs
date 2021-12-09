use std::io;
use std::io::Read;
use structopt::StructOpt;
use urlencoding::decode;

/// url-decode a string
#[derive(StructOpt, Debug)]
#[structopt(name = "urldec")]
struct Opt {
    /// Text to url-decode
    #[structopt(name = "TEXT")]
    text: String,
}

fn main() {
    if atty::isnt(atty::Stream::Stdin) {
        let mut buffer = String::new();
        let _read = io::stdin().read_to_string(&mut buffer).unwrap();
        println!("{}", decode(buffer.trim()).unwrap());
    } else {
        let opt = Opt::from_args();
        println!("{}", decode(&opt.text).unwrap());
    }
}

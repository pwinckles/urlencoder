use std::io;
use std::io::Read;
use structopt::StructOpt;
use urlencoding::encode;

/// url-encode a string
#[derive(StructOpt, Debug)]
#[structopt(name = "urlenc")]
struct Opt {
    /// Text to url-encode
    #[structopt(name = "TEXT")]
    text: String,
}

fn main() {
    if atty::isnt(atty::Stream::Stdin) {
        let mut buffer = String::new();
        let _read = io::stdin().read_to_string(&mut buffer).unwrap();
        println!("{}", encode(buffer.trim()));
    } else {
        let opt = Opt::from_args();
        println!("{}", encode(&opt.text));
    }
}

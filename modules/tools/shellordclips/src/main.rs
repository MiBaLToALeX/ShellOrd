//! ShellOrdClips
//! Licence: Apache-2.0

extern crate base64;
extern crate clipboard;
extern crate flate2;

use base64::{encode};
use std::{env, io};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use clipboard::{ClipboardProvider, ClipboardContext};
use flate2::bufread::GzEncoder;
use flate2::Compression;
use std::io::BufReader;

static OOPS: &str = "¯\\_(ツ)_/¯";

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("{}", OOPS),
        2 => {
            let filename = &args[1];
            println!("{}", filename);
            let secreto = read_file(filename);
            if secreto.is_ok() {
                let b64 = encode(secreto.unwrap());
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(b64.to_owned()).unwrap();
            }
        },
        _ => {
            println!("{}", OOPS)
        }
    }
}

fn read_file(file_name: &String) -> io::Result<Vec<u8>> {
    let path = Path::new(&file_name);
    if !path.exists() {
        return Ok(String::from(OOPS).into());
    }
    let file = File::open(&file_name).expect(OOPS);
    let b = BufReader::new(file);
    let mut gz = GzEncoder::new(b, Compression::fast());
    let mut buffer = Vec::new();
    gz.read_to_end(&mut buffer)?;
    Ok(buffer)
}

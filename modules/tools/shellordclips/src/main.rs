//! ShellOrdClips
//! Licence: Apache-2.0

extern crate base64;
extern crate clipboard;

use base64::{encode};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use clipboard::{ClipboardProvider, ClipboardContext};

static OOPS: &str = "¯\\_(ツ)_/¯";

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("{}", OOPS),
        2 => {
            let filename = &args[1];
            println!("{}", filename);
            let secreto = read_file(filename);
            let b64 = encode(secreto);
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(b64.to_owned()).unwrap();
        },
        _ => {
            println!("{}", OOPS)
        }
    }
}

fn read_file(file_name: &String) -> Vec<u8> {
    let path = Path::new(&file_name);
    if !path.exists() {
        return String::from(OOPS).into();
    }
    let mut file_content = Vec::new();
    let mut file = File::open(&file_name).expect(OOPS);
    file.read_to_end(&mut file_content).expect(OOPS);
    file_content
}

extern crate argh;
extern crate core;

use std::io::{Read, stdin, stdout, Write};
use std::io::Result as IoResult;
use std::process;
use argh::FromArgs;

#[derive(FromArgs)]
/// base2 to binary conversion program
struct Base2 {
    /// decode mode
    #[argh(switch, short = 'd')]
    decode: bool,
}

fn main() -> IoResult<()> {
    let args: Base2 = argh::from_env();

    if args.decode {
        decode()?;
    } else {
        encode()?;
    }

    Ok(())
}

fn decode() -> IoResult<()> {
    let mut bytes = stdin().bytes();

    let mut ord: u8 = 7;
    let mut buf: u8 = 0;

    while let Some(Ok(byte)) = bytes.next() {
        match byte {
            b'1' => {
                buf |= 1 << ord;
            }
            b'0' => (),
            _ => {
                eprintln!("Invalid input detected.");
                process::exit(1);
            }
        }
        if ord == 0 {
            stdout().write_all(&[buf])?;
            ord = 7;
            buf = 0;
        } else {
            ord -= 1;
        }
    }

    stdout().write_all(&[buf])?;
    eprintln!("{} trailing `0` bits added.", ord + 1);

    Ok(())
}

fn encode() -> IoResult<()> {
    let mut bytes = stdin().bytes();

    while let Some(Ok(byte)) = bytes.next() {
        let mut bytes = [b'0'; 8];
        for i in 0..8 {
            if (byte >> i) % 2 == 1 {
                bytes[7 - i] = b'1';
            }
        }
        stdout().write_all(&bytes)?;
    }

    Ok(())
}

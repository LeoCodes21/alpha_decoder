extern crate decoder;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::env;
use std::fs::File;
use std::io::Write;

use clap::Clap;
use pretty_hex::*;

#[derive(Clap)]
#[clap(name = "alpha_decoder", version = "1.0", author = "Leo S. <coderleo42@gmail.com>")]
struct Opts {
    #[clap(about = "Alphanumeric shellcode encoded in one of the Alpha2 formats", required = true, index = 1)]
    code: String,

    #[clap(long, short, about = "Path to output file for decoded shellcode")]
    output_path: Option<String>,

    #[clap(long, short, about = "Print a hex dump of the decoded result")]
    hex_dump: bool,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();
    println!(r#"        _       _                _                    _
       | |     | |              | |                  | |
   __ _| |_ __ | |__   __ _   __| | ___  ___ ___   __| | ___ _ __
  / _` | | '_ \| '_ \ / _` | / _` |/ _ \/ __/ _ \ / _` |/ _ \ '__|
 | (_| | | |_) | | | | (_| || (_| |  __/ (_| (_) | (_| |  __/ |
  \__,_|_| .__/|_| |_|\__,_| \__,_|\___|\___\___/ \__,_|\___|_|
         | |             ______
         |_|            |______|                                  "#);

    let opts: Opts = Opts::parse();
    let available_formats: Vec<Box<dyn decoder::formats::ShellcodeFormat>> = vec![
        Box::new(decoder::formats::unicode::mixed_case::MixedCaseUnicodeFormat {}),
        Box::new(decoder::formats::unicode::uppercase::UppercaseUnicodeFormat {}),
        Box::new(decoder::formats::unicode::mixed_case_nocompress::MixedCaseNoCompressUnicodeFormat {}),
        Box::new(decoder::formats::unicode::uppercase_nocompress::UppercaseNoCompressUnicodeFormat {}),
        Box::new(decoder::formats::ascii::mixed_case::MixedCaseAsciiFormat {}),
        Box::new(decoder::formats::ascii::uppercase::UppercaseAsciiFormat {}),
        Box::new(decoder::formats::ascii::mixed_case_nocompress::MixedCaseNoCompressAsciiFormat {}),
        Box::new(decoder::formats::ascii::uppercase_nocompress::UppercaseNoCompressAsciiFormat {}),
    ];

    let mut decoded_shellcode: Option<Vec<u8>> = None;
    let mut used_format: Option<Box<dyn decoder::formats::ShellcodeFormat>> = None;

    for format in available_formats {
        let decoded = format.decode(opts.code.as_str());

        if decoded.is_none() {
            continue;
        }

        decoded_shellcode.replace(decoded.unwrap());
        used_format.replace(format);
        break;
    }

    if decoded_shellcode.is_none() {
        error!("Shellcode could not be decoded.");
    } else {
        let unwrapped_code = decoded_shellcode.unwrap();
        info!("Decoded shellcode as {}: {}", used_format.unwrap().get_name(), unwrapped_code.hex_conf(HexConfig { ascii: false, chunk: 0, group: 0, title: false, width: 0 }));

        if opts.hex_dump {
            println!("{:?}", unwrapped_code.hex_dump());
        }

        match opts.output_path {
            None => {}
            Some(s) => {
                let mut file = File::create(s)?;

                match file.write_all(unwrapped_code.as_slice()) {
                    Ok(_) => {}
                    Err(e) => error!("Could not write file: {}", e.to_string())
                }
            }
        }
    }

    Ok(())
}

use anyhow::Result;

use clap::Parser;
use file_fmt::{*, cli::Cli};


fn main() -> Result<()> {
    let args = Cli::parse();

    let format = args.format.unwrap_or_else(|| get_file_format(args.file(), args.format).unwrap());
    let data = if args.streaming {
        println!("Reading {:?} via stream", args.file());
        deserialize_streaming(args.file(), format)
    }
    else {
        println!("Reading {:?}", args.file());
        deserialize_default(args.file(), format)
    }?;

    let pretty = args.mode.pretty();

    if args.streaming {
        println!("Writing {:?} via stream", args.file());
        serialize_streaming(args.file(), data, pretty)?;
    }
    else {
        println!("Writing {:?}", args.file());
        serialize_default(args.file(), data, pretty)?;
    }

    Ok(())
}

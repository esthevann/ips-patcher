mod record;
mod ips;
mod rom;

use crate::{ips::Ips, rom::Rom};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = miette::Result<T, Error>;

struct Args {
    ips_name: std::path::PathBuf,
    rom_name: std::path::PathBuf,
    new_name: Option<std::path::PathBuf>
}

fn main() -> Result<()> {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            println!("usage: <ips_file> <file_to_patch> -o <output_name>");
            Err(e)?
        }
    };
    

    let ips = Ips::load_file(&args.ips_name)?;
    let rom = Rom::load_file(&args.rom_name)?;

    
    let rom = ips.apply_patch(rom);
    if let Some(name) = args.new_name {
        rom.write_file(name)?;
    } else {
        rom.write_file(args.rom_name)?;
    }

    Ok(())
}



fn parse_args() -> Result<Args> {
    let mut pargs = pico_args::Arguments::from_env();

    let args = Args {
        ips_name: pargs.free_from_str()?,
        rom_name: pargs.free_from_str()?,
        new_name: pargs.opt_value_from_str("-o")?
    };

    Ok(args)
}
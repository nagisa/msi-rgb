extern crate clap;
#[macro_use]
extern crate error_chain;

use std::io::{self, Seek, Write};
use std::fs;
use clap::{Arg, App, ArgMatches};

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Parse(::std::num::ParseIntError);
    }
}

const PORT1: u8 = 0x4e;
const PORT2: u8 = 0x4f;

const REDCELL: u8 = 0xf0;
const GREENCELL: u8 = 0xf4;
const BLUECELL: u8 = 0xf8;

fn outb(f: &mut fs::File, cell: u8, data: u8) -> io::Result<()> {
    f.seek(io::SeekFrom::Start(cell as _))?;
    f.write(&[data])?;
    Ok(())
}

fn write_byte_to_cell(f: &mut fs::File, cell: u8, data: u8) -> io::Result<()> {
    outb(f, PORT1, 0x87)?;
    outb(f, PORT1, 0x07)?;
    outb(f, PORT2, 0x12)?;
    outb(f, PORT1, cell)?;
    outb(f, PORT2, data)
}

fn write_colour(f: &mut fs::File, cell_offset: u8, data: u32) -> io::Result<()> {
    write_byte_to_cell(f, cell_offset, (data >> 24) as u8)?;
    write_byte_to_cell(f, cell_offset + 1, (data >> 16) as u8)?;
    write_byte_to_cell(f, cell_offset + 2, (data >> 8) as u8)?;
    write_byte_to_cell(f, cell_offset + 3, data as u8)
}

fn run<'a>(matches: ArgMatches<'a>) -> Result<()> {
    let mut f = fs::OpenOptions::new().read(true).write(true).open("/dev/port")
        .chain_err(|| { "could not open /dev/port; try sudo?" })?;
    let disable = matches.is_present("DISABLE");
    let pulse = matches.is_present("PULSE");
    let flash = matches.value_of("BLINK").expect("bug: BLINK argument").parse::<u8>()?;
    let red = u32::from_str_radix(matches.value_of("RED").expect("bug: RED argument"), 16)?;
    let green = u32::from_str_radix(matches.value_of("GREEN").expect("bug: GREEN argument"), 16)?;
    let blue = u32::from_str_radix(matches.value_of("BLUE").expect("bug: BLUE argument"), 16)?;
    let step_duration = matches.value_of("STEPDURATION").expect("bug: STEPDURATION argument")
                               .parse::<u16>()?;
    let invs = matches.values_of("INVHALF").map(|i| i.collect()).unwrap_or(Vec::new());

    let e4_val = if disable { 1 } else { 0 } |
                 if pulse { 0b1000 } else { 0 } |
                 if flash == 0 { 0 } else { (flash + 1) & 0b111 };
    write_byte_to_cell(&mut f, 0xe4, e4_val)?;
    write_colour(&mut f, REDCELL, red)?;
    write_colour(&mut f, GREENCELL, green)?;
    write_colour(&mut f, BLUECELL, blue)?;

    write_byte_to_cell(&mut f, 0xfe, step_duration as u8)?;
    let ff_val = (step_duration >> 8) as u8 & 1 |
                 0b10 | // if 0 disable lights on rgb header only, not on board
                 if invs.contains(&"b") { 0b10000 } else { 0 } |
                 if invs.contains(&"g") { 0b01000 } else { 0 } |
                 if invs.contains(&"r") { 0b00100 } else { 0 };
    write_byte_to_cell(&mut f, 0xff, ff_val)?;
    Ok(())
}

fn main() {
    let matches = App::new("msi-rgb")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0"))
        .about(option_env!("CARGO_PKG_DESCRIPTION").unwrap_or(""))
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("RED").required(true)
             .help("values of red colour (32 bit hex number, up to FFFFFFFF)"))
        .arg(Arg::with_name("GREEN").required(true)
             .help("values of green colour (32 bit hex number, up to FFFFFFFF)"))
        .arg(Arg::with_name("BLUE").required(true)
             .help("values of blue colour (32 bit hex number, up to FFFFFFFF)"))
        .arg(Arg::with_name("INVHALF").long("invert").short("i").multiple(true)
             .takes_value(true).possible_values(&["r","g","b"])
             .help("invert the specified channel(s) and halve their bit depth (=8 distinct steps)"))
        .arg(Arg::with_name("PULSE").long("pulse").short("p")
             .help("smooth pulsing"))
        .arg(Arg::with_name("STEPDURATION").long("duration").short("d").takes_value(true)
             .default_value("25").validator(|s| { match s.parse::<u16>() {
                 Err(e) => Err(format!("{}", e)),
                 Ok(s) => if s > 511 { Err("duration must not exceed 511".into()) } else { Ok(()) }
             }})
             .help("duration between distinct steps of colours (0 - fastest, 511 - slowest"))
        .arg(Arg::with_name("BLINK").long("blink").short("b").takes_value(true).default_value("0")
             .possible_values(&["0", "1", "2", "3", "4", "5", "6"])
             .help("duration between blinks (from 0 to 6, 0 is always on, 6 is slowest)"))
        .arg(Arg::with_name("DISABLE").long("disable").short("x")
             .help("disable the RGB subsystem altogether"))
        .get_matches();

    ::std::process::exit(if let Err(e) = run(matches) {
        let _ = writeln!(::std::io::stderr(), " {}", e);
        for e in e.iter().skip(1) {
            let _ = writeln!(::std::io::stderr(), "    caused by: {}", e);
        }
        1
    } else {
        0
    });
}

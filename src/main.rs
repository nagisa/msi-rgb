//! Utility for controlling RGB header on MSI boards
//!
//! The RGB header is controlled by the NCT6795D Super I/O chip.
//!
//! The Advanced "mode" is enabled by writing 87 87 to PORT 1. It is later disabled by writing AA
//! to that same port. `07 12` then selects the bank 12h. This bank then looks like this:
//!
//! 00 |  ...
//! 10 |  ...
//! .  |
//! .  |
//! .  |
//! E0 | EE XX XX XX  PX XX XX XX  XX XX XX XX  XX XX XX XX
//! F0 | RR RR RR RR  GG GG GG GG  BB BB BB BB  XX XX TT TX
//!     --------------------------------------------------
//!      00 01 02 03  04 05 06 07  08 09 0A 0B  0C 0D 0E 0F
//!
//! Here:
//!
//! Purpose of following bits in `EE` is known:
//!
//! `0b10000000` - red channel can handle 16 levels
//! `0b01000000` - green channel can handle 16 levels
//! `0b00100000` - blue channel can handle 16 levels
//!
//! If the corresponding bit is `0`, the channel always receives the maximum "brightness",
//! regardless of setting.
//!
//!
//! `R` - intensity of the red colour
//! `G` - intensity of the green colour
//! `B` - intensity of the blue colour
//!
//! There’s 8 distinct steps that can be specified, hence the four instances of `RR`, `GG` and
//! `BB`. These colours change every given interval specified in the `TTT` bytes. TTT has the bit
//! format like this: `bgrdt tttt tttt`
//!
//! Here `t` bits are a duration between changes from one colour to another (takes the next column
//! of RR GG BB);
//!
//! `d` bit specifies whether the RGB header is turned on (distinct from the motherboard lights).;
//!
//! `bgr` invert the intensity (`F` is 0%, `0` is 100%) for blue, green and red channels
//! respectively.
//!
//! `P` here is another bitmask of the form `pbbb`, where `p` specifies whether smooth pulsing
//! behaviour is enabled. `bbb` specifies duration between blinks. If `bbb` is `001`,
//! all lightning is turned off, including the one on the motherboard itself. `000` is always on.

extern crate clap;
#[macro_use]
extern crate error_chain;

use std::io::{self, Seek, Write, Read};
use std::fs;
use clap::{Arg, App, ArgMatches};

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Parse(::std::num::ParseIntError);
    }
}

const RGB_BANK: u8 = 0x12;
const NCT6795D_MASK: u16 = 0xD350;


const REG_DEVID_MSB: u8 = 0x20;
const REG_DEVID_LSB: u8 = 0x21;
const REDCELL: u8 = 0xf0;
const GREENCELL: u8 = 0xf4;
const BLUECELL: u8 = 0xf8;

fn inb(f: &mut fs::File, port: u16) -> io::Result<u8> {
    let mut d = [0u8];
    f.seek(io::SeekFrom::Start(port.into()))?;
    f.read(&mut d)?;
    Ok(d[0])
}

fn outb(f: &mut fs::File, port: u16, data: u8) -> io::Result<()> {
    f.seek(io::SeekFrom::Start(port.into()))?;
    f.write(&[data])?;
    Ok(())
}

fn write_byte_to_cell(f: &mut fs::File, base_port: u16, cell: u8, data: u8) -> io::Result<()> {
    outb(f, base_port, cell)?;
    outb(f, base_port + 1, data)
}

fn write_colour(f: &mut fs::File, base_port: u16, cell_offset: u8, data: u32) -> io::Result<()> {
    write_byte_to_cell(f, base_port, cell_offset, (data >> 24) as u8)?;
    write_byte_to_cell(f, base_port, cell_offset + 1, (data >> 16) as u8)?;
    write_byte_to_cell(f, base_port, cell_offset + 2, (data >> 8) as u8)?;
    write_byte_to_cell(f, base_port, cell_offset + 3, data as u8)
}

fn run<'a>(f: &mut fs::File, base_port: u16, matches: ArgMatches<'a>) -> Result<()> {
    let disable = matches.is_present("DISABLE");
    let pulse = matches.is_present("PULSE");
    let ignore = matches.is_present("IGNORECHECK");
    let flash = matches.value_of("BLINK").expect("bug: BLINK argument").parse::<u8>()?;
    let red = u32::from_str_radix(matches.value_of("RED").expect("bug: RED argument"), 16)?;
    let green = u32::from_str_radix(matches.value_of("GREEN").expect("bug: GREEN argument"), 16)?;
    let blue = u32::from_str_radix(matches.value_of("BLUE").expect("bug: BLUE argument"), 16)?;
    let step_duration = matches.value_of("STEPDURATION").expect("bug: STEPDURATION argument")
                               .parse::<u16>()?;
    let invs = matches.values_of("INVHALF").map(|i| i.collect()).unwrap_or(Vec::new());

    // Check if indeed a NCT6795D
    if !ignore {
        outb(f, base_port, REG_DEVID_MSB)?;
        let msb = inb(f, base_port + 1)?;
        outb(f, base_port, REG_DEVID_LSB)?;
        let ident = (msb as u16) << 8 | inb(f, base_port + 1)? as u16;
        if ident & 0xFFF0 != NCT6795D_MASK {
            let err: Result<()> = Err("`--ignore-check` flag, which would skip the check, \
                                       is not specified (may be dangerous); \
                                       also try `--base-port`".into());
            return err.chain_err(|| format!("The sI/O chip identifies as {:x}, which does not \
                                            seem to be NCT6795D", ident));
        }
    }

    // Without this pulsing does not work
    outb(f, base_port, 0x07)?;
    outb(f, base_port + 1, 0x09)?;
    outb(f, base_port, 0x2c)?;
    let c = inb(f, base_port + 1)?;
    if c & 0x10 != 0x10 {
        outb(f, base_port + 1, c | 0x10)?;
    }

    // Select the 0x12th bank.
    outb(f, base_port, 0x07)?;
    outb(f, base_port + 1, RGB_BANK)?;

    // Check if RGB control enabled?
    outb(f, base_port, 0xe0)?;
    let d = inb(f, base_port + 1)?;
    if d & 0xe0 != 0xe0 {
        outb(f, base_port + 1, 0xe0 | (d & !0xe0))?;
    }

    let e4_val = if disable { 1 } else { 0 } |
                 if pulse { 0b1000 } else { 0 } |
                 if flash == 0 { 0 } else { (flash + 1) & 0b111 };
    write_byte_to_cell(f, base_port, 0xe4, e4_val)?;

    write_byte_to_cell(f, base_port, 0xfe, step_duration as u8)?;
    let ff_val = (step_duration >> 8) as u8 & 1 |
                 0b10 | // if 0 disable lights on rgb header only, not on board
                 if invs.contains(&"b") { 0b10000 } else { 0 } |
                 if invs.contains(&"g") { 0b01000 } else { 0 } |
                 if invs.contains(&"r") { 0b00100 } else { 0 };
    write_byte_to_cell(f, base_port, 0xff, ff_val)?;

    write_colour(f, base_port, REDCELL, red)?;
    write_colour(f, base_port, GREENCELL, green)?;
    write_colour(f, base_port, BLUECELL, blue)?;

    Ok(())
}

/// Wrapper which enables and disables the advanced mode
fn run_wrap<'a>(matches: ArgMatches<'a>) -> Result<()> {
    let base_port = u16::from_str_radix(matches.value_of("BASEPORT")
                                               .expect("bug: BASEPORT argument"), 16)?;

    let mut f = fs::OpenOptions::new().read(true).write(true).open("/dev/port")
        .chain_err(|| { "could not open /dev/port; try sudo?" })?;
    // Enable the advanced mode.
    outb(&mut f, base_port, 0x87).chain_err(|| "could not enable advanced mode")?;
    outb(&mut f, base_port, 0x87).chain_err(|| "could not enable advanced mode")?;

    // These are something the built-in app does during initialization…
    // Purpose unclear
    // outb(&mut f, base_port, 0x07)?;
    // outb(&mut f, base_port + 1, 0x0B)?;
    // outb(&mut f, base_port, 0x60)?;
    // let a = inb(&mut f, base_port + 1)?;
    // outb(&mut f, base_port, 0x61)?;
    // let b = inb(&mut f, base_port + 1)?;
    // println!("{:x} {:x}", a, b);

    let r = run(&mut f, base_port, matches);
    // Disable the advanced mode.
    outb(&mut f, base_port, 0xAA).chain_err(|| "could not disable advanced mode")?;
    r.chain_err(|| "could not set the colour")
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
             .help("invert the specified channel(s)"))
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
        .arg(Arg::with_name("IGNORECHECK").long("ignore-check")
             .help("ignore the result of sI/O identification check"))
        .arg(Arg::with_name("BASEPORT").long("base-port").default_value("4e")
             .help("base port to use. Values known to be in use are 4e and 2e"))
        .get_matches();

    ::std::process::exit(if let Err(e) = run_wrap(matches) {
        let _ = writeln!(::std::io::stderr(), "error: {}", e);
        for e in e.iter().skip(1) {
            let _ = writeln!(::std::io::stderr(), "    caused by: {}", e);
        }
        1
    } else {
        0
    });
}

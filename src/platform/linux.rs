use std::fs;
use std::io::{self, Seek, Write, Read};
use ResultExt;

pub fn open_device() -> ::Result<fs::File> {
    fs::OpenOptions::new().read(true).write(true).open("/dev/port")
        .chain_err(|| { "could not open /dev/port; try sudo?" })
}

pub fn inb(f: &mut fs::File, port: u16) -> ::Result<u8> {
    let mut d = [0u8];
    f.seek(io::SeekFrom::Start(port.into()))?;
    f.read(&mut d)?;
    Ok(d[0])
}

pub fn outb(f: &mut fs::File, port: u16, data: u8) -> ::Result<()> {
    f.seek(io::SeekFrom::Start(port.into()))?;
    f.write(&[data])?;
    Ok(())
}

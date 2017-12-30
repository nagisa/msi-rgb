use std::fs;
use std::os::unix::io::AsRawFd;
use ResultExt;

#[repr(C)]
pub struct IodevPioReq {
    access: u32,
    port: u32,
    width: u32,
    val: u32,
}

ioctl!(readwrite iodev_pio with b'I', 0; IodevPioReq);

pub fn open_device() -> ::Result<fs::File> {
    fs::OpenOptions::new().read(true).write(true).open("/dev/io")
        .chain_err(|| { "could not open /dev/io; try sudo?" })
}

pub fn inb(f: &mut fs::File, port: u16) -> ::Result<u8> {
    let mut req = IodevPioReq {
        access: 0,
        port: port as u32,
        width: 1,
        val: 0,
    };
    unsafe { iodev_pio(f.as_raw_fd(), &mut req) }?;
    Ok(req.val as u8)
}

pub fn outb(f: &mut fs::File, port: u16, data: u8) -> ::Result<()> {
    let mut req = IodevPioReq {
        access: 1,
        port: port as u32,
        width: 1,
        val: data as u32,
    };
    unsafe { iodev_pio(f.as_raw_fd(), &mut req) }?;
    Ok(())
}

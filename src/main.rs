extern crate libc;
extern crate rand;

#[macro_use]
extern crate error_chain;

use rand::{Rng, SeedableRng, XorShiftRng};
use std::mem;
use std::slice;

mod area;
mod flash;
pub mod api;

use flash::Flash;
use area::{BootReq, CBootReq, FlashId};

fn main() {
    let (mut flash, bootreq) = if false {
        // STM style flash.  Large sectors, with a large scratch area.
        let flash = Flash::new(vec![16 * 1024, 16 * 1024, 16 * 1024, 16 * 1024,
                               64 * 1024,
                               128 * 1024, 128 * 1024, 128 * 1024]);
        let mut bootreq = BootReq::new(&flash);
        bootreq.add_image(0x020000, 0x020000, FlashId::Image0);
        bootreq.add_image(0x040000, 0x020000, FlashId::Image1);
        bootreq.add_image(0x060000, 0x020000, FlashId::ImageScratch);
        (flash, bootreq)
    } else {
        // NXP style flash.  Small sectors, one small sector for scratch.
        let flash = Flash::new(vec![4096; 128]);

        let mut bootreq = BootReq::new(&flash);
        bootreq.add_image(0x020000, 0x020000, FlashId::Image0);
        bootreq.add_image(0x040000, 0x020000, FlashId::Image1);
        bootreq.add_image(0x060000, 0x001000, FlashId::ImageScratch);
        (flash, bootreq)
    };

    // Install the boot trailer signature, so that the code will start an upgrade.
    install_image(&mut flash, 0x020000, 32779);

    // Install an upgrade image.
    let upgrade = install_image(&mut flash, 0x040000, 41922);

    // Mark the upgrade as ready to install.  (This looks like it might be a bug in the code,
    // however.)
    mark_upgrade(&mut flash, 0x03fff8);

    // show_flash(&flash);

    println!("First boot for upgrade");
    boot_go(&mut flash, &bootreq);

    verify_image(&flash, 0x020000, &upgrade);

    println!("\n------------------\nSecond boot");
    boot_go(&mut flash, &bootreq);
}

/// Show the flash layout.
#[allow(dead_code)]
fn show_flash(flash: &Flash) {
    println!("---- Flash configuration ----");
    for sector in flash.sector_iter() {
        println!("    {:2}: 0x{:08x}, 0x{:08x}",
                 sector.num, sector.base, sector.size);
    }
    println!("");
}

/// Invoke the bootloader on this flash device.
fn boot_go(flash: &mut Flash, bootreq: &BootReq) {
    unsafe { invoke_boot_go(flash as *mut _ as *mut libc::c_void,
                            &bootreq.get_c() as *const _) };
}

/// Install a "program" into the given image.  This fakes the image header, or at least all of the
/// fields used by the given code.  Returns a copy of the image that was written.
fn install_image(flash: &mut Flash, offset: usize, len: usize) -> Vec<u8> {
    let offset0 = offset;

    // Generate a boot header.  Note that the size doesn't include the header.
    let header = ImageHeader {
        magic: 0x96f3b83c,
        tlv_size: 0,
        _pad1: 0,
        hdr_size: 32,
        key_id: 0,
        _pad2: 0,
        img_size: len as u32,
        flags: 0,
        ver: ImageVersion {
            major: 1,
            minor: 0,
            revision: 1,
            build_num: 1,
        },
        _pad3: 0,
    };

    let b_header = header.as_raw();
    /*
    let b_header = unsafe { slice::from_raw_parts(&header as *const _ as *const u8,
                                                  mem::size_of::<ImageHeader>()) };
                                                  */
    assert_eq!(b_header.len(), 32);
    flash.write(offset, &b_header).unwrap();
    let offset = offset + b_header.len();

    // The core of the image itself is just pseudorandom data.
    let mut buf = vec![0; len];
    splat(&mut buf, offset);
    flash.write(offset, &buf).unwrap();
    let offset = offset + buf.len();

    // Copy out the image so that we can verify that the image was installed correctly later.
    let mut copy = vec![0u8; offset - offset0];
    flash.read(offset0, &mut copy).unwrap();

    copy
}

/// Verify that given image is present in the flash at the given offset.
fn verify_image(flash: &Flash, offset: usize, buf: &[u8]) {
    let mut copy = vec![0u8; buf.len()];
    flash.read(offset, &mut copy).unwrap();

    assert_eq!(buf, &copy[..]);
}

/// The image header
#[repr(C)]
pub struct ImageHeader {
    magic: u32,
    tlv_size: u16,
    key_id: u8,
    _pad1: u8,
    hdr_size: u16,
    _pad2: u16,
    img_size: u32,
    flags: u32,
    ver: ImageVersion,
    _pad3: u32,
}

impl AsRaw for ImageHeader {}

#[repr(C)]
pub struct ImageVersion {
    major: u8,
    minor: u8,
    revision: u16,
    build_num: u32,
}

/// Write out the magic so that the loader tries doing an upgrade.
fn mark_upgrade(flash: &mut Flash, offset: usize) {
    let magic = vec![0x21u8, 0x43, 0x34, 0x12];
    flash.write(offset, &magic).unwrap();
}

// Drop some pseudo-random gibberish onto the data.
fn splat(data: &mut [u8], seed: usize) {
    let seed_block = [0x135782ea, 0x92184728, data.len() as u32, seed as u32];
    let mut rng: XorShiftRng = SeedableRng::from_seed(seed_block);
    rng.fill_bytes(data);
}

/// Return a read-only view into the raw bytes of this object
trait AsRaw : Sized {
    fn as_raw<'a>(&'a self) -> &'a [u8] {
        unsafe { slice::from_raw_parts(self as *const _ as *const u8,
                                       mem::size_of::<Self>()) }
    }
}

extern "C" {
    // Unsure how to get rid of this warning.
    fn invoke_boot_go(flash: *mut libc::c_void, bootreq: *const CBootReq) -> libc::c_int;
}

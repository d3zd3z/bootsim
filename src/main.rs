extern crate libc;
extern crate rand;

#[macro_use]
extern crate error_chain;

use rand::{Rng, SeedableRng, XorShiftRng};
use std::mem;
use std::slice;

mod flash;
pub mod api;

use flash::Flash;

fn main() {
    let mut flash = Flash::new(vec![16 * 1024, 16 * 1024, 16 * 1024, 16 * 1024,
                               64 * 1024,
                               128 * 1024, 128 * 1024, 128 * 1024]);
    // let mut flash = gen_image();
    println!("boot go:");

    // Install the boot trailer signature, so that the code will start an upgrade.
    install_image(&mut flash, 0x020000, 32779);

    // Install an upgrade image.
    install_image(&mut flash, 0x040000, 41922);

    // Mark the upgrade as ready to install.  (This looks like it might be a bug in the code,
    // however.)
    mark_upgrade(&mut flash, 0x03fff8);

    show_flash(&flash);

    println!("First boot for upgrade");
    boot_go(&mut flash);

    println!("\n------------------\nSecond boot");
    boot_go(&mut flash);
}

/// Show the flash layout.
fn show_flash(flash: &Flash) {
    println!("---- Flash configuration ----");
    for sector in flash.sector_iter() {
        println!("    {:2}: 0x{:08x}, 0x{:08x}",
                 sector.num, sector.base, sector.size);
    }
    println!("");
}

/// Invoke the bootloader on this flash device.
fn boot_go(flash: &mut Flash) {
    unsafe { invoke_boot_go(flash as *mut _ as *mut libc::c_void) };
}

/// Install a "program" into the given image.  This fakes the image header, or at least all of the
/// fields used by the given code.
fn install_image(flash: &mut Flash, offset: usize, len: usize) {
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
    fn invoke_boot_go(flash: *mut libc::c_void) -> libc::c_int;
}

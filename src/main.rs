extern crate byteorder;
extern crate libc;
extern crate rand;

#[macro_use]
extern crate error_chain;

use byteorder::{NativeEndian, WriteBytesExt};
use rand::{Rng, SeedableRng, XorShiftRng};

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
    let mut header = vec![];

    // ih_magic
    header.write_u32::<NativeEndian>(0x96f3b83c).unwrap();
    // ih_tlv_size
    header.write_u16::<NativeEndian>(0).unwrap();
    // ih_key_id
    header.write_u8(0).unwrap();
    // pad1
    header.write_u8(0).unwrap();
    // ih_hdr_size
    header.write_u16::<NativeEndian>(32).unwrap();
    // pad2
    header.write_u16::<NativeEndian>(0).unwrap();
    // Image size.
    header.write_u32::<NativeEndian>(len as u32).unwrap();
    // Flags
    header.write_u32::<NativeEndian>(0).unwrap();
    // Version: major, minor, rev, build.
    header.write_u8(0).unwrap();
    header.write_u8(0).unwrap();
    header.write_u16::<NativeEndian>(0).unwrap();
    header.write_u32::<NativeEndian>(0).unwrap();
    // Pad
    header.write_u32::<NativeEndian>(0).unwrap();

    assert_eq!(header.len(), 32);

    flash.write(offset, &header).unwrap();
    let offset = offset + header.len();

    // The core of the image itself is just pseudorandom data.
    let mut buf = vec![0; len];
    splat(&mut buf, offset);
    flash.write(offset, &buf).unwrap();
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

extern "C" {
    fn invoke_boot_go(flash: *mut libc::c_void) -> libc::c_int;
}

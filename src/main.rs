extern crate libc;
extern crate rand;

#[macro_use]
extern crate error_chain;

use rand::{Rng, SeedableRng, XorShiftRng};

mod flash;
pub mod api;

use flash::Flash;

fn main() {
    let mut flash = Flash::new(vec![16 * 1024, 16 * 1024, 16 * 1024, 64 * 1024,
                               128 * 1024, 128 * 1024, 128 * 1024]);
    // let mut flash = gen_image();
    println!("boot go:");

    // Install the boot trailer signature, so that the code will start an upgrade.

    unsafe { invoke_boot_go(&mut flash) };
}

/*
/// Generate a flash image suitable for simulating image swapping.
fn gen_image() -> Vec<u8> {
    let mut result = vec![0xFF; 4 * 128 * 1024];

    /* Put in a boot-loader to make sure it isn't changed. */
    splat(&mut result[0 .. 32184], 0);

    /* Put a first image. */
    splat(&mut result[1 * 128 * 1024 .. 1 * 128 * 1024 + 49155], 1);

    result
}

// Drop some pseudo-random gibberish onto the data.
fn splat(data: &mut [u8], seed: usize) {
    let seed_block = [0x135782ea, 0x92184728, data.len() as u32, seed as u32];
    let mut rng: XorShiftRng = SeedableRng::from_seed(seed_block);
    rng.fill_bytes(data);
}
*/

extern "C" {
    fn invoke_boot_go(flash: *mut Flash) -> libc::c_int;
}

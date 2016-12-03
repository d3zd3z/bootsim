//! Describe flash areas.

use flash::{Flash, Sector};
use std::marker::PhantomData;

/// Build a boot request.  This is used to build up the area descriptors.
#[derive(Debug)]
pub struct BootReq {
    sectors: Vec<Sector>,
    areas: Vec<FlashArea>,
    slots: Vec<u8>,
    scratch: u8,
    size: u32,
}

impl BootReq {
    /// Construct an empty boot request.
    pub fn new(flash: &Flash) -> BootReq {
        BootReq {
            sectors: flash.sector_iter().collect(),
            areas: vec![],
            slots: vec![],
            scratch: 0,
            size: 0,
        }
    }

    /// Add a slot to the image.  The slot must align with erasable units in the flash device.
    /// Panics if the description is not valid.  There are also bootloader assumptions that the
    /// slots are SLOT0, SLOT1, and SCRATCH in that order.
    pub fn add_image(&mut self, base: usize, len: usize, id: FlashId) {
        let mut base = base;
        let mut len = len;
        let mut first = true;

        if id != FlashId::ImageScratch {
            if self.size == 0 {
                self.size = len as u32;
            } else {
                if self.size != len as u32 {
                    panic!("Multiple images of different sizes");
                }
            }
        }

        for sector in &self.sectors {
            if len == 0 {
                break;
            }
            // println!("Add: base={:x}, len={:x}, sector={:?}", base, len, sector);
            if base > sector.base + sector.size - 1 {
                continue;
            }
            if sector.base != base {
                panic!("Image does not start on a sector boundary");
            }

            if first {
                if id == FlashId::ImageScratch {
                    self.scratch = self.areas.len() as u8;
                } else {
                    self.slots.push(self.areas.len() as u8);
                }
                first = false;
            }

            self.areas.push(FlashArea {
                flash_id: id,
                pad: [0; 3],
                off: sector.base as u32,
                size: sector.size as u32,
            });

            base += sector.size;
            len -= sector.size;
        }

        if len != 0 {
            panic!("Image goes past end of device");
        }
    }

    pub fn get_c(&self) -> CBootReq {
        CBootReq {
            area_descs: &self.areas[0],
            slot_areas: &self.slots[0],
            num_image_areas: self.areas.len() as u8,
            scratch_area_idx: self.scratch,
            img_sz: self.size,
            phantom: PhantomData,
        }
    }
}

/// Boot request
#[repr(C)]
#[derive(Debug)]
pub struct CBootReq<'a> {
    /// Array of area descriptors indicating the layout of flash(es); must be terminated with a
    /// 0-length element.
    area_descs: *const FlashArea,

    /// Array of indices of elements in the array_descs array; indicates which areas represent the
    /// beginning of an image slot.  These are indices into area_descs.
    slot_areas: *const u8,

    /// The number of image areas (size of image_areas array (comment is wrong))
    num_image_areas: u8,

    /// The area to use as the image scratch area, index is index to br_area_descs array of the
    scratch_area_idx: u8,

    // (16 bits of padding here)

    /// Size of the image slot
    img_sz: u32,

    /// Holder of Phantom data for raw pointers.
    phantom: PhantomData<&'a BootReq>,
}

/// Flash area map.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub enum FlashId {
    BootLoader = 0,
    Image0 = 1,
    Image1 = 2,
    ImageScratch = 3,
    Nffs = 4,
    Core = 5,
    RebootLog = 6
}

#[repr(C)]
#[derive(Debug)]
pub struct FlashArea {
    flash_id: FlashId,
    pad: [u8; 3],
    off: u32,
    size: u32,
}


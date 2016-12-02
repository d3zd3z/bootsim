/* Run the boot image. */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <bootutil/loader.h>
#include <hal/flash_map.h>

#include "bootutil_priv.h"

extern int sim_flash_erase(void *flash, uint32_t offset, uint32_t size);
extern int sim_flash_read(void *flash, uint32_t offset, uint8_t *dest, uint32_t size);
extern int sim_flash_write(void *flash, uint32_t offset, const uint8_t *src, uint32_t size);

#if 1
/* TODO: This should really be const-able. */
/* TODO: Pass which flash device in, so it is dynamic. */

/*
 * Flash configuration to simulate the Carbon device
 */
static struct flash_area flash_areas[] = {
	{
		.fa_flash_id = FLASH_AREA_BOOTLOADER,
		.fa_off = 0,
		.fa_size = 128*1024,
	},
	{
		.fa_flash_id = FLASH_AREA_IMAGE_0,
		.fa_off = 128*1024,
		.fa_size = 128*1024,
	},
	{
		.fa_flash_id = FLASH_AREA_IMAGE_1,
		.fa_off = 2*128*1024,
		.fa_size = 128*1024,
	},
	{
		.fa_flash_id = FLASH_AREA_IMAGE_SCRATCH,
		.fa_off = 3*128*1024,
		.fa_size = 128*1024,
	},
	{
		.fa_size = 0,
	},
};

static uint8_t slot_areas[] = { 1, 2, 3 };
static const struct boot_req carbon_req = {
	.br_area_descs = flash_areas,
	.br_slot_areas = slot_areas,
	.br_scratch_area_idx = 2,
	.br_img_sz = 128*1024,
};

static struct image_header first_image = {
	.ih_magic = IMAGE_MAGIC,
};
static struct image_header second_image = {
	.ih_magic = IMAGE_MAGIC,
};
#endif

static void *flash_device;

int invoke_boot_go(void *flash)
{
	int res;
	struct boot_rsp rsp;

	flash_device = flash;
	res = boot_go(&carbon_req, &rsp);
	printf("boot_go result: %d (0x%08x)\n", res, rsp.br_image_addr);
	return res;
}

int boot_read_image_header(struct boot_image_location *loc,
			   struct image_header *out_hdr)
{
	printf("boot_read_image_header: %d, 0x%08x\n",
	       loc->bil_flash_id,
	       loc->bil_address);
	if (loc->bil_flash_id == FLASH_AREA_IMAGE_0) {
		memcpy(out_hdr, &first_image, sizeof(struct image_header));
		return 0;
	}
	if (loc->bil_flash_id == FLASH_AREA_IMAGE_1) {
		memcpy(out_hdr, &second_image, sizeof(struct image_header));
		return 0;
	}
	abort();
}

int hal_flash_read(uint8_t flash_id, uint32_t address, void *dst,
		   uint32_t num_bytes)
{
	printf("hal_flash_read: %d, 0x%08x (0x%x)\n",
	       flash_id, address, num_bytes);
	return sim_flash_read(flash_device, address, dst, num_bytes);
}

int hal_flash_write(uint8_t flash_id, uint32_t address,
		    const void *src, int32_t num_bytes)
{
	printf("hal_flash_write\n");
	abort();
}

int hal_flash_erase(uint8_t flash_id, uint32_t address,
		    uint32_t num_bytes)
{
	printf("hal_flash_erase\n");
	abort();
}

uint8_t hal_flash_align(uint8_t flash_id)
{
	return 1;
}

void *os_malloc(size_t size)
{
	printf("os_malloc\n");
	abort();
}

int bootutil_img_validate(struct image_header *hdr, uint8_t flash_id,
			  uint32_t addr, uint8_t *tmp_buf,
			  uint32_t tmp_buf_sz)
{
	printf("bootutil_img_validate\n");
	abort();
}

int boot_vect_write_test(int slot)
{
	printf("boot_vect_write_test\n");
	abort();
}

int boot_write_status(struct boot_status *bs)
{
	printf("boot_write_status\n");
	abort();
}

int boot_read_status(struct boot_status *bs)
{
	printf("boot_read_status\n");
	/* For now, just return no status. */
	return 0;
}

void boot_clear_status(void)
{
	printf("boot_write_status\n");
	abort();
}

/* Run the boot image. */

#include <setjmp.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <bootutil/bootutil.h>
#include <bootutil/image.h>
#include "flash_map/flash_map.h"

#include "bootutil_priv.h"

extern int sim_flash_erase(void *flash, uint32_t offset, uint32_t size);
extern int sim_flash_read(void *flash, uint32_t offset, uint8_t *dest, uint32_t size);
extern int sim_flash_write(void *flash, uint32_t offset, const uint8_t *src, uint32_t size);

static void *flash_device;
static jmp_buf boot_jmpbuf;
int flash_counter;

int invoke_boot_go(void *flash)
{
	int res;
	struct boot_rsp rsp;

	flash_device = flash;
	if (setjmp(boot_jmpbuf) == 0) {
		res = boot_go(&rsp);
		printf("boot_go result: %d (0x%08x)\n", res, rsp.br_image_addr);
		return res;
	} else {
		return -0x13579;
	}
}

int hal_flash_read(uint8_t flash_id, uint32_t address, void *dst,
		   uint32_t num_bytes)
{
	// printf("hal_flash_read: %d, 0x%08x (0x%x)\n",
	//        flash_id, address, num_bytes);
	return sim_flash_read(flash_device, address, dst, num_bytes);
}

int hal_flash_write(uint8_t flash_id, uint32_t address,
		    const void *src, int32_t num_bytes)
{
	if (--flash_counter == 0) {
		longjmp(boot_jmpbuf, 1);
	}
	// printf("hal_flash_write: 0x%08x (0x%x)\n", address, num_bytes);
	return sim_flash_write(flash_device, address, src, num_bytes);
}

int hal_flash_erase(uint8_t flash_id, uint32_t address,
		    uint32_t num_bytes)
{
	if (--flash_counter == 0) {
		longjmp(boot_jmpbuf, 1);
	}
	// printf("hal_flash_erase: 0x%08x, (0x%x)\n", address, num_bytes);
	return sim_flash_erase(flash_device, address, num_bytes);
}

uint8_t hal_flash_align(uint8_t flash_id)
{
	return 1;
}

void *os_malloc(size_t size)
{
	// printf("os_malloc 0x%x bytes\n", size);
	return malloc(size);
}

int flash_area_id_from_image_slot(int slot)
{
	printf("%s\n", __FUNCTION__);
	abort();
}

int flash_area_open(uint8_t id, const struct flash_area **area)
{
	printf("%s\n", __FUNCTION__);
	abort();
}

void flash_area_close(const struct flash_area *area)
{
	printf("%s\n", __FUNCTION__);
	abort();
}

/*
 * Read/write/erase. Offset is relative from beginning of flash area.
 */
int flash_area_read(const struct flash_area *area, uint32_t off, void *dst,
		    uint32_t len)
{
	printf("%s\n", __FUNCTION__);
	abort();
}

int flash_area_write(const struct flash_area *area, uint32_t off, const void *src,
		     uint32_t len)
{
	printf("%s\n", __FUNCTION__);
	abort();
}

int flash_area_erase(const struct flash_area *area, uint32_t off, uint32_t len)
{
	printf("%s\n", __FUNCTION__);
	abort();
}

int flash_area_to_sectors(int idx, int *cnt, struct flash_area *ret)
{
	printf("%s\n", __FUNCTION__);
	abort();
}

uint8_t flash_area_align(const struct flash_area *area)
{
	printf("%s\n", __FUNCTION__);
	abort();
}


int bootutil_img_validate(struct image_header *hdr,
                          const struct flash_area *fap,
                          uint8_t *tmp_buf, uint32_t tmp_buf_sz,
                          uint8_t *seed, int seed_len, uint8_t *out_hash)
{
	printf("%s\n", __FUNCTION__);
	abort();
}

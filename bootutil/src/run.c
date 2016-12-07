/* Run the boot image. */

#include <setjmp.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <bootutil/loader.h>
#include <hal/flash_map.h>

#include "bootutil_priv.h"

extern int sim_flash_erase(void *flash, uint32_t offset, uint32_t size);
extern int sim_flash_read(void *flash, uint32_t offset, uint8_t *dest, uint32_t size);
extern int sim_flash_write(void *flash, uint32_t offset, const uint8_t *src, uint32_t size);

static void *flash_device;
static jmp_buf boot_jmpbuf;
int flash_counter;

int invoke_boot_go(void *flash, struct boot_req *req)
{
	int res;
	struct boot_rsp rsp;

	flash_device = flash;
	if (setjmp(boot_jmpbuf) == 0) {
		res = boot_go(req, &rsp);
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

int bootutil_img_validate(struct image_header *hdr, uint8_t flash_id,
			  uint32_t addr, uint8_t *tmp_buf,
			  uint32_t tmp_buf_sz)
{
	printf("bootutil_img_validate: 0x%08x\n", addr);
	abort();
}

int boot_vect_write_test(int slot)
{
	printf("boot_vect_write_test\n");
	abort();
}

#if 0
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
#endif

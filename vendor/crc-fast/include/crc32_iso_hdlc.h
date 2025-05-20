/* Generated header for hardware-accelerated CRC-32/ISO_HDLC implementation */
/* Original implementation from https://github.com/corsix/fast-crc32/ */
/* MIT licensed */

#ifndef CRC32_ISO_HDLC_H
#define CRC32_ISO_HDLC_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * The target build properties (CPU architecture and fine-tuning parameters) for the compiled implementation.
 */
extern const char *const ISO_HDLC_TARGET;

/**
 * Gets the target build properties (CPU architecture and fine-tuning parameters) for this implementation.
 */
const char *get_iso_hdlc_target(void);

/**
 * Calculate CRC-32/ISO_HDLC checksum using hardware acceleration
 *
 * @param crc0 Initial CRC value (typically 0)
 * @param buf Pointer to input data buffer
 * @param len Length of input data in bytes
 *
 * @return Calculated CRC-32/ISO_HDLC checksum
 */
uint32_t crc32_iso_hdlc_impl(uint32_t crc0, const char* buf, size_t len);

#ifdef __cplusplus
}
#endif

#endif /* CRC32_ISO_HDLC_H */
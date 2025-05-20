/* Generated header for hardware-accelerated CRC-32/ISCSI implementation */
/* Original implementation from https://github.com/corsix/fast-crc32/ */
/* MIT licensed */

#ifndef CRC32_ISCSI_H
#define CRC32_ISCSI_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * The target build properties (CPU architecture and fine-tuning parameters) for the compiled implementation.
 */
extern const char *const ISCSI_TARGET;

/**
 * Gets the target build properties (CPU architecture and fine-tuning parameters) for this implementation.
 */
const char *get_iscsi_target(void);

/**
 * Calculate CRC-32/ISCSI checksum using hardware acceleration
 *
 * @param crc0 Initial CRC value (typically 0)
 * @param buf Pointer to input data buffer
 * @param len Length of input data in bytes
 *
 * @return Calculated CRC-32/ISCSI checksum
 */
uint32_t crc32_iscsi_impl(uint32_t crc0, const char* buf, size_t len);

#ifdef __cplusplus
}
#endif

#endif /* CRC32_ISCSI_H */
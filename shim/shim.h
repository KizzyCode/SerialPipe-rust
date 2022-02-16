#include <stdint.h>
#include <stddef.h>


/**
 * @brief Opens a serial device
 * 
 * @param device The pointer that will be set to the newly created device file descriptor
 * @param bauds The baud rate to use
 * @param path The path of the device to open
 * @return int `0` on success or an appropriate `errno` value otherwise
 */
int spipe_open(int* device, uint32_t bauds, const char* path);

/**
 * @brief Reads up to `buf_len` bytes into `buf`
 * 
 * @param buf The buffer to write the bytes to
 * @param buf_len The length of `buf`
 * @param bytes_read The pointer that will be set to the amount of bytes that have been read
 * @param device The device handle to operate on
 * @return int `0` on success or an appropriate `errno` value otherwise
 */
int spipe_read(uint8_t* buf, size_t buf_len, size_t* bytes_read, int device);

/**
 * @brief Writes `len` bytes from `buf`
 * 
 * @param buf The buffer to write the bytes from
 * @param buf_len The length of `buf`
 * @param bytes_written The pointer that will be set to the amount of bytes that have been written
 * @param device The device handle to operate on
 * @return int `0` on success or an appropriate `errno` value otherwise
 */
int spipe_write(const uint8_t* buf, size_t buf_len, size_t* bytes_written, int device);

/**
 * @brief Flushes all pending data
 * 
 * @param device The device handle to operate on
 * @return int `0` on success or an appropriate `errno` value otherwise
 */
int spipe_flush(int device);

/**
 * @brief Closes a serial device file descriptor
 * 
 * @param device The device file descriptor to close
 * @return int `0` on success or an appropriate `errno` value otherwise
 */
int spipe_close(int device);

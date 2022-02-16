#include "shim.h"
#include <unistd.h>
#include <fcntl.h>
#include <termios.h>
#include <errno.h>


int spipe_open(int* device, uint32_t bauds, const char* path) {
    // Open the file
    *device = open(path, O_RDWR | O_NDELAY | O_NOCTTY);
    if (*device < 0) {
        goto fail;
    }

    // Get the device config
    struct termios config;
    if (tcgetattr(*device, &config) < 0) {
        goto fail;
    }

    // Set the baudrate
    if (cfsetispeed(&config, (speed_t)bauds) < 0) {
        goto fail;
    }
    if (cfsetospeed(&config, (speed_t)bauds) < 0) {
        goto fail;
    }
    
    // Update the device config
    config.c_iflag &= ~(IGNBRK | BRKINT | ICRNL | INLCR | PARMRK | INPCK | ISTRIP | IXON);
    config.c_oflag = 0;
    config.c_lflag &= ~(ECHO | ECHONL | ICANON | IEXTEN | ISIG);
    config.c_cflag &= ~(CSIZE | PARENB);
    config.c_cflag |= CS8;
    config.c_cc[VMIN]  = 1;
    config.c_cc[VTIME] = 0;
    if (tcsetattr(*device, TCSAFLUSH, &config) < 0) {
        goto fail;
    }

    // Return success
    return 0;

fail:
    // Close the device if any
    if (*device >= 0) {
        close(*device);
        *device = -1;
    }

    // Return the errno
    return errno;
}


int spipe_read(uint8_t* buf, size_t buf_len, size_t* bytes_read, int device) {
    // Read up to buf_len bytes
    ssize_t result = read(device, buf, buf_len);
    if (result < 0) {
        return errno;
    }

    // Propagate the amount of bytes read and return success
    *bytes_read = (size_t)result;
    return 0;
}


int spipe_write(const uint8_t* buf, size_t buf_len, size_t* bytes_written, int device) {
    // Write up to buf_len bytes
    ssize_t result = write(device, buf, buf_len);
    if (result < 0) {
        return errno;
    }

    // Propagate the amount of bytes written and return success
    *bytes_written = (size_t)result;
    return 0;
}


int spipe_flush(int device) {
    // Sync the device
    if (fsync(device) < 0) {
        return errno;
    }

    // Return success
    return 0;
}


int spipe_close(int device) {
    // Close the device
    if (close(device) < 0) {
        return errno;
    }

    // Return success
    return 0;
}

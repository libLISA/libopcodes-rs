#include <stdint.h>
#include <stdarg.h>
#include <stdio.h>

typedef struct {
    char* ptr;
    uint64_t len;
} stream_state_t;

int print_to_buffer(void *stream, const char *fmt, ...);
int print_to_styled_buffer(void *stream, uint32_t style, const char *fmt, ...);
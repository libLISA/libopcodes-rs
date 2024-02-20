#include "print.h"

int print_to_buffer(void *stream, const char *fmt, ...) {
    stream_state_t *state = (stream_state_t *)stream;
    va_list arg;
    va_start(arg, fmt);
    uint64_t len = vsnprintf(state->ptr, state->len, fmt, arg);
    va_end(arg);

    if (len > state->len) {
        len = state->len;
    }

    state->ptr += len;
    state->len -= len;

    return 0;
}

int print_to_styled_buffer(void *stream, uint32_t _style, const char *fmt, ...) {
    // Suppress warning
    (void)(_style);

    stream_state_t *state = (stream_state_t *)stream;
    va_list arg;
    va_start(arg, fmt);
    uint64_t len = vsnprintf(state->ptr, state->len, fmt, arg);
    va_end(arg);

    if (len > state->len) {
        len = state->len;
    }

    state->ptr += len;
    state->len -= len;

    return 0;
}
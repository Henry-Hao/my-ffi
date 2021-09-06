#include <stdio.h>
#include <stdlib.h>

typedef void(*rust_callback)(int32_t);
rust_callback cb;

int32_t register_callback(rust_callback callback) {
    cb = callback;
    return 1;
}

void trigger_callback() {
    cb(3);
}

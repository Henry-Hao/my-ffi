#include <stdio.h>
#include <stdlib.h>

typedef void(*rust_callback)(void*, int32_t);

void* cb_target;
rust_callback cb;

int32_t register_callback(void* target, rust_callback callback) {
    cb = callback;
    cb_target = target;
    return 1;
}

void trigger_callback() {
    cb(cb_target,3);
}

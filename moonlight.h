#include <stdint.h>
#include <stdbool.h>

struct NativeView {
    const void **attributes_ptr;
    uintptr_t attributes_size;
    const struct NativeView **children_ptr;
    uintptr_t children_size;
    bool is_wrapper;
};

void render(const struct NativeView *tree);
void run_app(void);
void end_app(const struct NativeView *tree);

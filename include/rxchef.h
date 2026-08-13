#ifndef RXCHEF_H
#define RXCHEF_H

/* Experimental C ABI: source compatibility and ABI stability are not yet
 * guaranteed. Every returned allocation must be released exactly once by the
 * matching rxchef_free_* function. Callers retain ownership of input buffers. */

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct RxChefArgValue RxChefArgValue;

typedef struct RxChefResult {
    uint8_t *data;
    size_t length;
    size_t capacity;
    char *error;
} RxChefResult;

char *rxchef_list_operations(void);
char *rxchef_get_metadata(const char *op_name);
char *rxchef_get_all_metadata(void);
char *rxchef_magic(const uint8_t *input_data, size_t input_len);
void rxchef_free_string(char *value);

RxChefArgValue *rxchef_arg_str(const char *value);
RxChefArgValue *rxchef_arg_num(double value);
RxChefArgValue *rxchef_arg_bool(bool value);
RxChefArgValue *rxchef_arg_bytes(const uint8_t *data, size_t length);
void rxchef_free_arg(RxChefArgValue *argument);

RxChefResult *rxchef_run(const char *operation, const uint8_t *input_data,
                         size_t input_len, RxChefArgValue *const *arguments,
                         size_t argument_count);
void rxchef_free_result(RxChefResult *result);

#ifdef __cplusplus
}
#endif

#endif

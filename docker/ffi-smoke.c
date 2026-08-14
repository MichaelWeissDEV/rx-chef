#include "rxchef.h"

#include <stdio.h>
#include <string.h>

int main(void) {
    const uint8_t input[] = {0x00, 0x01, 0x02, 0xff};
    const uint8_t expected[] = "00 01 02 ff";
    RxChefResult *result = rxchef_run("To Hex", input, sizeof(input), NULL, 0);
    if (result == NULL) {
        fputs("rxchef_run returned NULL\n", stderr);
        return 1;
    }
    if (result->error != NULL || result->length != sizeof(expected) - 1 ||
        memcmp(result->data, expected, sizeof(expected) - 1) != 0) {
        fprintf(stderr, "unexpected FFI result: error=%s length=%zu data=",
                result->error == NULL ? "<none>" : result->error,
                result->length);
        if (result->data != NULL) {
            fwrite(result->data, 1, result->length, stderr);
        }
        fputc('\n', stderr);
        rxchef_free_result(result);
        return 1;
    }
    rxchef_free_result(result);

    result = rxchef_run("unknown operation", NULL, 0, NULL, 0);
    if (result == NULL || result->error == NULL) {
        fputs("unknown operation did not return a structured error\n", stderr);
        rxchef_free_result(result);
        return 1;
    }
    rxchef_free_result(result);
    return 0;
}

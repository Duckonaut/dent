#include "cdent.h"
#include <bits/stdint-intn.h>
#include <stdio.h>
#include <string.h>

const char* dent_data = "@merge [ [ 1 2 ] [ 3 4 ] ]";

int main(int argc, char* argv[]) {
    (void)argc;
    (void)argv;

    dent_init();

    dent_value_t* value = dent_parse(dent_data, strlen(dent_data));

    if (value == NULL) {
        printf("Error parsing dent data\n");
        return EXIT_FAILURE;
    }

    if (!dent_is_list(value)) {
        printf("Error: value is not a list\n");
        return EXIT_FAILURE;
    }

    int64_t sum = 0;

    for (size_t i = 0; i < dent_len(value); i++) {
        if (!dent_is_int(dent_get_index(value, i))) {
            printf("Error: value is not an int\n");
            return EXIT_FAILURE;
        }

        sum += dent_as_int(dent_get_index(value, i));

        printf("Value at index %zu: %ld\n", i, dent_as_int(dent_get_index(value, i)));
    }

    printf("Sum: %ld\n", sum);

    dent_free(value);

    dent_shutdown();

    return EXIT_SUCCESS;
}

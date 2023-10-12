#include "cdent.h"
#include <stdio.h>
#include <string.h>

const char* dent_data = "{ name: simple version: \"0.0.1\" }";

int main(int argc, char* argv[]) {
    (void)argc;
    (void)argv;

    dent_init();

    dent_value_t* value = dent_parse(dent_data, strlen(dent_data));

    if (value == NULL) {
        printf("Error parsing dent data\n");
        return EXIT_FAILURE;
    }

    dent_value_t* name = dent_get(value, "name");

    if (name == NULL) {
        printf("Error getting name\n");
        return EXIT_FAILURE;
    }

    if (!dent_is_str(name)) {
        printf("Error name is not a string\n");
        return EXIT_FAILURE;
    }

    char* name_str = dent_as_str(name);

    printf("Name: %s\n", name_str);

    dent_free_str(name_str);

    dent_free(value);

    dent_shutdown();

    return EXIT_SUCCESS;
}

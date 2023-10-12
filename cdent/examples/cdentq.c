#include "cdent.h"

#include <stdbool.h>
#include <stdio.h>
#include <string.h>

typedef struct args {
    char* file;
    char* query;
} args_t;

char* strndup(const char* str, size_t n) {
    char* new_str = malloc(n + 1);
    memcpy(new_str, str, n);
    new_str[n] = '\0';
    return new_str;
}

args_t parse_args(int argc, char* argv[]) {
    args_t args = { NULL, NULL };

    if (argc < 3) {
        printf("Usage: %s <file> <query>\n", argv[0]);
        exit(EXIT_FAILURE);
    }

    args.file = argv[1];

    args.query = argv[2];

    return args;
}

int main(int argc, char* argv[]) {
    args_t args = parse_args(argc, argv);

    dent_init();

    dent_value_t* value = dent_parse_file(args.file);

    if (value == NULL) {
        printf("Error parsing dent data\n");
        return EXIT_FAILURE;
    }

    dent_value_t* current = value;

    size_t pos = 0;
    const char* query = args.query;
    while (true) {
        if (query[pos] == '\0') {
            break;
        }

        if (query[pos] == '[') {
            if (!dent_is_list(current)) {
                printf("Error: value is not a list\n");
                return EXIT_FAILURE;
            }

            pos++;
            size_t start = pos;

            while (query[pos] != ']') {
                if (query[pos] == '\0') {
                    printf("Error: unexpected end of query\n");
                    return EXIT_FAILURE;
                }
                pos++;
            }

            char* index_str = strndup(query + start, pos - start);

            size_t index = strtoul(index_str, NULL, 10);

            if (index >= dent_len(current)) {
                printf("Error: index out of bounds\n");
                return EXIT_FAILURE;
            }

            current = dent_get_index(current, index);

            free(index_str);

            pos++;
        } else if (query[pos] == ' ') {
            pos++;
        } else if (query[pos] == '.') {
            if (!dent_is_dict(current)) {
                printf("Error: value is not a dict\n");
                return EXIT_FAILURE;
            }

            pos++;

            if (query[pos] == '\0') {
                break;
            }

            if (query[pos] == '"') {
                pos++;
                size_t start = pos;
                while (query[pos] != '"') {
                    if (query[pos] == '\0') {
                        printf("Error: unexpected end of query\n");
                        return EXIT_FAILURE;
                    }
                    pos++;
                }
                size_t len = pos - start;

                char* key = strndup(query + start, len);

                dent_value_t* next = dent_get(current, key);

                if (next == NULL) {
                    printf("Error: key not found\n");
                    return EXIT_FAILURE;
                }

                current = next;

                free(key);
            } else {
                size_t start = pos;
                while (query[pos] != ' ' && query[pos] != '\0' && query[pos] != '[' &&
                       query[pos] != '.') {
                    pos++;
                }
                size_t len = pos - start;

                char* key = strndup(query + start, len);

                dent_value_t* next = dent_get(current, key);

                if (next == NULL) {
                    printf("Error: key not found\n");
                    return EXIT_FAILURE;
                }

                current = next;

                free(key);
            }

        } else {
            printf("Error: unexpected character '%c'\n", query[pos]);
            return EXIT_FAILURE;
        }
    }

    char* value_str = dent_to_str(current);

    printf("%s\n", value_str);

    dent_free_str(value_str);

    dent_free(value);

    dent_shutdown();

    return EXIT_SUCCESS;
}

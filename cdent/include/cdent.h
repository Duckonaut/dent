#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct dent_value dent_value_t;

void dent_init(void);

void dent_shutdown(void);

dent_value_t *dent_parse(const char *input, uintptr_t len);

dent_value_t *dent_parse_file(const char *path);

void dent_free(dent_value_t *value);

dent_value_t *dent_get(const dent_value_t *value, const char *key);

dent_value_t *dent_get_index(const dent_value_t *value, uintptr_t index);

bool dent_is_none(const dent_value_t *value);

bool dent_is_str(const dent_value_t *value);

bool dent_is_bool(const dent_value_t *value);

bool dent_is_int(const dent_value_t *value);

bool dent_is_float(const dent_value_t *value);

bool dent_is_list(const dent_value_t *value);

bool dent_is_dict(const dent_value_t *value);

uintptr_t dent_len(const dent_value_t *value);

bool dent_is_empty(const dent_value_t *value);

char *dent_as_str(const dent_value_t *value);

void dent_free_str(char *value);

bool dent_as_bool(const dent_value_t *value);

int64_t dent_as_int(const dent_value_t *value);

double dent_as_float(const dent_value_t *value);

dent_value_t *dent_list_get(dent_value_t *value, uintptr_t index);

dent_value_t *dent_dict_get(dent_value_t *value, const char *key);

char *dent_to_str(const dent_value_t *value);

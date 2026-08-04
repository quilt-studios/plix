#pragma once

#include <stddef.h>
#include <stdint.h>

#define PLIX_EVERYFILE_MAX_CHILDREN 8u
#define PLIX_EVERYFILE_MAX_PATH 128u

typedef enum plix_everyfile_kind {
    PLIX_EVERYFILE_DIRECTORY,
    PLIX_EVERYFILE_TEXT,
} plix_everyfile_kind_t;

typedef struct plix_everyfile_node plix_everyfile_node_t;

struct plix_everyfile_node {
    const char *name;
    plix_everyfile_kind_t kind;
    const char *text;
    plix_everyfile_node_t *parent;
    plix_everyfile_node_t *children[PLIX_EVERYFILE_MAX_CHILDREN];
    size_t child_count;
};

typedef struct plix_everyfile {
    plix_everyfile_node_t *root;
    plix_everyfile_node_t *current;
    plix_everyfile_node_t *active_user;
} plix_everyfile_t;

void plix_everyfile_init(plix_everyfile_t *fs);
int plix_everyfile_enter_user(plix_everyfile_t *fs, const char *name);
const plix_everyfile_node_t *plix_everyfile_current(const plix_everyfile_t *fs);
int plix_everyfile_goto(plix_everyfile_t *fs, const char *path);
size_t plix_everyfile_show(const plix_everyfile_t *fs, const char **names, size_t capacity);
size_t plix_everyfile_path(const plix_everyfile_t *fs, char *buffer, size_t capacity);

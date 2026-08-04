#include <plix/everyfile.h>

static plix_everyfile_node_t root;
static plix_everyfile_node_t users;
static plix_everyfile_node_t root_user;
static plix_everyfile_node_t root_main_dir;
static plix_everyfile_node_t root_house_dir;
static plix_everyfile_node_t root_system_note;
static plix_everyfile_node_t root_welcome_note;
static plix_everyfile_node_t guest_user;
static plix_everyfile_node_t guest_main_dir;
static plix_everyfile_node_t guest_house_dir;
static plix_everyfile_node_t guest_system_note;
static plix_everyfile_node_t guest_welcome_note;

static int plix_streq(const char *left, const char *right) {
    while (*left != '\0' && *right != '\0') {
        if (*left != *right) {
            return 0;
        }
        ++left;
        ++right;
    }
    return *left == '\0' && *right == '\0';
}

static void attach(plix_everyfile_node_t *parent, plix_everyfile_node_t *child) {
    if (parent->child_count < PLIX_EVERYFILE_MAX_CHILDREN) {
        parent->children[parent->child_count] = child;
        ++parent->child_count;
        child->parent = parent;
    }
}

static void node_init(plix_everyfile_node_t *node, const char *name, plix_everyfile_kind_t kind, const char *text) {
    node->name = name;
    node->kind = kind;
    node->text = text;
    node->parent = 0;
    node->child_count = 0;
    for (size_t i = 0; i < PLIX_EVERYFILE_MAX_CHILDREN; ++i) {
        node->children[i] = 0;
    }
}

void plix_everyfile_init(plix_everyfile_t *fs) {
    node_init(&root, "", PLIX_EVERYFILE_DIRECTORY, 0);
    node_init(&users, "users", PLIX_EVERYFILE_DIRECTORY, 0);
    node_init(&root_user, "root", PLIX_EVERYFILE_DIRECTORY, 0);
    node_init(&root_main_dir, "main", PLIX_EVERYFILE_DIRECTORY, 0);
    node_init(&root_house_dir, "house", PLIX_EVERYFILE_DIRECTORY, 0);
    node_init(&root_system_note, "system", PLIX_EVERYFILE_TEXT, "main ersetzt etc fuer globale Konfiguration");
    node_init(&root_welcome_note, "welcome", PLIX_EVERYFILE_TEXT, "house ersetzt home fuer Benutzerdaten");
    node_init(&guest_user, "guest", PLIX_EVERYFILE_DIRECTORY, 0);
    node_init(&guest_main_dir, "main", PLIX_EVERYFILE_DIRECTORY, 0);
    node_init(&guest_house_dir, "house", PLIX_EVERYFILE_DIRECTORY, 0);
    node_init(&guest_system_note, "system", PLIX_EVERYFILE_TEXT, "guest main konfiguration");
    node_init(&guest_welcome_note, "welcome", PLIX_EVERYFILE_TEXT, "guest house daten");

    attach(&root, &users);
    attach(&users, &root_user);
    attach(&users, &guest_user);
    attach(&root_user, &root_main_dir);
    attach(&root_user, &root_house_dir);
    attach(&root_main_dir, &root_system_note);
    attach(&root_house_dir, &root_welcome_note);
    attach(&guest_user, &guest_main_dir);
    attach(&guest_user, &guest_house_dir);
    attach(&guest_main_dir, &guest_system_note);
    attach(&guest_house_dir, &guest_welcome_note);

    fs->root = &root;
    fs->active_user = &root_user;
    fs->current = &root_user;
}

const plix_everyfile_node_t *plix_everyfile_current(const plix_everyfile_t *fs) {
    return fs->current;
}

static plix_everyfile_node_t *find_child(plix_everyfile_node_t *node, const char *name, size_t length) {
    for (size_t i = 0; i < node->child_count; ++i) {
        const char *candidate = node->children[i]->name;
        size_t j = 0;
        while (j < length && candidate[j] != '\0' && candidate[j] == name[j]) {
            ++j;
        }
        if (j == length && candidate[j] == '\0') {
            return node->children[i];
        }
    }
    return 0;
}

int plix_everyfile_goto(plix_everyfile_t *fs, const char *path) {
    plix_everyfile_node_t *cursor = path[0] == '/' ? fs->root : fs->current;
    size_t start = path[0] == '/' ? 1u : 0u;

    if (path[start] == '\0') {
        fs->current = cursor;
        return 0;
    }

    while (path[start] != '\0') {
        size_t end = start;
        while (path[end] != '/' && path[end] != '\0') {
            ++end;
        }

        if (end == start || (end - start == 1u && path[start] == '.')) {
            /* stay */
        } else if (end - start == 2u && path[start] == '.' && path[start + 1u] == '.') {
            if (cursor->parent != 0) {
                cursor = cursor->parent;
            }
        } else {
            plix_everyfile_node_t *next = find_child(cursor, path + start, end - start);
            if (next == 0 || next->kind != PLIX_EVERYFILE_DIRECTORY) {
                return -1;
            }
            cursor = next;
        }

        start = path[end] == '/' ? end + 1u : end;
    }

    fs->current = cursor;
    return 0;
}

size_t plix_everyfile_show(const plix_everyfile_t *fs, const char **names, size_t capacity) {
    size_t count = fs->current->child_count;
    size_t copied = count < capacity ? count : capacity;
    for (size_t i = 0; i < copied; ++i) {
        names[i] = fs->current->children[i]->name;
    }
    return count;
}

size_t plix_everyfile_path(const plix_everyfile_t *fs, char *buffer, size_t capacity) {
    const plix_everyfile_node_t *stack[PLIX_EVERYFILE_MAX_PATH / 2u];
    size_t depth = 0;
    const plix_everyfile_node_t *cursor = fs->current;
    while (cursor != 0 && depth < (PLIX_EVERYFILE_MAX_PATH / 2u)) {
        stack[depth] = cursor;
        ++depth;
        cursor = cursor->parent;
    }

    size_t written = 0;
    if (capacity > 0u) {
        buffer[written++] = '/';
    }

    while (depth > 0u && written < capacity) {
        --depth;
        const char *name = stack[depth]->name;
        if (name[0] == '\0') {
            continue;
        }
        for (size_t i = 0; name[i] != '\0' && written + 1u < capacity; ++i) {
            buffer[written++] = name[i];
        }
        if (depth > 0u && written + 1u < capacity) {
            buffer[written++] = '/';
        }
    }

    if (capacity > 0u) {
        if (written >= capacity) {
            written = capacity - 1u;
        }
        buffer[written] = '\0';
    }
    return written;
}

int plix_everyfile_name_is_reserved(const char *name) {
    return plix_streq(name, "etc") || plix_streq(name, "home");
}

int plix_everyfile_enter_user(plix_everyfile_t *fs, const char *name) {
    for (size_t i = 0; i < users.child_count; ++i) {
        const char *candidate = users.children[i]->name;
        size_t j = 0;
        while (candidate[j] != '\0' && name[j] != '\0' && candidate[j] == name[j]) {
            ++j;
        }
        if (candidate[j] == '\0' && name[j] == '\0') {
            fs->active_user = users.children[i];
            fs->current = users.children[i];
            return 0;
        }
    }
    return -1;
}

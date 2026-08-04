#include <plix/auth.h>

#define FNV_OFFSET 1469598103934665603ULL
#define FNV_PRIME 1099511628211ULL

static const plix_user_t users[] = {
    {"root", 0x3684f6712bedfc93ULL, 0x706c6978726f6f74ULL, 1},
    {"guest", 0xc824e1f335353f6eULL, 0x706c697867756573ULL, 0},
};

static int streq(const char *left, const char *right) {
    while (*left != '\0' && *right != '\0') {
        if (*left != *right) {
            return 0;
        }
        ++left;
        ++right;
    }
    return *left == '\0' && *right == '\0';
}

static uint64_t hash_byte(uint64_t hash, uint8_t value) {
    hash ^= value;
    return hash * FNV_PRIME;
}

uint64_t plix_auth_hash_password(const char *name, const char *password, uint64_t salt) {
    uint64_t hash = FNV_OFFSET;
    for (unsigned shift = 0; shift < 64u; shift += 8u) {
        hash = hash_byte(hash, (uint8_t)(salt >> shift));
    }
    for (unsigned i = 0; name[i] != '\0'; ++i) {
        hash = hash_byte(hash, (uint8_t)name[i]);
    }
    hash = hash_byte(hash, (uint8_t)':');
    for (unsigned i = 0; password[i] != '\0'; ++i) {
        hash = hash_byte(hash, (uint8_t)password[i]);
    }
    return hash;
}

void plix_auth_init(plix_session_t *session) {
    session->user = &users[1];
    session->authenticated = 1;
}

int plix_auth_login(plix_session_t *session, const char *name, const char *password) {
    for (unsigned i = 0; i < sizeof(users) / sizeof(users[0]); ++i) {
        if (streq(users[i].name, name) && users[i].password_hash == plix_auth_hash_password(name, password, users[i].salt)) {
            session->user = &users[i];
            session->authenticated = 1;
            return 0;
        }
    }
    return -1;
}

int plix_auth_check_password(const plix_session_t *session, const char *password) {
    if (session->user == 0) {
        return 0;
    }
    return session->user->password_hash == plix_auth_hash_password(session->user->name, password, session->user->salt);
}

const char *plix_auth_user_name(const plix_session_t *session) {
    return session->user == 0 ? "none" : session->user->name;
}

int plix_auth_is_power_user(const plix_session_t *session) {
    return session->user != 0 && session->authenticated != 0 && session->user->power_user != 0;
}

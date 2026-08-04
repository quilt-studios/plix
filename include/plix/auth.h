#pragma once

#include <stdint.h>

#define PLIX_AUTH_MAX_NAME 16u

typedef struct plix_user {
    const char *name;
    uint64_t password_hash;
    uint64_t salt;
    int power_user;
} plix_user_t;

typedef struct plix_session {
    const plix_user_t *user;
    int authenticated;
} plix_session_t;

void plix_auth_init(plix_session_t *session);
int plix_auth_login(plix_session_t *session, const char *name, const char *password);
int plix_auth_check_password(const plix_session_t *session, const char *password);
const char *plix_auth_user_name(const plix_session_t *session);
int plix_auth_is_power_user(const plix_session_t *session);
uint64_t plix_auth_hash_password(const char *name, const char *password, uint64_t salt);

#include <plix/cli.h>
#include <plix/auth.h>
#include <plix/console.h>
#include <plix/everyfile.h>

static plix_everyfile_t fs;
static plix_session_t session;
static plix_cli_t boot_cli;

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

static void append_char(plix_cli_t *cli, char value) {
    if (cli->output_len + 1u < PLIX_CLI_OUTPUT_CAPACITY) {
        cli->output[cli->output_len] = value;
        ++cli->output_len;
        cli->output[cli->output_len] = '\0';
        plix_console_putc(value);
    }
}

static void append(plix_cli_t *cli, const char *text) {
    for (size_t i = 0; text[i] != '\0'; ++i) {
        append_char(cli, text[i]);
    }
}

static const char *skip_spaces(const char *text) {
    while (*text == ' ' || *text == '\t') {
        ++text;
    }
    return text;
}

static size_t token_len(const char *text) {
    size_t len = 0;
    while (text[len] != '\0' && text[len] != ' ' && text[len] != '\t') {
        ++len;
    }
    return len;
}

static int token_is(const char *token, size_t len, const char *expected) {
    size_t i = 0;
    while (i < len && expected[i] != '\0' && token[i] == expected[i]) {
        ++i;
    }
    return i == len && expected[i] == '\0';
}

void plix_cli_init(plix_cli_t *cli) {
    plix_everyfile_init(&fs);
    plix_auth_init(&session);
    (void)plix_everyfile_enter_user(&fs, plix_auth_user_name(&session));
    cli->output_len = 0;
    cli->output[0] = '\0';
    append(cli, "plix cli ready at ");
    char path[PLIX_EVERYFILE_MAX_PATH];
    (void)plix_everyfile_path(&fs, path, sizeof(path));
    append(cli, path);
    append(cli, "\n");
}

int plix_cli_execute(plix_cli_t *cli, const char *line) {
    const char *command = skip_spaces(line);
    size_t command_len = token_len(command);
    const char *argument = skip_spaces(command + command_len);


    if (token_is(command, command_len, "login")) {
        const char *name = argument;
        size_t name_len = token_len(name);
        const char *password = skip_spaces(name + name_len);
        if (name_len == 0u || password[0] == '\0') {
            append(cli, "login braucht user und passwort\n");
            return -1;
        }
        char user_name[PLIX_AUTH_MAX_NAME];
        size_t copied = name_len < (PLIX_AUTH_MAX_NAME - 1u) ? name_len : (PLIX_AUTH_MAX_NAME - 1u);
        for (size_t i = 0; i < copied; ++i) {
            user_name[i] = name[i];
        }
        user_name[copied] = '\0';
        if (plix_auth_login(&session, user_name, password) != 0 || plix_everyfile_enter_user(&fs, user_name) != 0) {
            append(cli, "login fehlgeschlagen\n");
            return -1;
        }
        append(cli, "angemeldet als ");
        append(cli, plix_auth_user_name(&session));
        append(cli, "\n");
        return 0;
    }

    if (token_is(command, command_len, "who")) {
        append(cli, plix_auth_user_name(&session));
        append(cli, "\n");
        return 0;
    }

    if (token_is(command, command_len, "goto") || token_is(command, command_len, "gt")) {
        if (argument[0] == '\0') {
            append(cli, "goto braucht ein ziel\n");
            return -1;
        }
        if (plix_everyfile_goto(&fs, argument) != 0) {
            append(cli, "ziel nicht gefunden\n");
            return -1;
        }
        append(cli, "jetzt in ");
        char path[PLIX_EVERYFILE_MAX_PATH];
        (void)plix_everyfile_path(&fs, path, sizeof(path));
        append(cli, path);
        append(cli, "\n");
        return 0;
    }

    if (token_is(command, command_len, "show") || token_is(command, command_len, "sw")) {
        const char *names[PLIX_EVERYFILE_MAX_CHILDREN];
        size_t count = plix_everyfile_show(&fs, names, PLIX_EVERYFILE_MAX_CHILDREN);
        for (size_t i = 0; i < count && i < PLIX_EVERYFILE_MAX_CHILDREN; ++i) {
            append(cli, names[i]);
            append(cli, "\n");
        }
        return 0;
    }

    if (token_is(command, command_len, "pudo")) {
        const char *password = argument;
        size_t password_len = token_len(password);
        const char *power_command = skip_spaces(password + password_len);
        char pudo_password[PLIX_AUTH_MAX_NAME];
        size_t pudo_copied = password_len < (PLIX_AUTH_MAX_NAME - 1u) ? password_len : (PLIX_AUTH_MAX_NAME - 1u);
        for (size_t i = 0; i < pudo_copied; ++i) {
            pudo_password[i] = password[i];
        }
        pudo_password[pudo_copied] = '\0';
        if (!plix_auth_is_power_user(&session) || password_len == 0u || plix_auth_check_password(&session, pudo_password) == 0) {
            append(cli, "pudo verweigert\n");
            return -1;
        }
        append(cli, "pudo erlaubt: ");
        append(cli, power_command[0] == '\0' ? "<leer>" : power_command);
        append(cli, "\n");
        return 0;
    }

    if (streq(command, "")) {
        return 0;
    }

    append(cli, "unbekannter befehl\n");
    return -1;
}

const char *plix_cli_output(const plix_cli_t *cli) {
    return cli->output;
}

void plix_cli_boot(void) {
    plix_cli_init(&boot_cli);
    (void)plix_cli_execute(&boot_cli, "show");
}

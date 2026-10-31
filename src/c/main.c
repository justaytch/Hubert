#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <signal.h>
#include <errno.h>
#include "tomlc17.h"

static void error(const char *msg) {
  fprintf(stderr, "ERROR: %s\n", msg);
  exit(1);
}

int main(void) {
    toml_result_t result = toml_parse_file_ex("./config.toml");
    if (!result.ok) {
        toml_free(result);
        error(result.errmsg);
    }
    toml_datum_t botname = toml_seek(result.toptab, "name");
    if (botname.type != TOML_STRING) {
        toml_free(result);
        error("Invalid name, check config.");
    }
    toml_datum_t discord = toml_seek(result.toptab, "discord.enabled");
    if (discord.type != TOML_BOOLEAN) {
        toml_free(result);
        error("Invalid discord.enabled, check config.");
    }
    toml_datum_t fluxer = toml_seek(result.toptab, "fluxer.enabled");
    if (fluxer.type != TOML_BOOLEAN) {
        toml_free(result);
        error("Invalid fluxer.enabled, check config.");
    }
    toml_datum_t stoat = toml_seek(result.toptab, "stoat.enabled");
    if (stoat.type != TOML_BOOLEAN) {
        toml_free(result);
        error("Invalid stoat.enabled, check config.");
    }
    pid_t bot_pgid = -1;
    pid_t discordpid = -1;
    pid_t fluxerpid = -1;
    int bots_started = 0;
    if (discord.u.boolean) {
        discordpid = fork();
        if (discordpid < 0) {
            toml_free(result);
            error("fork");
        }
        if (discordpid == 0) {
            if (bot_pgid == -1) {
                setpgid(0, 0);
            }
            else {
                setpgid(0, bot_pgid);
            }
            execl("./output/discordbot", "./output/discordbot", (char *)NULL);
            toml_free(result);
            error("execl");
            _exit(127);
        }
        if (bot_pgid == -1) {
            bot_pgid = discordpid;
        }
        if (setpgid(discordpid, bot_pgid) < 0 && errno != EACCES) {
            toml_free(result);
            error("setpgid");
        }
        bots_started++;
        printf("Started %s Discord as process %ld\n",botname.u.s,(long)discordpid);
    }
    if (fluxer.u.boolean) {
        fluxerpid = fork();
        if (fluxerpid < 0) {
            toml_free(result);
            error("fork");
        }
        if (fluxerpid == 0) {
            if (bot_pgid == -1) {
                setpgid(0, 0);
            }
            else {
                setpgid(0, bot_pgid);
            }
            execl("./output/fluxerbot", "./output/fluxerbot", (char *)NULL);
            toml_free(result);
            error("execl");
            _exit(127);
        }
        if (bot_pgid == -1) {
            bot_pgid = fluxerpid;
        }
        if (setpgid(fluxerpid, bot_pgid) < 0 && errno != EACCES) {
            toml_free(result);
            error("setpgid");
        }
        bots_started++;
        printf("Started %s Fluxer as process %ld\n",botname.u.s,(long)fluxerpid);
    }
    if (stoat.u.boolean) {
        printf("Hello from Stoat!!!\n");
    }
    if (bots_started > 0) {
        int status;
        pid_t ended = waitpid(-1, &status, 0);
        if (ended < 0) {
            toml_free(result);
            error("waitpid");
        }
        printf("Bot process %ld exited, stopping the other bots...\n", (long)ended);
        if (kill(-bot_pgid, SIGTERM) < 0 && errno != ESRCH) {
            toml_free(result);
            error("kill");
        }
        while (waitpid(-1, NULL, 0) > 0);
    }

    toml_free(result);
    return 0;
}

/* Invented standalone Seatbelt probe. No credentials or application data. */
#include <errno.h>
#include <spawn.h>
#include <stdio.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

int main(int argc, char **argv) {
    if (argc == 2 && strcmp(argv[1], "--child") == 0) return 0;
    if (argc == 2 && strcmp(argv[1], "--anchor") == 0) {
        printf("anchor_group=%ld\n", (long)getpgrp()); fflush(stdout);
        char command[16] = {0};
        ssize_t count = read(STDIN_FILENO, command, sizeof command - 1);
        if (count != 5 || memcmp(command, "KILL\n", 5) != 0) return 2;
        char *args[] = {"/bin/kill", "-KILL", "0", NULL};
        char *empty[] = {NULL};
        execve(args[0], args, empty);
        return 2;
    }
    if (argc == 2 && strcmp(argv[1], "--hold-stdio") == 0) {
        const char ready[] = "{\"method\":\"fixture/trivial-ready\",\"params\":{}}\n";
        if (write(STDOUT_FILENO, ready, sizeof ready - 1) != sizeof ready - 1) return 2;
        char buffer[256];
        ssize_t count;
        do { count = read(STDIN_FILENO, buffer, sizeof buffer); } while (count > 0);
        return count == 0 ? 0 : 2;
    }
    if (argc != 2) return 2;
    errno = 0;
    pid_t child = fork();
    int fork_error = child == -1 ? errno : 0;
    if (child == 0) _exit(0);
    if (child > 0 && waitpid(child, NULL, 0) != child) return 2;

    char *child_args[] = {argv[0], "--child", NULL};
    char *empty_env[] = {NULL};
    int spawn_error = posix_spawn(&child, argv[0], NULL, NULL, child_args, empty_env);
    if (spawn_error == 0 && waitpid(child, NULL, 0) != child) return 2;

    char *exec_args[] = {argv[1], "--child", NULL};
    errno = 0;
    execve(exec_args[0], exec_args, empty_env);
    int exec_error = errno;
    printf("fork_errno=%d spawn_errno=%d exec_errno=%d\n", fork_error, spawn_error, exec_error);
    return fork_error == EPERM && spawn_error == EPERM && exec_error == EPERM ? 0 : 1;
}

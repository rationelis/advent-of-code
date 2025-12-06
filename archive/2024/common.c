#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "common.h"

void read_input(char **input, int *length) {
    char buffer[LINE_BYTES];
    *length = 0;

    while (fgets(buffer, sizeof(buffer), stdin) != NULL) {
        input[*length] = malloc(strlen(buffer) + 1);
        if (!input[*length]) {
            fprintf(stderr, "Memory allocation failed\n");
            exit(1);
        }
        strcpy(input[(*length)++], buffer);
    }
}


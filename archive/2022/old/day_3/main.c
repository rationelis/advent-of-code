#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <ctype.h>

#define FILE_LENGTH 300
#define MAX_LENGTH 256

int get_value(char s) {
    return isupper(s) ? s - 38 : s - 96;
}

void remove_value(int *a, size_t size, int value) {
    for (size_t i = 0; i < size; i++) {
        if (a[i] == value) {
            a[i] = 0;
        }
    } 
}

int size_of_array(int *a) {
    int size = 0;
    for (int i = 1; i < 100; i++) {
        if (!a[i]) {
            return size;
        }
        size += 1;
    }
}

int in_array(int *a, int value) {
    for (int i = 0; i < size_of_array(a); i++) {
        // printf("Is %i equal %i\n", a[i], value);
        if (a[i] == value) {
            // printf("On %i - value %i\n", i, value);
            return 0;
        } 
    }

    // printf("Was not in the list.\n");

    return 1;
}

int calc_part_1(FILE *f) {
    int sum = 0;

    char buffer[MAX_LENGTH];

    while (fgets(buffer, MAX_LENGTH, f)) {
        size_t middle = (strlen(buffer) - 1) / 2;

        int *left = malloc(sizeof(int) * middle);
        int *right = malloc(sizeof(int) * middle);

        if (!left || !right) {
            return 1;
        }

        for (size_t i = 0; i < middle; i++) {
            left[i] = get_value(buffer[i]);
        }

        for (size_t i = 0; i < middle; i++) {
            right[i] = get_value(buffer[i + middle]);
        }
        
        for (int i = 0; i < middle; i++) {
            if (left[i] == 0) {
                continue;
            }

            for (int j = 0; j < middle; j++) {
                if (left[i] == right[j]) {
                    // Found duplicate.
                    sum += left[i];

                    // Remove other occurrences.
                    remove_value(left, middle, left[i]);

                    break;
                }
            }
        }

        free(left);
        free(right);
    }

    return sum;
}

int calc_part_2(FILE *f) {
    char buffer[MAX_LENGTH];

    int sum = 0;

    int **values = malloc(sizeof(int*) * FILE_LENGTH - 1);

    if (!values) {
        return 1;
    }

    int count = 0; 

    while (fgets(buffer, MAX_LENGTH, f)) {
        size_t length = strlen(buffer) - 1;

        values[count] = malloc(sizeof(int) * length);

        if (!values[count]) {
            return 1;
        }

        for (size_t i = 0; i < length; i++) {
            values[count][i] = get_value(buffer[i]);
        }

        count += 1;
    }
    
    int cycle = 0;

    // For every line.
    for (int rows = 0; rows < FILE_LENGTH; rows += 3) {
        printf("%i\n", rows);
        // For every value on row 1.
        for (int i = 0; i < size_of_array(values[rows]); i++) {
            // Row 1 value
            int value = values[rows][i];

            if (value == 0) {
                continue;
            }

            // If row 1 value is in row 2
            if (in_array(values[rows+1], value) == 0) {
                for (int j = 0; j < size_of_array(values[rows+2]); j++) {
                    // If row 1 value is in row 3
                    if (in_array(values[rows+2], value) == 0) {
                        printf("- %i was all 3 rucksacks\n", value);
                        sum += value;
                        remove_value(values[rows], size_of_array(values[rows]), value);
                        break;
                    }
                }
            }      
        }

        // return sum;
    }

    for (size_t i = 0; i < FILE_LENGTH; i++) {
        free(values[i]);
    }

    free(values);

    return sum;
}

int main(int argc, char *argv[]) {
    FILE *f = fopen("input.txt", "r");

    if (!f) {
        return 1;
    }

    // printf("Part 1: %i\n", calc_part_1(f));
    printf("Part 2: %i\n", calc_part_2(f));

    return 0;
}
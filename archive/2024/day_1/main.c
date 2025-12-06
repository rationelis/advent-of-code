#include <stdio.h>
#include <stdlib.h>
#include "../common.h"

int comp(const void* a, const void* b) {
    return (*(int*)a - *(int*)b);
}

int main() {
    char *input[INPUT_LENGTH];
    int length;

    read_input(input, &length);

    int left_list[length], right_list[length];

    for (int i = 0; i < length; i++) {
        int first, second;
        sscanf(input[i], "%d   %d", &first, &second);
        left_list[i] = first;
        right_list[i] = second;
    }
    
    int n = sizeof(left_list) / sizeof(left_list[0]);

    qsort(left_list, n, sizeof(int), comp);
    qsort(right_list, n, sizeof(int), comp);

    int total_distance = 0;

    for (int i = 0; i < length; i++) {
        int distance = abs(left_list[i] - right_list[i]); 
        total_distance += distance;
    }

    printf("Part 1: %d\n", total_distance);

    int similarity_score = 0;

    for (int i = 0; i < length; i++) {
        int target = left_list[i];
        int occurrences_right = 0;
        for (int j = 0; j < length; j++) {
            if (right_list[j] == target) {
                occurrences_right += 1;
            }
        }
        similarity_score += target * occurrences_right;
    }

    printf("Part 2: %d\n", similarity_score);
    
    return 0;
}


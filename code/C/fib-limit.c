#include <stdio.h>

int a = 0, b = 1, i = 0;
int limit = 2;

int main() {
    while (1) {
        a += b;
        printf(a, "\n");
        i += 1;
        b += a;
        printf(b, "\n");
        i += 1;
        if (i == limit) {
            break;
        }
    }
}
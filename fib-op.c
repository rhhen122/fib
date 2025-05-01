#include <stdio.h>

int a = 0, b = 1;

int main() {
    while (1) {
        a += b;
        printf("%d", a);
        b += a;
        printf("%d", b);
    }
}
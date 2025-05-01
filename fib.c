#include <stdio.h>

int a = 0, b = 1, o;

int main() {
    while (1) {
        printf("%d", a);
        o = a + b;
        a = b;
        b = o;
    }
}
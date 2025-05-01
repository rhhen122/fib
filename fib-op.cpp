#include <iostream>
using namespace std;

int a = 0, b = 1;

int main() {
    while (1) {
        a += b;
        printf("%d", a, "\n")
        b += a;
        printf("%d", b, "\n")
    }
}
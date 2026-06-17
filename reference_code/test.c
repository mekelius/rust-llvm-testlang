#include <stdio.h>

int test(int x) {
    int y = x * 2;
    return y;
}

int main() {
    int x;
    scanf("%d", &x);
    printf("%d\n", x);
}
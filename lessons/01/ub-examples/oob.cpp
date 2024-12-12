#include <iostream>

int in_array(int v) {
    int table[4] = {5, 13, 8, 12};

    for (int i = 0; i <= 4; i++) {
        if (table[i] == v) return 1;
    }
    return 0;
}

int main() {
    int result = in_array(50);
    std::cout << result << std::endl;

    return 0;
}

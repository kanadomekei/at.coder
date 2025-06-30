// C++ と Zig の関数の比較
#include <iostream>
#include <string>
#include <vector>

// --- Zig との比較用コメント ---
// (省略)
// --------------------------

// --- 1. 基本的な関数 ---
int add(int a, int b) {
    return a + b;
}

// --- 2. 戻り値がない関数 (void) ---
void printMessage(const std::string& message) {
    std::cout << message << std::endl;
}

// --- 3. 引数の渡し方 ---
void modifyValue(int value) {
    value = 100;
    std::cout << "Inside modifyValue: " << value << std::endl;
}

void modifyReference(int& value) {
    value = 200;
    std::cout << "Inside modifyReference: " << value << std::endl;
}

void printVector(const std::vector<int>& vec) {
    std::cout << "Vector elements: ";
    for (int num : vec) {
        std::cout << num << " ";
    }
    std::cout << std::endl;
}

// --- 4. デフォルト引数 ---
void showDialog(const std::string& text, const std::string& title = "Info", bool isError = false) {
    std::cout << "-- " << title << " --" << (isError ? " [ERROR]" : "") << std::endl;
    std::cout << text << std::endl;
    std::cout << "--------------------" << std::endl;
}

// --- 5. 関数のオーバーロード ---
int findMax(int a, int b) {
    return (a > b) ? a : b;
}

double findMax(double a, double b) {
    return (a > b) ? a : b;
}

// --- 6. 再帰関数 ---
int factorial(int n) {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

int main() {
    std::cout << "=== C++ と Zig の関数の比較 ===" << std::endl;

    std::cout << "\n--- 1. 基本的な関数の呼び出し ---" << std::endl;
    int sum = add(10, 20);
    std::cout << "add(10, 20) = " << sum << std::endl;

    std::cout << "\n--- 2. void関数の呼び出し ---" << std::endl;
    printMessage("Hello from a function!");

    std::cout << "\n--- 3. 引数の渡し方の比較 ---" << std::endl;
    int original = 50;
    std::cout << "Original value before calls: " << original << std::endl;
    modifyValue(original);
    std::cout << "Original value after modifyValue: " << original << std::endl;
    modifyReference(original);
    std::cout << "Original value after modifyReference: " << original << std::endl;

    std::vector<int> myVector = {1, 2, 3, 4, 5};
    printVector(myVector);

    std::cout << "\n--- 4. デフォルト引数の使用 ---" << std::endl;
    showDialog("Operation successful.");
    showDialog("File not found.", "File System", true);

    std::cout << "\n--- 5. オーバーロードされた関数の呼び出し ---" << std::endl;
    std::cout << "Max of 10, 20 is " << findMax(10, 20) << std::endl;
    std::cout << "Max of 3.14, 2.71 is " << findMax(3.14, 2.71) << std::endl;

    std::cout << "\n--- 6. 再帰関数の呼び出し ---" << std::endl;
    std::cout << "Factorial of 5 is " << factorial(5) << std::endl;

    std::cout << "\n=== 関数の学習完了 ===" << std::endl;

    return 0;
}
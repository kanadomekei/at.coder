#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
#include <numeric>
#include <cmath>
#include <sstream>
#include <climits>

using namespace std;

// 入力処理
int readInt() {
    int n;
    cin >> n;
    return n;
}

long long readLong() {
    long long n;
    cin >> n;
    return n;
}

vector<int> readInts(int n) {
    vector<int> arr(n);
    for (int i = 0; i < n; i++) {
        cin >> arr[i];
    }
    return arr;
}

vector<long long> readLongs(int n) {
    vector<long long> arr(n);
    for (int i = 0; i < n; i++) {
        cin >> arr[i];
    }
    return arr;
}

string readString() {
    string s;
    cin >> s;
    return s;
}

vector<string> readStrings(int n) {
    vector<string> arr(n);
    for (int i = 0; i < n; i++) {
        cin >> arr[i];
    }
    return arr;
}

// 数学関数
template<typename T>
T maxVal(T a, T b) {
    return max(a, b);
}

template<typename T>
T minVal(T a, T b) {
    return min(a, b);
}

template<typename T>
T absVal(T x) {
    return abs(x);
}

long long power(long long base, long long exp) {
    long long result = 1;
    while (exp > 0) {
        if (exp % 2 == 1) {
            result *= base;
        }
        base *= base;
        exp /= 2;
    }
    return result;
}

long long modPow(long long base, long long exp, long long mod) {
    long long result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp % 2 == 1) {
            result = (result * base) % mod;
        }
        base = (base * base) % mod;
        exp /= 2;
    }
    return result;
}

long long gcd(long long a, long long b) {
    while (b != 0) {
        long long temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

long long lcm(long long a, long long b) {
    return a / gcd(a, b) * b;
}

// 配列操作
template<typename T>
void reverseVec(vector<T>& arr) {
    reverse(arr.begin(), arr.end());
}

template<typename T>
T sumVec(const vector<T>& arr) {
    return accumulate(arr.begin(), arr.end(), T(0));
}

template<typename T>
T maxVec(const vector<T>& arr) {
    return *max_element(arr.begin(), arr.end());
}

template<typename T>
T minVec(const vector<T>& arr) {
    return *min_element(arr.begin(), arr.end());
}

// 素数判定
bool isPrime(long long n) {
    if (n < 2) return false;
    if (n == 2) return true;
    if (n % 2 == 0) return false;
    
    for (long long i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return false;
    }
    return true;
}

// エラトステネスの篩
vector<bool> sieveOfEratosthenes(int n) {
    vector<bool> is_prime(n + 1, true);
    is_prime[0] = is_prime[1] = false;
    
    for (int i = 2; i * i <= n; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= n; j += i) {
                is_prime[j] = false;
            }
        }
    }
    return is_prime;
}

// 順列・組み合わせ
long long factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

long long permutation(int n, int r) {
    if (r > n || r < 0) return 0;
    long long result = 1;
    for (int i = 0; i < r; i++) {
        result *= (n - i);
    }
    return result;
}

long long combination(int n, int r) {
    if (r > n || r < 0) return 0;
    if (r > n - r) r = n - r;
    
    long long result = 1;
    for (int i = 0; i < r; i++) {
        result = result * (n - i) / (i + 1);
    }
    return result;
}

// 文字列操作
string reverseString(const string& s) {
    string result = s;
    reverse(result.begin(), result.end());
    return result;
}

bool isPalindrome(const string& s) {
    int n = s.length();
    for (int i = 0; i < n / 2; i++) {
        if (s[i] != s[n - 1 - i]) {
            return false;
        }
    }
    return true;
}

// ソート
template<typename T>
void sortVec(vector<T>& arr) {
    sort(arr.begin(), arr.end());
}

template<typename T>
void sortVecDesc(vector<T>& arr) {
    sort(arr.begin(), arr.end(), greater<T>());
}

// 二分探索
template<typename T>
int binarySearch(const vector<T>& arr, T target) {
    int left = 0, right = arr.size();
    while (left < right) {
        int mid = (left + right) / 2;
        if (arr[mid] == target) {
            return mid;
        } else if (arr[mid] < target) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    return -1;
}

// 出力処理
template<typename T>
void printVec(const vector<T>& arr) {
    for (size_t i = 0; i < arr.size(); i++) {
        if (i > 0) cout << " ";
        cout << arr[i];
    }
    cout << endl;
}

template<typename T>
void printVecLines(const vector<T>& arr) {
    for (const auto& item : arr) {
        cout << item << endl;
    }
}

// 使用例
int main() {
    // 入力例
    int n = readInt();
    vector<int> arr = readInts(n);
    
    cout << "Input: n=" << n << ", arr=";
    printVec(arr);
    
    vector<long long> arr_ll(arr.begin(), arr.end());
    cout << "Max: " << maxVec(arr_ll) << ", Min: " << minVec(arr_ll) << ", Sum: " << sumVec(arr_ll) << endl;
    cout << "GCD of first two: " << gcd(arr_ll[0], arr_ll[1]) << endl;
    cout << "Is " << n << " prime? " << (isPrime(n) ? "true" : "false") << endl;
    
    return 0;
}
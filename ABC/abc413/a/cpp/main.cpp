#include <iostream>
#include <vector>
#include <algorithm>
#include <string>
#include <map>
#include <set>
#include <queue>
#include <stack>
#include <cmath>
#include <climits>

using namespace std;

int readInt() {
    int n;
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

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    
    cout << "Hello, World!" << endl;
    
    return 0;
}
#include <algorithm>
#include <climits>
#include <cmath>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <stack>
#include <string>
#include <vector>

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

  int n, m;
  n = readInt();
  m = readInt();
  vector<int> a = readInts(n);

  int sum = 0;
  for (int i = 0; i < n; i++) {
    sum += a[i];
  }
  if (sum <= m) {
    cout << "Yes" << endl;
  } else {
    cout << "No" << endl;
  }
  return 0;
}
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

  int n = readInt();

  int count = 0;

  for (int i = 0; i < n; i++) {
    vector<int> data = readInts(2);
    if (data[1] > data[0]) {
      count++;
    }
  }

  cout << count << endl;

  return 0;
}
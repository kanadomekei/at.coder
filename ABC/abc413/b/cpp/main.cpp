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

int main() {
  ios::sync_with_stdio(false);
  cin.tie(nullptr);

  int n = readInt();

  vector<string> s = readStrings(n);

  set<string> uniqueStrings;

  for (int i = 0; i < n; i++) {
    for (int j = 0; j < n; j++) {
      if (i == j) {
        continue;
      }
      uniqueStrings.insert(s[i] + s[j]);
    }
  }

  cout << uniqueStrings.size() << endl;

  return 0;
}
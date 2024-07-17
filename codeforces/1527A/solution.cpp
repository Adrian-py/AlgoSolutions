#include <bits/stdc++.h>
 
using namespace std;

void solve() {
    int n;
    cin >> n;
    int temp = n, curr = 1;
    while (temp >= curr * 2) {
        curr <<= 1;
    }
    cout << curr - 1 << endl;
}

int main() {
    ios_base::sync_with_stdio(0);
    cin.tie(0); cout.tie(0);
    int t = 1;
    cin >> t;
    while(t--) {
        solve();
    }
} 
#include <bits/stdc++.h>
 
using namespace std;

void solve() {
    int n;
    cin >> n;
    while (n > 9) {
        int temp = n, res = 0;
        while (temp > 0) {
            res += temp % 10;
            temp /= 10;
        }
        n = res;
    }
    cout << n << endl;
}

int main() {
    ios_base::sync_with_stdio(0);
    cin.tie(0); cout.tie(0);
    int t = 1;
    // cin >> t;
    while(t--) {
        solve();
    }
} 
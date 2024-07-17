#include <bits/stdc++.h>
 
using namespace std;

void solve() {
    long long n, a, b;
    cin >> n >> a >> b;
    if (a == 1) {
        if ((n - 1) % b == 0) {
            cout << "Yes" << endl;
        } else {
            cout << "No" << endl;
        }
        return;
    }
    long long m = 1;
    while (n >= m) {
        if ((n - m) % b == 0) {
            cout << "Yes" << endl;
            return;
        }
        m *= a;
    }
    cout << "No" << endl;
}

int main() {
    ios_base::sync_with_stdio(0);
    cin.tie(0); cout.tie(0);
    int t;
    cin >> t;
    while(t--) {
        solve();
    }
} 
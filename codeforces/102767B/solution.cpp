#include <bits/stdc++.h>
 
using namespace std;

const long long MOD = 1e9 + 7;

long long binary_expo(long long n) {
    if (n == 0) {
        return 1;
    }
    long long res = binary_expo(n / 2);
    if (n % 2 == 1) {
        return (res * res * 2) % MOD;
    }
    return (res * res) % MOD;
}


void solve() {
    int n, x;
    cin >> n >> x;
    long long res = binary_expo(n);
    cout << ((res - 1) * x) % MOD << endl;
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
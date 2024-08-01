class Solution {
public:
    int countSeniors(vector<string>& details) {
        int num_of_senior = 0;
        for(int i = 0; i < details.size(); i++) {
            if ((details[i][11] - '0') * 10 + (details[i][12] - '0') > 60) {
                num_of_senior += 1;
            }
        }
        
        return num_of_senior;
    }
};
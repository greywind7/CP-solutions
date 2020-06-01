#include <bits/stdc++.h>

using namespace std;

int main()
{
    int n;
    cin >> n;

    map<int,pair<int,int> > scores;
    vector<string> names(n);
    
    for (int i = 0; i < n; i++)
    {
        int oregano, boletus, jungle, score;
        cin >> names[i];
        cin >> oregano >> boletus >> jungle;
        score = -1 * (5*oregano + 3*boletus + 15*jungle);    
        scores[score].first++;
        scores[score].second = i;
    }

    auto it = scores.begin();
    while(it->second.first != 1 && it != scores.end()) it++;
    cout << (it == scores.end() ? "all disqualified" : names[it->second.second]);
}
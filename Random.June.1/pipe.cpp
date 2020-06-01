#define pi (float) 355/113
#include<bits/stdc++.h>

using namespace std;

int main()
{
    int n;
    cin >> n;
    
    float len, dia, max_dia;
    float surface = 0.0;
    while(n--)
    {
        cin >> len >> dia >> max_dia; 
        float side = float(max_dia/2);
        float upper_area = float(3*sqrt(3)/2*side*side) - float(pi*dia*dia/4);
        printf("volume: %.3f\n",len * upper_area);
        surface += float(6*len*side) + float(2*upper_area) + float(len*pi*dia*dia/4);
    }
    printf("sum of surface areas: %.3f",surface);
}
#include <bits/stdc++.h>

using namespace std;

int main()
{
    int score = 0;
    char p1, p2;

    while (cin.peek() != EOF)
    {
        cin >> p1 >> p2;

        if (p2 == 'X')
        {
            score += 1;
            if (p1 == 'A')
            {
                score += 3;
            }
            else if (p1 == 'C')
            {
                score += 6;
            }
        }
        else if (p2 == 'Y')
        {
            score += 2;
            if (p1 == 'B')
            {
                score += 3;
            }
            else if (p1 == 'A')
            {
                score += 6;
            }
        }
        else if (p2 == 'Z')
        {
            score += 3;
            if (p1 == 'C')
            {
                score += 3;
            }
            else if (p1 == 'B')
            {
                score += 6;
            }
        }
    }

    cout << score << endl;

    return 0;
}

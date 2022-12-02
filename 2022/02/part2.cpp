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
            if (p1 == 'A')
            {
                score += 3;
            }
            else if (p1 == 'B')
            {
                score += 1;
            }
            else
            {
                score += 2;
            }
        }
        else if (p2 == 'Y')
        {
            score += 3;
            if (p1 == 'A')
            {
                score += 1;
            }
            else if (p1 == 'B')
            {
                score += 2;
            }
            else
            {
                score += 3;
            }
        }
        else if (p2 == 'Z')
        {
            score += 6;
            if (p1 == 'A')
            {
                score += 2;
            }
            else if (p1 == 'B')
            {
                score += 3;
            }
            else
            {
                score += 1;
            }
        }
    }

    cout << score << endl;

    return 0;
}

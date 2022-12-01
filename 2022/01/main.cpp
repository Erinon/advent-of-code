#include <bits/stdc++.h>
#include <fstream>

using namespace std;

int vec_min_index(vector<unsigned int> vec)
{
    if (vec.empty())
    {
        return -1;
    }

    int min_index = 0, min_element = vec[0];

    for (int i = 1; i < vec.size(); ++i)
    {
        if (vec[i] < min_element)
        {
            min_index = i;
            min_element = vec[i];
        }
    }

    return min_index;
}

int vec_max(vector<unsigned int> vec)
{
    if (vec.empty())
    {
        return -1;
    }

    int max_element = 0;

    for (int e : vec)
    {
        if (e > max_element)
        {
            max_element = e;
        }
    }

    return max_element;
}

int vec_sum(vector<unsigned int> vec)
{
    if (vec.empty())
    {
        return -1;
    }

    int sum = 0;

    for (int e : vec)
    {
        sum += e;
    }

    return sum;
}

vector<string> read_file(string fname)
{
    string line;
    vector<string> lines;

    fstream infile(fname);

    while (getline(infile, line))
    {
        lines.push_back(line);
    }

    infile.close();

    return lines;
}

int main()
{
    unsigned int current_calories;
    vector<unsigned int> max_calories(3, 0);

    vector<string> lines = read_file("input.txt");

    current_calories = 0;

    for (string line : lines)
    {
        if (line.empty())
        {
            current_calories = 0;
        }
        else
        {
            current_calories += stoi(line);

            int min_index = vec_min_index(max_calories);
            if (current_calories > max_calories[min_index])
            {
                max_calories[min_index] = current_calories;
            }
        }
    }

    cout << vec_max(max_calories) << endl
         << vec_sum(max_calories) << endl;

    return 0;
}

#include <iostream>
#include <vector>

using std::vector;

int iter(vector<int> arr, int value)
{
    int start = 0;
    int end = (int) arr.size() - 1;
    
    while (start <= end)
    {
        int middle = (int) ((end - start) / 2) + start;

        if (value == arr[middle])
            return middle;
        if (value > arr[middle])
            start = middle + 1;
        else
            end = middle - 1;
    }

    return -1;    
}

int recursive(vector<int> arr, int value, int start = 0, int end = -2)
{
    if (end == -2)
        end = arr.size() - 1;
    if (start > end)
        return -1;

    int middle = (int) ((end - start) / 2) + start;

    if (value == arr[middle])
        return middle;
    if (value > arr[middle])
        return recursive(arr, value, middle + 1, end);
    return recursive(arr, value, start, middle - 1);
    
}

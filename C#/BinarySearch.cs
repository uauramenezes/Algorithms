class BinarySearch
{
    public static int iter(int[] arr, int value)
    {
        int start = 0;
        int end = arr.Length - 1;

        while (start < end) 
        {
            int middle = (end - start / 2) + start;

            if (value == arr[middle]) 
                return middle;

            if (value > arr[middle]) 
                start = middle + 1;
            else
                end = middle - 1;
            
        }

        return -1;            
    }

    public static int recursive(
        int[] arr, int value, int start = 0, int end = -1)
    {
        if (end < 0) end = arr.Length - 1;
        if (start > end) return -1;
        
        int middle = ((end - start) / 2) + start;

        if (value == arr[middle])
            return middle;
        if (value > arr[middle])
            return recursive(arr, value, middle + 1, end);
        return recursive(arr, value, start, middle - 1);
    }
}
# Time Complexity: O(n²)
# Space Complexity: O(1) 

def insertion_sort(arr):
    for i in range(1, len(arr)):
        j = i
        while j > 0 and arr[j-1] > arr[j]:
            arr[j], arr[j-1] = arr[j-1], arr[j]
            j -= 1
        

arr = [10,8,4, 7, 2, 5, 3, 6, 9, 1]
insertion_sort(arr)
print(arr)
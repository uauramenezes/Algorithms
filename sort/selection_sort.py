def selection_sort(array):
    for i in range(len(array)):
        min_value = i
        for j in range(i+1, len(array)):
            if array[j] < array[min_value]:
                min_value = j

        if i != min_value:
            array[i], array[min_value] = array[min_value], array[i]

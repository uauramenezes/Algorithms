import math

def iter(numbers, element):
    start = 0
    end = len(numbers) - 1

    while start <= end:
        middle = math.floor((end - start) / 2) + start

        if element == numbers[middle]:
            return middle
        elif element > numbers[middle]:
            start = middle + 1
        else:
            end = middle - 1
    
    return None


def recursive(numbers, element, start=0, end=None):
    if end == None: 
        end = len(numbers) - 1
    if start > end:
        return None
    
    middle = math.floor((end - start) / 2) + start

    if element == numbers[middle]:
        return middle
    if element > numbers[middle]:
        return recursive(numbers, element, middle + 1, end)
    return recursive(numbers, element, start, middle - 1)

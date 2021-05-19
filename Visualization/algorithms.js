function bubbleSort() {
  if (i >= array.length) return true;
  if (j < array.length-i-1) {
    if (array[j] > array[j+1]) {
      play = false;
      swap(j, j+1);
    }
    j++;
    current = j;
  } else {
    j = 0;
    i++;
  }
}

function selectionSort() {
  if (i >= array.length) return true;
  if (j === i + 1) minVal = i;
  if (j < array.length) {
    if (array[j] < array[minVal]) {
      minVal = j;
    }
    current = j;
    j++;
  } else {
    current = i;
    play = false;
    swap(i, minVal)
    j = i + 1;
    i++;
  }
}

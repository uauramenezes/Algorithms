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
  if (j === i + 1) selectedValue = i;
  if (j < array.length) {
    if (array[j] < array[selectedValue]) {
      selectedValue = j;
    }
    current = j;
    j++;
  } else {
    current = i;
    play = false;
    swap(i, selectedValue)
    j = i + 1;
    i++;
  }
}

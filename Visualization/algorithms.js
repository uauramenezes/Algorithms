function bubbleSort() {
  if (i<array.length) {
    if (j<array.length-i-1) {
      if (array[j] > array[j+1]) {
        swap(j, j+1);
      }
      j++;
      current = j;
    }
    else {
      j=0;
      i++;
    }
  }
  else {
    done = true;
    playBtn.textContent = 'Reset';
  }
}

function selectionSort() {
  if (i<array.length) {
    if (j === i + 1) minVal = i;
    if (j<array.length) {
      if (array[j] < array[minVal]) {
        minVal = j;
      }
      current = j;
      j++;
    } else {
      swap(i, minVal)
      i++;
      j=i+1;
    }
  } else {
    done = true;
    playBtn.textContent = 'Reset';
  }
}

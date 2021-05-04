function iter(array, element) {
  let start = 0;
  let end = array.length - 1;

  while (start <= end) {
    let middle = Math.floor((end - start) / 2) + start;

    if (element === array[middle]) {
      return middle;
    } else if (element > array[middle]) {
      start = middle + 1;
    } else {
      end = middle - 1;
    }
  }

  return -1;
}

function recursive(array, element, start=0, end=undefined) {
  if (end == undefined) end = array.length - 1;

  let middle = Math.floor((end - start) / 2) + start;

  if (start > end) return -1;
  if (element === array[middle]) return middle;
  
  if (element > array[middle]) {
    return recursive(array, element, middle + 1, end);
  } else {
    return recursive(array, element, start, middle - 1);
  }
}

module.exports = {iter, recursive}

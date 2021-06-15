pub fn binary_search(numbers: &Vec<i32>, element: i32) -> Option<usize> {
    let mut start = 0;
    let mut end = numbers.len();

    while start < end {
        let middle = (end - start) / 2 + start;

        if element == numbers[middle] {
            return Some(middle);
        } else if element > numbers[middle] {
            start = middle + 1;
        } else {
            end = middle - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search() {
        let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let index = binary_search(&vec, 1);
        assert_eq!(None, index);
    }
}

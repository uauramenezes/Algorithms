pub fn selection_sort<T: Ord>(vec: &mut [T]) {
    for i in 0..vec.len() {
        let mut selected_index = i;
        for j in i + 1..vec.len() {
            if vec[j] < vec[selected_index] {
                selected_index = j;
            }
        }
        vec.swap(i, selected_index);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sort() {
        let mut vec = vec![1, 5, 3, 2, 8, 7, 4, 6, 9, 0];
        let sorted_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        selection_sort(&mut vec);
        assert_eq!(vec, sorted_vec);
    }
}

pub fn insertion_sort<T: Ord>(vec: &mut [T]) {
    for i in 1..vec.len() {
        let mut j = i;
        while j > 0 && vec[j - 1] > vec[j] {
            vec.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sort() {
        let mut vec = vec![1, 5, 3, 2, 8, 7, 4, 6, 9, 0];
        let sorted_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        insertion_sort(&mut vec);
        assert_eq!(vec, sorted_vec);
    }
}

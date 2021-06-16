use std::cmp::Ordering;

pub fn binary_search<T: Ord>(items: &[T], element: T) -> Option<usize> {
    let mut start = 0;
    let mut end = items.len();

    while start < end {
        let middle = (end - start) / 2 + start;

        // Another solution using pattern matching found on the internet
        // https://the-algorithms.com/algorithm/binary-search
        // For an article discussing this solution
        // https://shane-o.dev/blog/binary-search-rust-part-2
        match items[middle].cmp(&element) {
            Ordering::Equal => return Some(middle),
            Ordering::Less => start = middle + 1,
            Ordering::Greater => end = middle,
        }

        // Initial solution
        // if element == items[middle] {
        //     return Some(middle);
        // } else if element > items[middle] {
        //     start = middle + 1;
        // } else {
        //     end = middle - 1;
        // }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let index = binary_search(&vec![], 5);
        assert_eq!(None, index);
    }

    #[test]
    fn search_one() {
        let index = binary_search(&vec![5], 5);
        assert_eq!(Some(0), index);
    }

    #[test]
    fn search_int() {
        let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let index = binary_search(&vec, 7);
        assert_eq!(Some(7), index);
    }
    #[test]
    fn search_char() {
        let vec = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let index = binary_search(&vec, 'd');
        assert_eq!(Some(3), index);
    }

    #[test]
    fn search_string() {
        let vec = vec!["Anna", "Jean", "Louise"];
        let index = binary_search(&vec, "Jean");
        assert_eq!(Some(1), index);
    }

    #[test]
    fn not_found() {
        let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let index = binary_search(&vec, 11);
        assert_eq!(None, index);
    }
}

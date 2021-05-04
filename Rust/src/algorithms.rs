pub fn binary_search(vec: &Vec<i32>, element: i32) -> i32 {
    let mut start: i32 = 0;
    let mut end: i32 = (vec.len() - 1) as i32 ;

    while start <= end {
        let middle: i32 = ((end - start) / 2) + start;
        let i = middle as usize;

        if element == vec[i] {
            return middle;
        } else if element > vec[i] {
            start = middle + 1;
        } else {
            end = middle - 1;
        }
    }

    return -1;
}

pub fn iter(vec: &Vec<i32>, element: i32) -> i32 {
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

pub fn recursive(vec: &Vec<i32>, element: i32) -> i32 {
    let start: i32 = 0;
    let end = (vec.len() - 1) as i32;
    
    fn recursion(v: &Vec<i32>, el: i32, s: i32, e: i32) -> i32 {
        if s > e {
            return -1
        }

        let middle: i32 = ((e - s) / 2) + s;
        let i = middle as usize;

        if el == v[i] {
            return middle;
        } else if el > v[i] {
            return recursion(v, el, middle + 1, e);
        } else {
            return recursion(v, el, s, middle - 1);
        }
    }

    return recursion(vec, element, start, end);
}

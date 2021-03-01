struct P {
    l: i32,
    r: i32,
}

fn main() {
    let x = 4;

    //let vec = vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    let vec = vec!(1, 3, 5, 7, 9);
    //let vec = vec!(2, 4, 6, 8, 10);

    let pointer = P{l: 0, r: vec.len() as i32};

    match search(&vec, x, pointer) {
        -1 => println!("Element Not Found!"),
        i => println!("The index of {} is {}", x, i)
    };
}

fn search(vec: &Vec<i32>, x: i32, mut p: P) -> i32 {
    let middle = ((p.l + p.r) / 2).abs();
    let i = middle as usize;

    if middle < 0 || i >= vec.len() {
        -1
    } else if vec[i] == x {
        middle
    } else if p.l >= p.r {
        - 1
    } else if vec[i] > x {
        p.r = middle - 1;
        search(vec, x, p)
    } else{
        p.l = middle + 1;
        search(vec, x, p)
    }
}

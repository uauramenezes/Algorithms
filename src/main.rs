struct P {
    l: usize,
    r: usize,
}

fn main() {
    let x = 33;

    let mut vec = Vec::new();
    for n in 1..=100 {
        vec.push(n);
    }

    let pointer = P{l: 0, r: vec.len() - 1};
    let index = bts(&vec, x, pointer);

    println!("The index of {} is {}", x, index);
}

fn bts(vec: &Vec<i32>, x: i32, mut p: P) -> usize {
    let i = (((p.l + p.r) as i32 / 2).abs()) as usize;

    if vec[i] == x {
        i
    } else if vec[i] > x {
        p.r = i - 1;
        bts(vec, x, p)
    } else{
        p.l = i + 1;
       bts(vec, x, p)
    }
}

mod algorithms;

fn main() {
    let x = 10;

    let vec = vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    // let vec = vec!(1, 3, 5, 7, 9);
    // let vec = vec!(2, 4, 6, 8, 10);

    match algorithms::binary_search(&vec, x,) {
        -1 => println!("Element Not Found!"),
        i => println!("The index of {} is {}", x, i)
    };
}

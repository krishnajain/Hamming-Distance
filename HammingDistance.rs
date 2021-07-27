pub fn hamming_distance(x: i32, y: i32) -> i32 {
    let mut counter = 0;
    let mut z = x ^ y;
    while z != 0 {
        counter += z & 1;
        z >>= 1;
    }
    counter
}

fn main() {
    let hd = hamming_distance(1, 4);
    println!("{}", hd);

    let hd = hamming_distance(9, 14);
    println!("{}", hd);
}
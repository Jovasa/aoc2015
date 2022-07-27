fn main() {
    let mut value = 20151125u64;
    let mut x = 1;
    let mut y = 1;
    for i in 2.. {
        value *= 252533;
        value %= 33554393;
        y -= 1;
        x += 1;
        if y == 0 {
            y = x;
            x = 1;
        }
        if x ==  3083 && y == 2978{
            break
        }
    }
    println!("{}", value);
}
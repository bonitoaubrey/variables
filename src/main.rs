const MAX: u64 = 33;

fn main() {
    let mut x: u16 = 65535;
    println!("The value of x is: {}", x);
    x -= 4;
    println!("The value of x is: {}", x);
    x = 0;
    println!("The value of x is: {}", MAX);
}

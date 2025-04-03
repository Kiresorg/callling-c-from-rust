// Declare external C functions using C ABI
extern "C" {
    fn triple(n: i32) -> i32;
    fn square(n: i32) -> i32;
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    let input = 7;

    // Call each C function using unsafe blocks
    let tripled = unsafe { triple(input) };
    println!("Triple of {} is {}", input, tripled);

    let squared = unsafe { square(input) };
    println!("Square of {} is {}", input, squared);

    let sum = unsafe { add(4, 9) };
    println!("Sum of 4 and 9 is {}", sum);
}

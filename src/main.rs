// Declare the external C function
extern "C" {
    fn triple(n: i32) -> i32;
}

fn main() {
    let input = 7;

    // Call the unsafe C function
    let result = unsafe { triple(input) };

    println!("Triple of {} is {}", input, result);
}

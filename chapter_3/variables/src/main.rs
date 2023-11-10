fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x after mutation is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("const value is {THREE_HOURS_IN_SECONDS}");

    let x = x + 1;
    println!("The value of x after shadowing is: {x}");

    let spaces = "    ";
    println!("The type of {spaces} is string");
    let spaces = spaces.len();
    println!("The type of spaces value: {spaces} after shadowing is different");

    let guess: u32 = match "44".trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    println!("{guess}");
    let c = 'c';
    println!("{c}");

    let f = true;
    let d: bool = false; // with explicit type annotation
    println!("Booleans are {f} and {d}");

    let x = 3.0;
    let y: f32 = 2.0;

    println!("Floats are {x} and {y}");

    let _tup: (u8, f64, i32) = (1, 6.5, 500);
    let tup = (1, 6.5, 500);
    let (x, y, z) = tup;
    println!("Value of x is {x}, y is {y}, z is {z}");
    let first_index = tup.0;
    println!("{first_index}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let first_index = tup.0;
    println!("Tuple is {:?}", tup);
    println!("First index is: {first_index}");

    let mut arr = [1, 3, 4, 5, 9];
    let first = arr[0];
    println!("First element of array is: {first}");

    arr = [2, 2, 4, 5, 7];
    println!("First element of array is: {:?}", arr);
}



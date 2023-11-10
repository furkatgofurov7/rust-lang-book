fn main() {
    another_func(5, 'e');

    let y = {
        let x = 3;
        x+1
    };
    println!("The value of y is {y}");

    let x: i32 = five();
    println!("The value of five fn is {:?}", x);

    let z = plus_one(1);
    println!("The value of z is {z}")

}

fn another_func(x: i32, label: char) {
    println!("Another function prints: {x} and {label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x+1
}



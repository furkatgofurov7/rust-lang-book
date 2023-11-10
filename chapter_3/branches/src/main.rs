fn main() {
    let num: i32 = 6;

    if num > 5 {
        println!("Condition was false")
    } else {
        println!("Condition was true")
    }

    if num != 0 || num > 2 {
        println!("It is a number either greater then 2 OR is positive")
    }

    if num % 4 == 0 {
        println!("number is divisible by 4");
    } else if num % 3 == 0 {
        println!("number is divisible by 3");
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    let condition: bool = true;
    let num = if condition {
        5
    } else {
        6
    };
    println!("The value of the number is {num}");

    loop_forever();

    boollean();

    sum_func();

    cel_to_fahr(20, 5);

    let fib_result = fib_nth(100);
    println!("fib result is {fib_result}");

    song()
}

fn loop_forever() {
    let x = 10;
    let mut i = 0;
    loop {
        i += 1;
        if i > x {
            println!("Breaking a loop!");
            break;
        } else if i % 2 == 0 {
            continue;
        } else {
            println!("Loop forever!");
        }
    }
    let mut counter = 1;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter*2;
        }
    };
    println!("The result is {result}");

    inner_loop();

    while_loop();

    for_loop();

    original_for_loop();
    
    reverse_order()
}


fn inner_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn original_for_loop() {
    let a = [10, 20, 33, 44, 55, 9];

    for element in a {
        println!("Element in a is {element}")
    }
}

fn reverse_order() {
    for num in (1..4).rev() {
        println!("{num}!");
    }
    println!("LIFTOFF!!!")
}

fn boollean() {
    let mut x = 0;
    'a: loop {
        println!("inside 'a loop");
        x += 1;
        println!("x is: {x}");
        'b: loop {
            if x > 10 {
                println!("inside x>10, and x is {x}");
                continue 'a;
            } else {
                println!("inside else x>10 and x is {x}");
                break 'b;
            }      
        }
        break;       
    }
}

fn sum_func() {
    let a = [5; 10];
    let mut sum = 0;
    for x in a {
        println!("x is {x}");
        sum += x;
        println!("sum is {sum}");
    }
    println!("{sum}");
}


/*

Extra exercises to complete at the end of the chapter

*/ 

// Celcius to Fahrenheit and vice versa converter
fn cel_to_fahr(celciuc: i32, fahrenheit: i32) {
    let cel_to_fahr = (celciuc * 9/5) + 32;
    let fahr_to_cel = (fahrenheit - 32) * 5/9;

    println!("{celciuc} celcius in fahrenheit is {cel_to_fahr}");
    println!("{fahrenheit} fahrenheit in celcius is {fahr_to_cel}")
}

// Generate the nth Fibonacci number.
fn fib_nth(fnum: u8) -> u128 {
    if fnum == 0 {
        println!("too low, fibonacci argument should be at least 1");
    } else if fnum == 1 {
        println!("1th Fibonacci number is: {fnum}");
    }
    
    let mut sum = 0;
    let mut first = 0;
    let mut current = 1;

    for _i in 1..fnum {
        sum = first + current;
        first = current;
        current = sum;
    }
    println!("{fnum}th Fibonacci number is: {sum}");
    sum
}

// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
fn song() {
    const inital_satires: &str = "
On the first day of Christmas my true love sent to me
A partridge in a pear tree

On the second day of Christmas my true love sent to me
Two turtle doves,
And a partridge in a pear tree.

On the third day of Christmas my true love sent to me
Three French hens,
Two turtle doves,
And a partridge in a pear tree.";

    let verses = ["Four calling birds", "Five gold rings", "Six geese a-laying", "Seven swans a-swimming",
        "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];
    
    let numbers = ["fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    println!("{inital_satires}");
    for (num, verse) in numbers.iter( ).zip(verses.iter()) {
        println!("
On the {} day of Christmas my true love sent to me
{},
Two turtle doves,
And a partridge in a pear tree.", num, verse)
    }
}

# Important notes: "Common Programming Concepts"

## Vars and mutability

- By default, variables are immutable and defined using `let` keyword:

    ```rust
    let x = 5;

    ```

- Although variables are immutable by default, you can make them mutable by adding `mut` in front of the variable name:

    ```rust
    let mut x = 6;

    ```

### Constants

- Constants are values that are bound to a name and are not allowed to change
- `mut` is not allowed to be used with constants. Constants aren’t just immutable by default — they’re always immutable!
- Declared using the `const` keyword instead of the `let` keyword, and the type of the value must be **annotated**, i.e `: u32`:

    ```rust
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    ```

- Naming convention for constants is to use all uppercase with underscores between words (*UPPER_SNAKE_CASE*)

### Shadowing

- declaring a new variable with the same name as an already existing variable

    ```rust
    fn main() {
        let x = 5;

        let x = x + 1;

        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {x}");
        }

        println!("The value of x is: {x}");
    }


    ```

## Data Types

Two datatype subsets:

1. Scalar
2. Compound

### Scalar Types

-  Rust has four primary scalar types: `integers`, `floating-point numbers`, `Booleans`, and `characters`.

#### Integer Types

- two types: 
    - `signed(i)` ==> can be negative/positive number in range: -(2^n-1) - 1 <-> (2^n-1) - 1, i.e i8 can store numbers from -(2^7) to 2^7 - 1, so -128 to 127
    - & `unsigned(ui)` => only ever be positive number in range: 2^n - 1, i.e u8 can store numbers from 0 to 2^8 - 1, so 0 to 255

#### Floating-Point Types

- two types: `f32` (32 bits) & `f64` (64 bits in size)

#### The Boolean Type

- two types: `true` & `false`

#### The Character Type

- `char` literals with **single quotes** (let c = 'z'; let heart_eyed_cat = '😻';), as opposed to `string` literals, which use **double quotes**.


### Compound Types

-  Rust has 2 primary Compound types: `tuples` and `arrays`.

#### Tuples

- way of grouping together a number of values with a variety of types into one compound type
- fixed length: once declared, they cannot grow or shrink in size.
- access index of tuple with period (.):

    ```rust
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let first_index = tup.0;
    println!("First index is: {first_index}")

    ```

#### Arrays

- every element of an array must have the same type.
- fixed length
- access index of array using index inside []:

    ```rust
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a = [3; 5]; // same as: let a = [3, 3, 3, 3, 3];
    let first = a[0];
    ```

## Functions

- A function must declare the types of its parameters.
- Rust code uses snake case as the conventional style for function and variable names
- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value.
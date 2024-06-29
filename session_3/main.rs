fn main(){
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // Strings are immutable
    let s = "hello";
    println!("The value of s is: {}", s);
    // Tuples
    let tup = (1, 2, 3);
    let (a, b, c) = tup;
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    println!("The value of c is: {}", c);
    // Arrays
    let arr = [1, 2, 3, 4, 5];
    println!("The value of arr[0] is: {}", arr[0]);
    println!("The value of arr[1] is: {}", arr[1]);
    println!("The value of arr[2] is: {}", arr[2]);
    println!("The value of arr[3] is: {}", arr[3]);
    println!("The value of arr[4] is: {}", arr[4]);
    // Tuples and arrays are stack allocated
    let arr = [1, 2, 3, 4, 5];
    let tup = (1, 2, 3);
    println!("The value of arr is: {:?}", arr);
    println!("The value of tup is: {:?}", tup);
    // loops
    let mut x = 0;
    loop {
        x += 1;
        println!("The value of x is: {}", x);
        if x == 5 {
            break;
        }
    }
    // while loop
    let mut x = 0;
    while x < 5 {
        x += 1;
        println!("The value of x is: {}", x);
    }
    // for loop
    for x in 0..5 {
        println!("The value of x is: {}", x);
    }
    // functions
    
    // if statements
    let x = 5;
    if x < 10 {
        println!("The value of x is less than 10");
    } else {
        println!("The value of x is greater than or equal to 10");
    }
    // match statements
    let x = 5;
    match x {
        1 => println!("The value of x is 1"),
        2 => println!("The value of x is 2"),
        3 => println!("The value of x is 3"),
        _ => println!("The value of x is not 1, 2, or 3"),
    }
    // enums
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    let direction = Direction::Up;
    match direction {
        Direction::Up => println!("The direction is up"),
        Direction::Down => println!("The direction is down"),
        Direction::Left => println!("The direction is left"),
        Direction::Right => println!("The direction is right"),
    }
}

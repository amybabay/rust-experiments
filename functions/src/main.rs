fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurements(5, 'h');

    let x = five();
    println!("the value of x is: {}", x);

    println!("x + 1 = {}", plus_one(x));
}

fn another_function(x: i32) {
    println!("the value of x is: {}", x);
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurment is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

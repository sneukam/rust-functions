/*
Program that plays with functions.
Follwing The Book (rust book), chapter 3.
*/


fn main() {
    println!("Hello, world!");

    // prints the values of the arguments
    another_function(5, 'Y');

    // prints the result of five()
    let five_result: i32 = five();
    println!("function five() = {five_result}");
}

#[doc="Print the values aof x and h arguments."]
fn another_function(x: i32, h: char) {
    println!("another_function()");
    println!("x = {x}, h = {h}");
}

#[doc="Return i32: 5"]
fn five() -> i32 {
    5
}

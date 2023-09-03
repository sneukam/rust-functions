fn main() {
    println!("Hello, world!");

    another_function(5, 'Y');

    let five_result: i32 = five();
    println!("function five() = {five_result}");
}

fn another_function(x: i32, h: char) {
    println!("another_function()");
    println!("x = {x}, h = {h}");
}

fn five() -> i32 {
    5
}

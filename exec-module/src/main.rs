use lib1_module;
use lib2_module;

fn main() {
    let num1 = 10;
    let num2 = 5;
    println!("Result: {}", lib1_module::add(num1, num2));
    let num3 = 13;
    let num4 = 10;
    println!("Result: {}", lib2_module::sub(num3, num4));
}

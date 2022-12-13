use crate::input::read_input;

mod input;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("the value of x is {x}");
    x = 6;
    println!("the value of x is {x} and {}", THREE_HOURS_IN_SECONDS);


    // datatype

    let guess: u32 = "4".parse().expect("Not a number!");

    let n_users: i32 = 1_000_000;

    println!("{guess} {n_users}");

    // tuples
    let tup = (12, "hello", 12.2);
    let (age, msg, amount) = tup;

    println!("{:?} {age} {msg} {amount}", tup);

    // Arrays

    let arr: [i8; 8] = [1, 2, 3, 8, 5, 3, 4, 3];
    let arr2 = [3; 12];
    println!("{:?} {:?}", arr, arr2);

    println!("{}", read_input());
}

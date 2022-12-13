fn main() {
    let reference_to_nothing = dangle_solved();
}

// fn dangle() -> &String{
//     let s = String::from("hello");
//     &s
// }

fn dangle_solved() -> String{
    let s = String::from("hello");
    s
}

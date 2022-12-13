fn main() {
    let mut count = 0;

    'counting_loo: loop {
        println!("count {}", count);
        println!("Hello, world!");

        let mut remainder = 10;

        loop {
            println!("remainder {}", remainder);

            if remainder == 9 {
                break;
            }

            if count == 2 {
                break 'counting_loo;
            }
            remainder -= 1;
        }
        count += 1;
    }
    println!("End count {}", count);

    let numbers = [10, 20, 30, 40, 50];

    for elem in numbers {
        print!("{elem} ");
    }
    println!("");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

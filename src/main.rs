// use std::io::{self, Write}; // io là module phục vụ in/out
// use std::fs; // fs là module làm việc với file
use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn main() {
    // let b = String::from("hello");
    // let b2 = b.clone();
    // println!("{}", b2);
    // print!("{}", b)
    // print!("hello");
    // io::stdout().flush().unwrap();

    // let mut name = String::new();
    // io::stdin().read_line(&mut name).unwrap(); // unwrap để lấy giá trị, lỗi sẽ trả called
    //`Result::unwrap()` on an `Err` value: Os {
    //     code: 2,
    //     kind: NotFound,
    //     message: "No such file or directory"
    // }
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("faild to read line"); // expect: value || "msg" err

    // println!("hello wewewew {}", name.trim());

    // let content = fs::read_to_string("/home/bss-group/Documents/InlineStack gap 100 align.txt")
    //     .expect("ERR READ FILE");
    // println!("{}", content.trim());

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Faild type");
                continue;
            }
        };
        let mut rng = rand::rng();
        let secret_number = rng.random_range(1..=100);

        println!("guess: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    // let a = 6;
    // let b = 10;
    // let result = a.cmp(&b);
    // match result {
    //     Ordering::Less => println!("Too small!"),  // Ordering: enum biểu diễn phép so sánh
    //     Ordering::Greater => println!("Too big!"),
    //     Ordering::Equal => println!("You win!"),
    // }
}

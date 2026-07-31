// use std::io::{self, Write}; // io là module phục vụ in/out
// use std::fs; // fs là module làm việc với file
// use rand::RngExt;
// use std::cmp::Ordering;
// use std::io;

// #[derive(Debug)]
// struct User {
//     active: bool,
//     name: String,
//     email: String,
//     sign_in_account: u64,
// }

// #[derive(Debug)] // allowed println!("{:?}")
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//     fn width(&self) -> bool {
//         self.width > 0
//     }
// }

fn main() {
    // let mut b = String::from("hello");
    // let b2 = b.clone();
    // println!("{}", b2);
    // b.push_str("\nworld");
    // println!("hello {b}");
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

    // 'arr_match: loop {
    //     let mut numbers: String = String::new();
    //     // // let arr_value = [10, 20, 30]

    //     io::stdin()
    //         .read_line(&mut numbers)
    //         .expect("Faild to read line");

    //     let arr: Vec<i32> = numbers
    //         .split_whitespace()
    //         .map(|x| x.parse().expect("Please input num"))
    //         .collect();

    //     let mut val_index: String = String::new();

    //     io::stdin()
    //         .read_line(&mut val_index)
    //         .expect("faild to types val_index");

    //     let index: usize = match val_index.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => {
    //             println!("Faild type index");
    //             continue 'arr_match;
    //         }
    //     };

    //     match arr.get(index) {
    //         Some(value) => println!("data: {value}"),
    //         None => println!("ERR"),
    //     }

    //     let mut rng = rand::rng();
    //     let secret_number = rng.random_range(1..=100);

    //     demo_fn(index, 's');
    //     match index.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break 'arr_match;
    //         }
    //     }
    // }

    // let a = 6;
    // let b = 10;
    // let result = a.cmp(&b);

    // if a % b > 0 {
    //     println!("{a}")
    // } else {
    //     print!("hello")
    // }
    // match result {
    //     Ordering::Less => println!("Too small!"), // Ordering: enum biểu diễn phép so sánh
    //     Ordering::Greater => println!("Too big!"),
    //     Ordering::Equal => println!("You win!"),
    // }

    // for number in (1..4).rev() {
    //     println!("{number}")
    // }

    // let mut user: User = build_user(true, String::from("AKA"), String::from("ass"), 1);
    // user.name = String::from("test");

    // let user2 = User {
    //     email: String::from("emaiul"),
    //     ..user
    // };
    // println!("{:?}", user2);

    // let scale = 2;
    // let rect1 = Rectangle {
    //     width: dbg!(30 * scale),
    //     height: 50,
    // };

    // dbg!(&rect1);

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     rect1.area()
    // );

    // if rect1.width() {
    //     println!("The rectangle has a nonzero width; it is {}", rect1.width);
    // }
}

// fn demo_fn(value: usize, unit_val: char) -> (usize, char) {
//     println!("{value}, {unit_val}");
//     (value, unit_val)
// }

// fn build_user(active: bool, name: String, email: String, sign_in_account: u64) -> User {
//     User {
//         active,
//         name,
//         email,
//         sign_in_account,
//     }
// }

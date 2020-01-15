use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let a = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("输入一个数!");
        let mut aa = String::new();
        let b = io::stdin().read_line(&mut aa).expect("输入错误");
        let aa: i32 = match aa.trim().parse() {
            Ok(a) => a,
            Err(_) => {
                println!("请输入数字");
                continue;
            }
        };
        println!("你输入的是：{}", aa);

        match aa.cmp(&a) {
            Ordering::Equal => {
                println!("正确");
                break;
            }
            Ordering::Greater => println!("大了"),
            Ordering::Less => println!("小了"),
        }
    }
}

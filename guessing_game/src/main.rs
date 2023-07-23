use rand::Rng;
use std::io;

fn main() {
    // println! 宏实际上是调用了 std::io::stdout().lock().write_fmt(args).unwrap() 进行输出
    // _print 函数 -> print! 宏 -> println! 宏
    // rust 中 ! 为宏的标识符
    println!("Guess the number!");

    // rand::thread_rng() 函数返回一个随机数生成器
    // thread_rng 的意思是线程局部的随机数生成器 random number generator
    // gen_range() 方法生成一个指定范围内的随机数
    // 它是一个特定线程的本地变量
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // 声明一个 mut 变量 guess，类型为 String
        // 调用 String 类的构造函数 new() 创建一个新的 String 对象
        // 将其赋值给 guess 变量
        let mut guess = String::new();

        // 调用 io::stdin()
        // read_line() 方法读取用户输入绑定到变量 guess 上
        // expect() 方法用于错误处理，处理 io::Result 类型的错误，如果出错则打印出错误信息
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // [ 改造前 ]
        // 将 guess 变量从 String 类型转换为 i32 类型
        // trim() 方法用于去除字符串首尾的空白字符
        // parse() 方法将字符串解析为数字
        // expect() 方法用于错误处理，处理 Result 类型的错误，如果出错则打印出错误信息
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // [ 改造后 ]
        // 使用 match 表达式处理 Result 类型的错误
        // Ok(num) 匹配成功时返回 num
        // Err(_) 匹配失败时返回 _，_ 为通配符
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // 使用 match 表达式处理 Ordering 类型的值
        // cmp() 方法比较两个值并返回一个 Ordering 类型的值
        // Ordering 类型有三个值：Less、Greater、Equal
        // match 表达式会将 guess.cmp(&secret_number) 的返回值与三个分支进行比较
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

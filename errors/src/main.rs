use std::error;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};

// Box<dyn error::Error> 表示任何实现了 Error trait 的类型, dyn 表示动态分发, 也就是说, 任何实现了 Error trait 的类型都可以作为 Box 的值
// 可以理解为 go 中的 error 接口
fn main() -> Result<(), Box<dyn error::Error>> {
    // rust 中错误分为两类
    // 1. 可恢复错误, 用 Result<T, E> 表示
    // 2. 不可恢复错误, 用 panic! 宏表示

    // 环境变量中设置 RUST_BACKTRACE=full 可以打印完整的调用栈信息, 默认是打印简略信息
    // panic!("crash and burn")

    let v = vec![1, 2, 3];
    // v[99]; // 越界访问, 会触发 panic
    println!("{:?}", v);

    // Result 的定义, 位于标准库中且 prelude 中已经导入
    // 利用泛型, 可以返回任意类型的错误信息
    // rust 也没有 try catch 语句, 只能使用 match, ?, if let, expect 等语句处理错误
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // fr 是一个 Result 类型的枚举值, 要获得 File 类型的值, 需要使用 match 匹配处理可能的错误
    // 默认的路径是 workdir
    let fr = File::open("hello.txt");
    // f 是 File 类型的值， 是文件句柄, 用于读写文件, 不是文件内容
    let f = match fr {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("file: {:?}", f);

    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // 匹配错误类型, 如果是 NotFound 错误, 则创建文件
        // 嵌套了 match 表达式, 但是这样写太繁琐了, 可以使用闭包来简化
        Err(error) => match error.kind() {
            // ErrorKind 是一个枚举类型, 包含了很多种内置的错误类型, 包括 NotFound, PermissionDenied, ConnectionRefused 等
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    println!("file: {:?}", greeting_file);

    // 使用闭包简化, unwrap_or_else 接受一个闭包作为参数, 如果 Result 是 Ok, 则返回 Ok 中的值, 如果是 Err, 则执行闭包
    // unwrap_or_else 与 unwrap 的区别是, unwrap_or_else 可以自定义错误处理逻辑, 而 unwrap 只能使用 panic!
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("file: {:?}", greeting_file);

    // unwrap 和 expect 都是返回 Ok 中的值, 如果是 Err, 则触发 panic
    // unwrap 会使用默认的 panic! 信息, expect 可以自定义 panic! 信息
    let greeting_file = File::open("hello.txt").unwrap();
    println!("file: {:?}", greeting_file);
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
    println!("file: {:?}", greeting_file);

    let file = match read_hello_from_file() {
        Ok(file) => file,
        Err(e) => panic!("Problem reading the file: {:?}", e),
    };
    println!("file: {:?}", file);

    // 在 main 中如果使用 ? 运算符, 则 main 函数的返回值必须是 Result 类型, 所以要把 main 的返回值改为 Result<(), Error>
    // 只有在返回 Result 类型的函数中才能使用 ? 运算符， 用于简化错误传递
    let file1 = read_hello_from_file_2()?;
    let file2 = match read_hello_from_file_2() {
        Ok(file) => file,
        Err(e) => panic!("Problem reading the file: {:?}", e),
    };
    println!("file1: {:?}, file2: {:?}", file1, file2);
    let last_char = last_char_of_first_line(&file2);
    println!("last char: {:?}", last_char);

    // 匹配 main, 需要一个 Ok(()) 返回
    Ok(())

    // 适合用 panic! 的场景
    // 1. 代码中有 bug, 但是不知道如何处理
    // 2. 示例代码中, 为了简化, 不处理错误
    // 3. 原型代码和测试代码中, 不处理错误
    // 4. 代码中有不可恢复的错误, 比如文件损坏, 网络连接中断等
    // 5. 我们比编译器更清楚程序的运行情况, 比如 unwrap, expect, unwrap_or_else 等
    // 6. 任何有可能导致有害状态的错误, 比如内存不足, 硬件故障等, panic 会导致程序退出, 从而避免了有害状态的发生, 明确程序有 bug
    // 7. 有些需要程序员修复代码的错误, 没有合理的恢复策略

    // 适合用 Result 的场景
    // 1. 代码中有 bug, 但是知道如何处理
    // 2. 代码中有可恢复的错误, 比如文件不存在, 网络连接超时等
    // 3. 出现了可预见的错误, 比如用户输入错误, 无法连接数据库等

    // 例子 guessing_game 中, 我们用 loop 逻辑处理了用户输入错误的情况, 也是一种错误处理方式
}

// 我们也可以自定义类型, 用于处理特定的错误
pub struct Guess {
    value: i32,
}

impl Guess {
    // 用于表示用户输入的数字, 只能是 1-100 之间的数字, 如果不是, 则 panic
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
    // 获取用户输入的数字
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn read_hello_from_file() -> Result<String, Error> {
    let hello_file_result = File::open("hello.txt");
    let mut hello_file = match hello_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    // 读取文件内容到字符串中
    let mut hello = String::new();
    // read_to_string 方法返回一个 Result 类型的值, 如果是 Ok, 则返回读取的字节数, 如果是 Err, 则返回错误信息
    match hello_file.read_to_string(&mut hello) {
        Ok(_) => Ok(hello),
        Err(e) => Err(e),
    }
}

// 使用 ? 运算符简化错误处理, 本函数等价于 read_hello_from_file
// ? 运算符只能用于返回 Result 类型的函数, 如果是 Ok, 则返回 Ok 中的值, 如果是 Err, 则将 Err 的值作为整个函数的返回值
fn read_hello_from_file_2() -> Result<String, Error> {
    // ? 在内部会执行 match 表达式, 如果是 Err, 则直接返回 Err 的值, 如果是 Ok, 则返回 Ok 中的值
    //  let hello_file_result = File::open("hello.txt");
    //  let mut hello_file = match hello_file_result {
    //      Ok(file) => file,
    //      Err(e) => return Err(e),
    //  };
    let mut hello_file = File::open("hello.txt")?;
    let mut hello = String::new();
    // match hello_file.read_to_string(&mut hello) {
    //     Ok(_) => Ok(hello),
    //     Err(e) => Err(e),
    // }
    hello_file.read_to_string(&mut hello)?;
    // 需要用 Ok 包装一下, 因为 ? 运算符只能用于返回 Ok 的值或者 Err 的值
    Ok(hello)
}

// ? 运算符的链式调用, 获取文件第一行的最后一个字符
fn last_char_of_first_line(text: &str) -> Option<char> {
    // lines() 方法返回一个迭代器, 迭代器的元素是字符串的引用
    // next()? 取第一行的字符串的引用, 然后使用 ? 运算符, 如果是 None, 则直接返回 None, 如果是 Some, 则取 Some 中的值, 得到字符串 slice
    // chars() 方法返回一个迭代器, 迭代器的元素是字符串 slice 的字符
    // last() 方法返回迭代法器的最后一个字符的 Option, 如果是 None, 则直接返回 None, 如果是 Some, 则取 Some 中的值, 得到字符
    text.lines().next()?.chars().last()
}

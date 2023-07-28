// 目标是通过执行 cargo run -- searchstring example-filename.txt
// 可以在 example-filename.txt 中搜索 searchstring

use std::{env, process};

fn main() {
    // env 是一个迭代器, 用于迭代所有的环境变量
    // args() 方法返回一个迭代器, 用于迭代命令行参数
    // collect() 方法将迭代器转换为集合, 可以指定为 Vec<String>
    // collect 总是需要显式指定类型, 因为 collect 可以转换为多种集合类型
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // let query = &args[1];
    // let path = &args[2];
    // [ 优化后 ]
    // let conf = match Config::new(&args) {
    //     Ok(conf) => conf,
    //     Err(err) => {
    //         eprintln!("parse arguments failed: {}", err);
    //         // 退出程序, 退出码为 1
    //         process::exit(1);
    //     }
    // };
    // 也可以用 unwrap_or_else
    let conf = pandastd_mini_grep::Config::new(&args).unwrap_or_else(|err| {
        // eprintln! 会将内容输出到标准错误流, 而不是标准输出流
        // 标准错误流会被输出到终端, 而不会被 shell 重定向
        eprintln!("parse arguments failed: {}", err);
        process::exit(1);
    });

    // 读取文件
    // let contents = match fs::read_to_string(conf.path) {
    //     Ok(contents) => contents,
    //     Err(err) => {
    //         eprintln!("Error reading file: {}", err);
    //         return;
    //     }
    // };
    // eprintln!("With text:\n{}", contents)
    // [ 优化后 ]
    // 这种写法就很像 go 的错误处理机制, if err != nil { handle err }
    if let Err(err) = pandastd_mini_grep::run(conf) {
        eprintln!("read file failed: {}", err);
        process::exit(1);
    }
}

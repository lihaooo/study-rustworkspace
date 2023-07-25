// 导入 front_of_house 模块, 该模块在 front_of_house.rs 文件中
// 也可能是 front_of_house/mod.rs 文件, 但是不管是哪种, 都会在当前目录下寻找
mod front_of_house;

// use 不一定要在模块的开头, 可以在任何地方使用
// 加了 pub 之后, 可以在模块外部使用， 否则只能在本模块中使用, 本模块指整个 crate
pub use crate::front_of_house::hosting;
// 一般不会使用 use 导入到模块的函数, 而是只导入到 mod 层
// 否则会使得函数的来源不明确, 但这是允许的
// use crate::front_of_house::hosting::add_to_waitlist;

// 而在导入结构体和枚举时, 一般会使用 use 导入完整路径
use std::collections::HashMap;

// 使用 as 关键字重命名导入的模块
// 为了避免 Result 重名, 一般会使用 as 重命名
use std::fmt::Result;
use std::io::Result as IoResult;

use rand::Rng;

// 使用 use 时, 可以通过大括号导入多个模块
// use std::cmp::Ordering;
// use std::io;
// [ 优化后 ]
// use std::{cmp::Ordering, io};

// 使用 use 时, 可以用 self, super, crate 关键字来访问当前模块, 父模块, crate 根模块
// use std::io;
// use std::io::Write;
// [ 优化后 ]
// use std::io::{self, Write};

// 使用 * 通配符导入所有公有的定义, 但是一般不推荐这么做, 容易造成命名冲突
// glob 运算符 * 常用于 tests 模块和 prelude 模式中
// prelude 模式是 rust 为了方便使用而预先导入的模块, 一般不需要手动导入
// use std::collections::*;

fn function1() -> Result {
    println!("function1");
    Ok(())
}

fn function2() -> IoResult<()> {
    println!("function2");
    Ok(())
}

pub fn eat_at_restaurant() {
    // 绝对路径访问 add_to_waitlist 函数
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径访问 add_to_waitlist 函数
    front_of_house::hosting::add_to_waitlist();
    // 使用 use 关键字导入 hosting 模块
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {:?} please", meal);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("{:?}, {:?}", order1, order2);

    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map);

    function1().expect("function1 error");
    function2().expect("function2 error");

    // 使用 rand 生成随机数, 通过 use rand::Rng 导入 Rng trait
    let num = rand::thread_rng().gen_range(1..101);
    println!("num: {}", num);
}

pub mod back_of_house;

fn serve_order() {}

// cfg 为编译器提供条件判断, cfg(test) 表示仅在测试时才编译
#[cfg(test)]
// tests 模块中的测试函数可以访问 crate 中的内容, 名称为 tests 的模块会被 cargo test 自动识别
mod tests {
    // 导入父模块中的内容, super 为父模块的路径, * 为导入所有内容, 写作 crate::* 也可以
    use super::*;
    // test 标记表示该函数为测试函数, 该函数仅在 cargo test 时才会被编译
    // cargo test 时会运行所有标记为 test 的函数
    // 和 cfg(test) 的区别是, cfg(test) 标记的是模块, test 标记的是函数
    #[test]
    // 测试函数的名称一般以 test_ 开头, 但不是必须的
    fn it_works() {
        front_of_house::hosting::add_to_waitlist();
        eat_at_restaurant();
        back_of_house::fix_incorrect_order();
    }
    #[test]
    fn test_1() {
        println!("test_1");
    }
}

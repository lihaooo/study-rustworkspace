fn main() {
    another_function(5, "hours"); // 调用 another_function 函数

    // x = y = 6 这样的赋值语句在 Rust 中是不合法的
    let x = 6;
    let y = 6;
    println!("x = {}, y = {}", x, y);

    // 使用代码块来创建新的作用域, 继而给 y 赋值
    // 其等价于
    // let x = 3;
    // let y = x + 1;
    // 只不过 x 的作用域变成了代码块内部, 只有 y 可以被其他代码访问
    let y = {
        let x = 3;
        x + 1 // 注意这里没有分号, 分号会变成语句, 而不是表达式
    };
    println!("The value of y is: {}", y);

    let x = 0;
    println!("Call function five(), the value is: {}", five());
    println!("Call function plus_one(), the value is: {}", plus_one(x));
}

// rust 中函数和变量名使用 snake_case 风格
fn another_function(x: i32, unit_label: &str) {
    println!("Another function.\nThe value is: {:02} {}", x, unit_label);
}

fn five() -> i32 {
    // return 5; // 有分号, 作为语句
    5 // 无分号, 作为返回值
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 无分号, 作为返回值
}

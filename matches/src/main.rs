fn main() {
    // Option<T> 是一个枚举类型, Some(T) 和 None 是它的两个成员, 可用于处理空值
    let x: Option<i32> = Some(5);
    // 通过 match 匹配 Option<T> 的值, match 需要穷举所有可能的值, _ 代表其他所有值
    match x {
        Some(5) => println!("got 5"),
        Some(6) => println!("got 6"),
        Some(7) => println!("got 7"),
        None => println!("got None"),
        _ => println!("got something else"),
    }

    // if let 语法糖, 用于匹配单个值, 适用于只关心某个值的情况
    let x = Some(5);
    if let Some(4) = x {
        println!("got 4");
    } else {
        println!("got something else");
    }

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    // "34" 是字符串字面量, parse() 方法将字符串转换为数字, 但是 parse() 方法返回的是 Result<T, E> 类型, 用于处理错误
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("favorite color is {}", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        // age 是 u8 类型
        if age > 30 {
            println!("age is {}", age);
        } else {
            println!("age is {}", age);
        }
    } else {
        println!("age is {}", age.unwrap());
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    for item in stack.iter() {
        println!("{}", item);
    }
    // while let 语法糖, 用于匹配多个值, 适用于只关心多个值的情况
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    // 上面的 while 把 stack 中的元素全部取出来了, 所以 stack 是空的
    for item in stack.iter() {
        println!("{}", item);
    }

    // for 循环中的模式匹配, iter() 的 enumerate() 方法返回一个元组, 元组的第一个元素是索引, 第二个元素是值
    // 这是非常经典的遍历数组的方法, 多种语言都有类似的实现
    let v = vec![1, 2, 3];
    for (i, v) in v.iter().enumerate() {
        println!("{} is at index {}", v, i);
    }

    // 可以用元组来创建多个变量
    let (x, y) = (1, "a");
    println!("x = {}, y = {}", x, y);

    let point = (3, 5);
    print_coordinates(&point);

    // let 后面只能跟不可反驳的模式, 不能跟可反驳的模式
    // 可反驳的模式是指可能会匹配失败的模式, 例如 Some(x)
    // let Some(x) = Some(5);
    if let Some(x) = Some(5) {
        println!("got {}", x);
    }
    // 在 if let 后跟不可反驳的模式是没有意义的, 因为一定会匹配成功
    // 直接改写为 let x = 5; 会更简洁
    // 但是不会报错, 编译器会给出警告
    // if let x = 5 {
    //     println!("got {}", x);
    // }

    // 对字面量进行模式匹配, 可以使用 | 运算符, 其他可以用的运算符还有 & 和 @
    // & 是取引用, 例如 &Some(5) 会匹配 Some(5) 的引用, 而不是 Some(5) 的值
    // @ 是给匹配的值起别名, 例如 Some(x) @ 5 会匹配 Some(5) 的值, 并且把 5 赋值给 x
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    let x = Some(5);
    match x {
        Some(5) => println!("got 5"),
        Some(_) => println!("got something else"),
        None => println!("got None"),
    }
    let x = Some(5);
    match &x {
        // 这里不写 & 也可以, 编译器会自动转换
        &Some(5) => println!("got 5"),
        &Some(_) => println!("got something else"),
        None => println!("got None"),
    }
    let x = Some(5);
    match x {
        y @ Some(5) => println!("got y = {:?}", y),
        Some(_) => println!("got something else"),
        None => println!("got None"),
    }
    // 还可以通过 x..[=]y 区间运算符匹配范围
    let x = 5;
    match x {
        // ..= 代表闭区间, 5 也会被匹配
        ..=5 => println!("got 5"),
        _ => println!("got something else"),
    }
    let x = 'c';
    match x {
        'a'..='z' => println!("got a char"),
        _ => println!("got something else"),
    }

    // 对命名变量进行模式匹配
    let x = Some(10);
    let y = 10;
    match x {
        Some(50) => println!("got 50"),
        // 匹配守卫, 可以在模式后面加上 if 条件, 只有条件为真时才会匹配成功
        Some(n) if y == n => println!("got 10"),
        // 这里的 y 是一个新的变量, 不是上面的 y, 这个 y 会匹配任何值, 并且把匹配的值赋值给 y
        Some(y) => println!("got y = {:?}", y),
        _ => println!("got something else"),
    }

    // 解构结构体
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    println!("point is {:?}", p);
    println!("a = {}, b = {}", a, b);

    // 简化写法, 变量名和结构体字段名相同的时候可以简写
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    println!("point is {:?}", p);
    println!("x = {}, y = {}", x, y);
    // 还可以在模式中忽略某些字段, 例如只关心 x 坐标或 y 坐标
    match p {
        Point { x, y: 0 } => println!("on the x axis at {x}"),
        Point { x: 0, y } => println!("on the y axis at {y}"),
        Point { x, y } => {
            println!("on neither axis: ({x}, {y})");
        }
    }

    // 解构枚举
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("quit"),
        Message::Move { x, y } => println!("move to ({}, {})", x, y),
        Message::Write(text) => println!("write {}", text),
        Message::ChangeColor(r, g, b) => println!("change color to ({}, {}, {})", r, g, b),
    }

    // 解构嵌套的结构体和枚举
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("change color to rgb({}, {}, {})", r, g, b)
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("change color to hsv({}, {}, {})", h, s, v)
        }
        _ => (),
    }

    // 解构结构体和元组, 类似解方程, let (x, y, z) = (1, 2, 3)
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet = {}, inches = {}, x = {}, y = {}", feet, inches, x, y);

    // 忽略模式中的值, 可以使用 _ 或 .., 例如 let _x = 5; 或 let Point { x: _, y: _ } = p;
    foo(3, 4);
    let mut setting_value = Some(5);
    let new_setting_value = None;
    match (setting_value, new_setting_value) {
        // _ 被包裹在 Some 中, 所以只有当 setting_value 和 new_setting_value 都是不为 None 的 Some 时才会匹配成功
        (Some(_), Some(_)) => {
            println!("can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    // .. 忽略剩余的值或 _ 忽略单个值
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("some numbers: {first}, {third}, {fifth}")
        }
    }
    match numbers {
        // .. 忽略剩余的值
        (first, .., last) => {
            println!("some numbers: {}, {}", first, last);
        }
    }
    // 因为 .. 忽略的是多个值， .., second, .. 并不能告诉编译器 second 是要匹配哪个值, 所以会报错
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     }
    // }

    // _x 指不会使用到的变量, 但是编译器会检查这个变量是否存在, 如果不存在会报错
    // 一定要用也可以用
    let _x = 5;
    let y = _x;
    println!("x: {}, y: {}", _x, y);

    // 用 .. 匹配结构体中的剩余字段
    let origin = Point3D { x: 1, y: 2, z: 3 };
    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }
    println!("o.x = {}, o.y = {}, o.z = {}", origin.x, origin.y, origin.z);

    // 匹配守卫
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ 绑定, 将 match 的匹配结构绑定到变量, 右边绑定到左边
    let msg = MessageHello::Hello { id: 5 };
    match msg {
        MessageHello::Hello {
            id: id_variable @ 3..=7,
        } => println!("found an id in range: {}", id_variable),
        MessageHello::Hello { id: 10..=12 } => {
            println!("found an id in another range")
        }
        MessageHello::Hello { id } => println!("found some other id: {}", id),
    }
}

enum MessageHello {
    Hello { id: i32 },
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

pub enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn foo(_: i32, y: i32) {
    println!("this code only uses the y parameter: {}", y);
}

// 函数参数可以是元组, 也可以是元组的引用, 可以简化代码不需要写多个参数或定义额外的结构体
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("current location: ({}, {})", x, y);
}

fn main() {
    let num = 3;
    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 不能直接使用非 bool 值作为条件
    // if num {
    //     println!("number was not zero");
    // }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if 是一个表达式, 可以在 let 语句的右侧使用, 起到类似三目运算符的作用
    // if 和 else 的返回值类型必须相同
    let cond = true;
    let num = if cond { 5 } else { 6 };
    println!("The value of num is: {}", num);

    // loop 关键字创建一个无限循环
    let mut x = 0;
    loop {
        x = x + 1; // 1, 2, 3
        println!("again! x = {}", x);
        if x > 2 {
            break;
        } // 使用 break 语句跳出循环
    }

    // 可以用 loop 对变量进行赋值, break 后返回赋值结果
    let mut count = 0;
    let res = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("The result is: {}", res);

    // 可以给 loop 命名, 用于 break 语句
    let mut a = 0;
    'loop_a: loop {
        println!("loop_a: a = {}", a);
        let mut b = 10;
        'loop_b: loop {
            println!("loop_b: b = {}", b);
            if b == 9 {
                break 'loop_b;
            }
            if a == 2 {
                break 'loop_a;
            }
            b -= 1;
        }
        a += 1;
    }

    // while 循环, 判断条件为 true 时执行循环体
    let mut num = 3;
    while num != 0 {
        println!("{}!", num); // 还可以写成 println!{"{num}!"};
        num -= 1;
    }
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let mut x: usize = 0;
    let length = a.len();
    while x < length {
        println!("The value of a[{x}] is: {}", a[x]);
        x += 1;
    }

    // for 遍历
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    for v in a {
        println!("The value is: {}", v);
    }
    // x..y 表示一个范围, 从 x 到 y, 不包括 y
    for v in 1..5 {
        println!("The value is: {}", v);
    }
    // x..=y 表示一个范围, 从 x 到 y, 包括 y, rev() 表示顺序反转
    for v in (1..=5).rev() {
        println!("{v}");
    }
}

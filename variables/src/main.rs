fn main() {
    // mut 关键字用来声明可变变量
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // 可变变量可以重新赋值
    println!("The value of x is: {}", x);

    // const 关键字用来声明常量
    // 常量必须声明类型，且只能使用不可变变量
    // 常量通常使用大写字母和下划线来命名 snake_case + UPPER_CASE
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!(
        "The value of THREE_HOURS_IN_SECONDS is: {}", // {} 可以占位任何实现了 Display trait 的类型
        THREE_HOURS_IN_SECONDS
    );

    // 遮蔽是指在同一作用域中声明了同名变量，新变量会遮蔽旧变量
    // 上面的 x 变量在这里被遮蔽了，变成了一个新的不可变变量
    let x = 5;
    // x = 1; // 编译报错，因为 x 是不可变变量
    let x = x + 1;
    // { ... } 用来创建一个新的作用域，不对外部作用域造成影响，这和 golang 一样
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("Ths value of x is {}", x);

    let spaces = "   ";
    let spaces = spaces.len(); // 遮蔽可以改变变量的类型
    println!("The value of spaces is: {}", spaces);

    // 通过 trim -> parse -> expect 来将字符串转换为数字
    // expect 用来处理 parse 失败的情况
    // parse 的结果是泛型，需要使用 let guess: u32 来指定类型
    let guess: u32 = "42".trim().parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    // 整数类型
    // i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
    // isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上是 64 位，32 位架构上是 32 位
    // 默认是 i32
    // 可以使用 0x 开头来表示十六进制，0o 开头来表示八进制，0b 开头来表示二进制
    // 可以使用 _ 来增加可读性，例如 1_000_000
    // 可以使用 [type]::MAX 来获取最大值，[type]::MIN 来获取最小值
    let a: u32 = 255_000_000;
    let b = u8::MAX;
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);

    // 浮点数类型
    // f32, f64
    // 默认是 f64，因为现代 CPU 上 f64 的运算速度和 f32 差不多
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("The value of x is: {:.01}", x); // {:.01} 表示小数点后保留一位
    println!("The value of y is: {:02}", y); // {:02} 表示占两位，不足两位用 0 填充

    // 数值运算
    let sum = 5 + 10; // 加法
    let diff = 95.5 - 4.3; // 减法
    let product = 4 * 30; // 乘法
    let quotient = 56.7 / 32.2; // 除法
    let floored = 2 / 3; // 整数除法，结果为 0
    let remainder = 43 % 5; // 取余
    println!("The value of sum is: {}", sum);
    println!("The value of diff is: {}", diff);
    println!("The value of product is: {}", product);
    println!("The value of quotient is: {:.02}", quotient);
    println!("The value of floored is: {}", floored);
    println!("The value of remainder is: {}", remainder);

    // 布尔类型
    let t = true;
    let f: bool = false; // 显式指定类型
    println!("The value of t is: {}", t);
    println!("The value of f is: {}", f);

    // 字符类型
    // char 类型使用单引号，而不是双引号
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    // 复合类型：包括元组和数组
    // 元组是一个固定长度的有序列表，每个元素都有自己的类型
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tuple is: {:?}", tuple); // {:?} 表示调试输出，否则 tuple 没有实现 Display trait
    let (x, y, z) = tuple; // 元组可以通过解构来获取其中的值
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    let unit_value = (); // 空元组，也称为 unit 类型，只有一个值，也就是空值
    println!("The value of unit_value is: {:?}", unit_value);
    // 元组可以通过索引来获取其中的值
    println!("The value of tuple.0 is: {}", tuple.0);
    println!("The value of tuple.1 is: {}", tuple.1);
    println!("The value of tuple.2 is: {}", tuple.2);

    // 数组是一个固定长度的有序列表，每个元素有相同的类型
    let a = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // 显式指定类型和长度
    let a = [3; 5]; // 等价于 let a = [3, 3, 3, 3, 3];
    println!("The value of a is: {:?}", a);
    let first = a[0]; // 数组可以通过索引来获取其中的值
    let second = a[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);
    // let index = 10;
    // let element = a[index]; // 这种情况 IDE 不会报错，但编译报错，数组越界
    // println!("The value of element is: {}", element);
}

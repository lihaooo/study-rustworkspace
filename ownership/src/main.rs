fn main() {
    // stack 中的数据必须是已知固定大小的, 不能动态增长或者缩小, 过程不视为分配, 但是速度快
    // 函数过程中, 传递给函数的值和局部变量都会被分配到栈上, 函数结束后, 局部变量会被释放
    // heap 中的数据是不固定大小的, 可以增长或者缩小, 但是需要用指针手动管理, 过程叫分配

    // 所有权规则
    // 1. rust 中的每一个值都有一个被称为其所有者 owner 的变量
    // 2. 值有且只有一个所有者
    // 3. 当所有者离开作用域, 这个值将被丢弃

    // 声明一个 String 类型的变量 s, 并将其值设置为 hello
    // 因为 String 是不定长的, s 是一个指向 heap 中的内存地址的指针, 该内存地址存储了字符串的内容
    let mut s = String::from("hello"); // String::from 相当于访问了 String 类的抽象方法 from
    s.push_str(", world!"); // push_str() 方法只接受字符串 slice, push() 方法只接受单个字符
    println!("{}", s); // s 将在函数结束时被释放

    // 变量与数据交互的方式: 移动, 克隆, 拷贝
    let mut x = 5;
    let y = x; // x 的值被复制到了 y 中, 两个变量都是 5, 因为这是 stack 数据, copy 成本小
    x += 1;
    println!("x = {}, y = {}", x, y);

    // s1 的值被移动到了 s2 中, s1 无效了, rust 的移动是浅拷贝加删除, 将指针指向了同一块内存
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // s1 无效了, 不能再使用, 值只有一个所有者, rust 永远也不会自动创建数据的深拷贝
    println!("s2 = {}", s2);

    // 如果需要深拷贝, 可以使用 clone 方法, clone 相比于移动, 是深拷贝, 会在 heap 上重新分配内存， 要注意性能问题
    let mut s1 = String::from("hello");
    let mut s2 = s1.clone();
    s1.push_str("_1");
    s2.push_str("_2");
    println!("s1 = {}, s2 = {}", s1, s2);

    // 如果一个类型实现了 Copy trait, 那么一个旧的变量在将其赋值给其他变量后仍然可用
    // rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait
    // 元组中的每一个元素都必须是 Copy 类型, 才能使用 Copy trait, 如 (i32, String) 就没有

    let s = String::from("hello");
    take_ownership(s);
    // let y = s; // s 的值被移动到了函数中了, 所以这里不再有效
    // println!("s = {}", s);

    let x = 5;
    make_copy(x);
    // 因为 i32 是 Copy 类型, 所以 x 依然有效
    println!("x = {}", x);

    let s1 = give_ownership();
    // s1 从函数返回值中获得了 String 的所有权
    println!("s1 = {}", s1);

    let s2 = take_and_give_back(s1);
    // s1 的值被移动到了函数中了, 所以 s1 不再有效
    // println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    let s1 = String::from("hello");
    // 如果用引用 &s1 作为参数, 而不是直接使用 s1, 这样 s1 依然有效
    let (s2, len) = calculate_length(s1);
    // println!("s1 = {}", s1);
    println!("s2 = {}, len = {}", s2, len);
    let len = calculate_length_2(&s2);
    println!("s2 = {}, len = {}", s2, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("s = {}", s);

    // 在同一时间, 只能有一个对某一特定数据的可变引用, 尝试创建两个可变引用的代码将会失败
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // 不能同时存在两个可变引用
    println!("r1 = {}", r1);

    // 同时创建多个不可变引用是可以的, 因为不可变引用不会改变数据, 所以没有竞争条件
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // 不能同时存在可变引用和不可变引用
    println!("r1 = {}, r2 = {}", r1, r2);

    // 数据竞争条件, 3 个同时发生才会产生数据竞争
    // 1. 两个或更多指针同时访问同一数据
    // 2. 至少有一个指针被用来写入数据
    // 3. 没有同步数据访问的机制

    let mut s = String::from("hello");
    {
        // r1 在这里离开了作用域, 所以我们完全可以创建一个新的引用
        let r1 = &mut s;
        println!("r1 = {}", r1);
    }
    let r2 = &mut s;
    println!("r2 = {}", r2);

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用
    // 编译器在作用域结束之前判断不再使用的引用的能力被称为非词法作用域生命周期 Non-Lexical Lifetimes, 简称 NLL
    let r3 = &mut s;
    println!("{}", r3);

    let s = no_dangle();
    println!("s = {}", s);

    // 在任意给定时间, 要么只能有一个可变引用, 要么只能有多个不可变引用
    // 引用必须总是有效的

    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("word = {}, s = {}", word, s);
    s.clear();
    // s 的内容被清空了, 但是 word 依然是 5, 这时候 word 没有同步与 s 的关系
    println!("word = {}, s = {}", word, s);

    let mut s = String::from("hello world");
    let hello = &s[0..5]; // 不包括 5, 因为索引从 0 开始
    let world = &s[6..11];
    let slice = &s[..]; // 等价于 &s[0..s.len()]
    let slice = &s[..2]; // 等价于 &s[0..2]
    let slice = &s[2..]; // 等价于 &s[2..s.len()]
    println!("hello = {}, world = {}, slice = {}", hello, world, slice);
    // s.clear();
    // s.push_str("123");
    // 如果 s 被清空了, 那么 hello 和 world 就不再有效了, 所以如果 hello 和 world 不是最后一次被使用, s 就不能被修改
    println!("hello = {}, world = {}", hello, world);

    let mut s = String::from("hello world");
    // 通过 first_word_2 函数, word 获得了 s 的 slice 的引用
    let word = first_word_2(&s);
    println!("word = {}, s = {}", word, s);
    // s.clear();
    // s.push_str("123");
    // 同上, 如果 s 被清空了, 那么 word 就不再有效了, 所以如果 word 不是最后一次被使用, s 就不能被修改
    println!("word = {}, s = {}", word, s);
    s.push_str("123");
    // 如果不再使用 word, s 是可以被修改的
    println!("s = {}", s);

    let s = String::from("hello world");
    let word = first_word_2(&s[0..6]);
    let word = first_word_2(&s[..]); // 等价于 &s[0..s.len()] 和 &s
    let s = "hello world"; // 字符串字面值就是 slice
    let word = first_word_2(&s[0..6]);
    let word = first_word_2(&s[..]);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("slice = {:?}", slice);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
    // some_string 在这里离开作用域并被丢弃, drop 方法被调用, 内存被释放
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
    // some_integer 在这里离开作用域, 但是因为是 Copy 类型, 所以没有特殊操作
}

fn give_ownership() -> String {
    let some_string = String::from("hello");
    // 将 some_string 的值移动到调用它的函数中
    some_string
}

fn take_and_give_back(some_string: String) -> String {
    // 将 some_string 的值移动到调用它的函数中
    some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    // 用元组返回多个值, 和 go 的多返回值类似
    (s, length)
}

fn calculate_length_2(s: &String) -> usize {
    // s 是对 String 的引用, 不会获取所有权, 所以 s 在函数结束后依然有效
    // & 符号就是引用, 它们允许你使用值但不获取其所有权
    s.len()
}

fn change(some_string: &mut String) {
    // 可变引用, 可以修改引用的值
    some_string.push_str(", world");
}

// fn dangle() -> &String { // dangle 返回一个字符串的引用
// let s = String::from("hello"); // s 是一个新字符串
// &s // 返回字符串 s 的引用
// 这里 s 离开作用域并被丢弃。其内存被释放
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // 将 String 转换为字节数组
    for (index, &item) in bytes.iter().enumerate() {
        // iter() 方法返回集合中的每一个元素, enumerate() 方法将其包装在一个元组中, 并返回一个元组的集合
        if item == b' ' {
            // b' ' 是字节字面值语法, 表示单字节 Unicode 标量值
            return index;
        }
    }
    s.len()
}

// 把 s 的类型从 &String 改为 &str, 这样函数就可以接受 String 和 &str 类型的值了
// String 是一个 Vec<u8> 的封装, 用于存储 UTF-8 编码的字符串
// &str 是一个不可变引用, 通常用于指向一些 UTF-8 编码的字符串的切片
// String 也可以看作是一个 UTF-8 编码的字符串的切片, &s[..]
fn first_word_2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..index];
        }
    }
    // 返回的是一个字符串 slice 的引用
    &s[..]
}

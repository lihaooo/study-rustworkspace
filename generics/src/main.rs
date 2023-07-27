use std::fmt::{Debug, Display};

fn main() {
    // 在 num_list_1 中找到最大的数字
    let num_list_1 = vec![34, 50, 25, 100, 65];
    let mut largest = &num_list_1[0];
    for number in &num_list_1 {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    // 在 num_list_2 中找到最大的数字
    let num_list_2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = &num_list_2[0];
    for number in &num_list_2 {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    // 调用 get_largest_i32 函数
    let num_list_1 = vec![34, 50, 25, 100, 65];
    let num_list_2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    println!(
        "The largest number in num_list_1 is {}",
        get_largest_i32(&num_list_1)
    );
    println!(
        "The largest number in num_list_2 is {}",
        get_largest_i32(&num_list_2)
    );

    // 在 char_list 中找到最大的字符, 字符的大小是比较 ASCII 码的大小, 例如 'a' < 'b'
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!(
        "The largest char in char_list is {}",
        get_largest_char(&char_list)
    );

    // 调用 largest 函数
    let num_list_1 = vec![34, 50, 25, 100, 65];
    let num_list_2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let char_list = vec!['y', 'm', 'a', 'q'];
    let num_list_3 = vec![34.0, 50.0, 25.0, 100.0, 65.0];
    println!(
        "The largest number in num_list_1 is {}",
        get_largest(&num_list_1)
    );
    println!(
        "The largest number in num_list_2 is {}",
        get_largest(&num_list_2)
    );
    println!(
        "The largest char in char_list is {}",
        get_largest(&char_list)
    );
    println!(
        "The largest number in num_list_3 is {:.1}",
        get_largest(&num_list_3)
    );

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!(
        "integer = {:?}, x = {}, y = {}",
        integer, integer.x, integer.y
    );
    println!("float = {:?}, x = {}, y = {}", float, float.x, float.y);
    println!("integer.x = {}", integer.x());
    println!(
        "float.distance_from_origin = {}",
        float.distance_from_origin()
    );
    // 不能使用不同类型的泛型参数来创建 Point 实例
    // let integer_and_float = Point { x: 5, y: 4.0 };

    let integer_and_float = Point2 { x: 5, y: 4.0 };
    println!(
        "integer_and_float = {:?}, x = {}, y = {}",
        integer_and_float, integer_and_float.x, integer_and_float.y
    );
    println!("integer_and_float.x = {}", integer_and_float.x);
    // 因为 Point2<T1, T2> 结构体中的 x 和 y 的类型不同, 所以不能调用 distance_from_origin 方法
    // println!(
    //     "float.distance_from_origin = {}",
    //     integer_and_float.distance_from_origin()
    // );

    // 定义多个泛型可能会让代码难以阅读和理解, 所以如果可以的话, 尽量使用单个泛型
    // 当你发现代码中需要很多泛型时，这可能表明你的代码需要重构分解成更小的结构

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mix(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // rust 中的泛型并不会使程序比具体类型运行得慢, 这和 go 语言不同
    // 因为 rust 在编译时会将泛型代码编译为具体类型的代码, 称为单态化, 所以运行时的性能和具体类型是一样的

    // let integer = Some(5);
    // let float = Some(5.0);
    // 编译器会将泛型代码编译为具体类型的代码, 类似于下面的代码
    // enum Option_i32 {
    //     Some(i32),
    //     None,
    // }
    //
    // enum Option_f64 {
    //     Some(f64),
    //     None,
    // }
    // let integer = Option_i32::Some(5);
    // let float = Option_f64::Some(5.0);

    // trait 在 rust 中类似于接口, 但是 trait 和接口有一些不同
    // trait 中可以包含方法的默认实现, 但是接口中不能包含方法的默认实现, 其他还有一些差别

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let news = NewsContent {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("news: {}", news.summarize());
    println!("news author: {}", news.summarize_author());

    // trait 无法从相同方法的重载实现中调用默认实现, 即不能基于某些条件选择默认实现, 只能完全重写

    // 因为 3 是 i32 类型, 而 i32 类型实现了 Display, 所以可以调用 to_string 方法
    let s = 3.to_string();
    println!("s = {}", s);

    // rust 通过生命周期来避免悬垂引用
    // let r;
    // {
    // let x = 5;
    // 以下代码是错误的, 因为 x 的生命周期比 r 的生命周期短, 所以不能将 x 的引用赋值给 r
    // r = &x;
    // }
    // println!("r: {}", r);

    // 正常的引用, 被引用的值的生命周期比引用的生命周期长
    // r 的生命到最后一次使用, 而 x 的生命周期到大括号结束
    let x = 5;
    let r = &x;
    println!("r: {}", r);

    let string1 = String::from("abc");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // &i32        // 引用
    // &'a i32     // 带有显式生命周期的引用
    // &'a mut i32 // 带有显式生命周期的可变引用

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        // 虽然 string1 和 string2 的生命周期不一样, 但他们都比 result 的生命周期长
        // 在使用了泛型生命周期参数的 longest 函数中, 他们可以被视为有效的参数, 即在函数中认为是同样的生命周期
        let result = longest(string1.as_str(), string2.as_str());
        // result dead
        println!("The longest string is {}", result);
        // string2 dead
    }
    // string1 dead

    // 如果我们把 result 的生命周期调整一下, 情况就不同了
    // 因为 println! 的关系, result 存活到了最后, 生命周期比 string1 和 string2 长, 所以不能使用
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("i.part = {}", i.part);

    // 'static 生命周期相同, 也就是整个程序的生命周期
    let s: &'static str = "I have a static lifetime.";
    println!("s = {}", s);
}

// 提取重复的代码, 得到从 Vec<i32> 中找到最大的 i32 的函数
fn get_largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    // 因为 list 是一个引用, 而引用是不可变的, for 循环会获取 list 中的每一个元素的所有权
    // 所以我们需要使用 &number 来获取 list 中的每一个元素
    for &number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn get_largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// 使用泛型来提取重复的代码, 需要限制泛型 T 的类型, 因为我们需要比较大小, 所以需要 PartialOrd trait
fn get_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        // 因为 > 运算符是 PartialOrd trait 的方法, 所以我们可以使用
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 定义一个 Point<T> 结构体, 里面有两个字段 x 和 y, 两个字段的类型都是 T, x 和 y 的类型必须相同
// 如果 x 和 y 的类型不同, 可以使用多个泛型参数, 例如 Point<T, U>
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 为 Point<T> 结构体定义方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 为 Point<f32> 结构体单独定义方法
impl Point<f32> {
    // 通过勾股定理计算点到原点的距离
    fn distance_from_origin(&self) -> f64 {
        // powi 方法是计算整数次幂的方法, 例如 2.powi(2) = 2^2 = 4
        // sqrt 方法是计算平方根的方法, 例如 4.sqrt() = 2, 返回的是 f32 类型, 要转为 f64 类型需要使用 as f64
        (self.x.powi(2) + self.y.powi(2)).sqrt() as f64
    }
}

#[derive(Debug)]
struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    // 融合两个 Point2 实例, 返回一个新的 Point2 实例
    fn mix<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

// Option<T> 枚举类型, 用于表示一个值是存在或不存在的情况, 已经在标准库中定义了, prelude 中也有导入
// enum Option<T> {
//     Some(T),
//     None,
// }

// Result<T, E> 枚举类型, 用于表示一个操作成功或失败的情况, 已经在标准库中定义了, prelude 中也有导入
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

pub trait Summary {
    // fn summarize(&self) -> String;
    // trait 可以有默认的实现, 这样实现了 Summary trait 的类型就不需要实现 summarize 方法了
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 为 NewsArticle 实现 Summary trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 为 Tweet 实现 Summary trait
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub struct NewsContent {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 为 NewsContent 实现 Summary trait, 使用默认的实现
impl Summary for NewsContent {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

// trait 作为参数时, 可以接受任何实现了该 trait 的类型， 仍然相当于 go 语言中的接口
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound 语法, 与上面的写法等价, 更接近于 go 语言中的接口
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 多个 trait 允许使用 + 连接, 表示同时实现了多个 trait, 以下两种写法等价, 扩大了参数的范围
pub fn notify3(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify4<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 使用 where 从句来简化 trait bound 语法, 以下两种写法等价, where 从句更加清晰
pub fn some_function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {
    println!("t = {}, u = {:?}", t, u);
}
pub fn some_function2<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("t = {}, u = {:?}", t, u);
}

// 同样的, 返回值也可以使用 trait bound
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// 不能用于返回不同类型的情况, 以下代码是错误的
// 单独只返回 NewsArticle 或 Tweet 类型是可以的, 但是不能同时返回两种类型
// fn returns_summarizable2(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 以下代码是错误的, 因为 rust 不知道将要返回的引用是指向 x 还是 y
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         x
//     }
// }
// 需要使用泛型生命周期参数来告诉 rust 返回的引用的生命周期和参数的生命周期的关系
// 'a 是一个泛型生命周期参数, 标记了返回值的生命周期和参数和返回值的生命周期是一样的
// 它的实际含义是 longest 函数返回的引用的生命周期与函数参数所引用的值的生命周期的较小者一致
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 这里的返回值的生命周期是 'a, 也就是参数的生命周期
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 如果 'a 的泛型生命周期返回值的生命周期与参数的生命周期无关, 那么就会出现悬垂引用的错误
// 'a 的返回值的生命周期随函数结束而结束
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// 给结构体 ImportantExcerpt 添加一个带有生命周期参数的字段 part
// 这里的生命周期参数 'a 是必须的, 因为 ImportantExcerpt 实例中的 part 字段的生命周期与 ImportantExcerpt 实例的生命周期有关
// 如果没有 'a, 那么 part 字段的生命周期就是默认的 'static, 也就是整个程序的生命周期
// 这样就无法编译, 因为 part 字段的生命周期比 ImportantExcerpt 实例的生命周期长, 会出现悬垂引用的错误
// 'a 使得 part 字段的生命周期与 ImportantExcerpt 实例的生命周期一致
pub struct ImportantExcerpt<'a> {
    part: &'a str,
}

// impl 和 ImportantExcerpt 之间的生命周期注解的意义是 ImportantExcerpt 的实例不能比 impl 中的引用存在的更久
impl<'a> ImportantExcerpt<'a> {
    // 基于生命周期省略的规则 1, 我们可以不标记 &self 和返回值的生命周期
    pub fn level(&self) -> i32 {
        3
    }
    // 基于生命周期省略的规则 3, 我们可以不标记 &self, announcement 和返回值的生命周期
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 生命周期省略
// 早期版本的 rust 中, 这个函数的签名应该为 fn first_word<'a>(s: &'a str) -> &'a str
// 函数的参数生命周期和返回值的生命周期是一样的, 所以可以省略
// 1. 每一个是引用的参数都分配一个生命周期参数, 如 fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
// 2. 只有一个输入生命周期参数时, 这个生命周期被赋予所有输出生命周期参数, 如 fn foo<'a>(x: &'a i32) -> &'a i32
// 3. 如果方法有多个输入生命周期, 且其中一个是 &self 或 &mut self, 说明这是对象的方法, 那么所有输出生命周期参数被赋予 self 的生命周期
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 结合生命周期, 泛型和 trait bound 的 where 从句的函数
pub fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

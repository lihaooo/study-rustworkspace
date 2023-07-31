// unsafe rust, 不安全的 rust, 不会强制执行内存安全规则, 但会提供额外的能力
// 如果 rust 不允许某些操作, 但是你知道这些操作是安全的, 那么可以使用 unsafe 关键字来标记这些操作

// 必须用到 unsafe 关键字的场景:
// 1. rust 编译器无法确定某些代码是否安全, 但是你知道这些代码是安全的
// 2. 底层计算机硬件固有的不安全性, 必须使用 unsafe 代码来完成特定的操作

// unsafe 的超能力有:
// 1. 解引用裸指针
// 2. 调用不安全的函数或方法
// 3. 访问或修改可变静态变量
// 4. 实现不安全 trait
// 5. 访问 union 的字段, union 主要用于和 c 语言交互, 在一个实例中同时只能使用一个声明的字段

// unsafe 并不会关闭借用检查器或禁用任何其他 rust 安全检查, 如果 unsafe 代码段中存在引用, 还是会进行借用检查
// 只有以上 5 种情况可以不被编译器检查内存安全, 其他情况下的 unsafe 代码都会被编译器检查
// unsafe 意在让程序员自行负责检查内存安全, 但是编译器仍然会确保 unsafe 代码本身是有效的
// 保持 unsafe 代码尽可能小, 并在安全代码中调用 unsafe 代码

use std::ops::Add;
use std::{fmt, slice};

fn main() {
    // 解引用裸指针
    let mut num = 5;
    // 将 &num 强制转换为不可变的裸指针
    let r1 = &num as *const i32;
    // 将 &mut num 强制转换为可变的裸指针
    let r2 = &mut num as *mut i32;
    // 将打印出裸指针的十六进制值, 这是内存地址
    println!("r1 = {:?}, r2 = {:?}", r1, r2);

    // 0x 表示十六进制, 0x012345 表示十六进制的 012345, usize 的意思是指针的大小, 32 位系统是 4 字节, 64 位系统是 8 字节
    // 0x012345usize 表示一个指向内存地址 0x012345 的指针
    // 0x012345usize as *const i32 表示将 0x012345usize 强制转换为一个指向 i32 类型的不可变裸指针
    let addr = 0x012345usize;
    let r = addr as *const i32;
    println!("r = {:?}", r);

    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // 使用 unsafe 关键字来解引用裸指针
    unsafe {
        // 把 *r2 赋值为 6, 这会改变 num 的值, 导致 r1 和 r2 指向的值都是 6
        *r2 = 6;
        println!("r1 = {}", *r1);
        println!("r2 = {}", *r2);
    }

    // 调用不安全的函数或方法必须使用 unsafe 关键字
    // dangerous();
    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    // split_at_mut 方法会返回一个元组, 分割得到两个可变引用
    // 这个函数本身无法通过安全 rust 代码实现, 所以需要 unsafe 实现后再封装为安全的函数
    let (a, b) = r.split_at_mut(3);
    println!("a = {:?}, b = {:?}", a, b);

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);
    println!("a = {:?}, b = {:?}", a, b);

    // 这样会导致应用程序崩溃, 这段代码获取任意内存地址并创造了一个长为 10000 的 slice
    // let address = 0x01234usize;
    // let r = address as *mut i32;
    // let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 1) };
    // println!("values = {:?}", values);

    // 调用 c 语言的函数
    unsafe {
        println!("abs(-3) = {}", abs(-3));
    }

    // 访问静态变量
    println!("name is: {}", HELLO_WORLD);

    // 访问和修改可变静态变量
    // add_to_count 本身是安全的, 已经在内部封装了 unsafe 代码
    add_to_count(3);
    unsafe {
        println!("COUNTER = {}", COUNTER);
    }

    // 访问 union 的字段
    let mut mu = MyUnion { f1: 0 };
    unsafe {
        mu.f2 = 1.0;
        println!("mu.f1 = {}, mu.f2 = {}", mu.f1, mu.f2);
        mu.f2 = 2.0;
        println!("mu.f1 = {}, mu.f2 = {}", mu.f1, mu.f2);
    }

    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    // 重载 + 运算符
    let p3 = p1 + p2;
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let m1 = Millimeters(1000);
    let m2 = Meters(1);
    // 因为只在 Millimeters 和 Meters 之间实现了 Add trait, 所以这里会报错
    // let m3 = m2 + m1;
    let m3 = m1 + m2;
    println!("m3 = {:?}", m3);

    let person = Human(1);
    // 如果 Human 同时实现了 Pilot, Wizard 两个 trait, 那么 fly 方法会有歧义
    // 但他本身也实现了 fly 方法, 所以这里会调用 Human 的 fly 方法
    person.fly();
    // 如果要明确地调用 Pilot 或 Wizard 的 fly 方法, 需要使用完全限定语法
    Pilot::fly(&person);
    Wizard::fly(&person);
    // 也可以调用 Human 本身的完全限定语法
    Human::fly(&person);

    let p1 = Point { x: 1, y: 0 };
    OutlinePrint::outline_print(&p1);

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

// 区别 newtype 和 type alias， newtype 是一个新类型， type alias 是一个别名
// 类似于 go 中的 type NewInt32 int32, type NewInt64 = int64
pub struct NewInt32(i32);
pub type NewInt64 = i64;

// 标准库中的 Result<T> 类型就利用了 type alias
// 即 Result<T, E> 中的 E 被定义为 std::io::Error
// type Result<T> = std::result::Result<T, std::io::Error>;

// trait 的继承, 这里的 OutlinePrint 继承了 fmt::Display
// 实现 OutlinePrint 的类型必须同时实现 fmt::Display
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        // 打印上边框
        println!("{}", "*".repeat(len + 4));
        // 打印左边框
        println!("*{}*", " ".repeat(len + 2));
        // 打印内容
        println!("* {} *", output);
        // 打印右边框
        println!("*{}*", " ".repeat(len + 2));
        // 打印下边框
        println!("{}", "*".repeat(len + 4));
    }
}

// 因为 Wrapper 是一个新类型, 所以虽然可以代理 Vec<T> 的 self.0, 但需要重新实现 Vec<T> 的所有方法才能使用
struct Wrapper(Vec<String>);

// 这是 newtype 模式, 也就是在一个元组结构体中只有一个元素
// 使用这个模式没有运行时性能惩罚, 在编译时就会被优化掉
// 如果我们想在 Vec<T> 上实现 fmt::Display, 那么就会遇到困难, 这样就可以通过 Wrapper 来实现
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 这里的 self.0 指的是 Wrapper 的第一个元素
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl OutlinePrint for Point {}

// 如果不实现 fmt::Display, 那么 OutlinePrint 也无法实现
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human(i32);

impl Pilot for Human {
    fn fly(&self) {
        println!("pilot fly");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("wizard fly");
    }
}

impl Human {
    fn fly(&self) {
        println!("human fly, but not very far");
    }
}

struct Point {
    x: i32,
    y: i32,
}

// 以下是系统内置的 trait, 用于重载运算符 +
// 其中 Rhs = Right hand side 是默认泛型参数, 指定了 + 运算符右边的类型, 意思是 + 两边的类型必须相同
// Rhs 的默认值是 self
// trait Add<Rhs = self> {
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

// 使用 Add trait 来重载 + 运算符
impl Add for Point {
    // type Output 指定了 Add trait 的关联类型, 该类型在实现 Add trait 时会被指定
    type Output = Point;
    // add 方法的参数是 self 和另一个 Point, 返回值是一个 Point
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 定义一个毫米单位的结构体, 是一个单元结构体, 里面只有一个字段
#[derive(Debug)]
struct Millimeters(u32);
// 定义一个米单位的结构体
struct Meters(u32);

// Rhs 不是 self 的情况, 重载 + 运算符
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    // 完成了 Millimeters 和 Meters 之间的相加运算
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

struct Counter {}

// 迭代器是一个 trait, 有一个 next 方法, 该方法会返回一个 Option<T> 类型的值
// 这里使用了 type Item; 定义了一个关联类型, 该类型在实现 Iterator trait 时会被指定
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(1)
    }
}

struct Counter2 {}

// 如果使用泛型来定义迭代器, 那么就需要在实现 Iterator trait 时指定 Item 类型
// 我们可以为 Counter2 实现多个 Iterator trait, 只要每个实现都指定了 Item 类型
// 与用关联类型定义的迭代器相比, 这样做的缺点是每个实现都需要指定 Item 类型
// 优点是可以为 Counter2 实现多个 Iterator trait, 每个实现都可以返回不同类型的值
pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}

impl Iterator2<String> for Counter2 {
    fn next(&mut self) -> Option<String> {
        Some(String::from("1"))
    }
}

impl Iterator2<u32> for Counter2 {
    fn next(&mut self) -> Option<u32> {
        Some(1)
    }
}

union MyUnion {
    f1: u32,
    f2: f32,
}

static HELLO_WORLD: &str = "Hello, world!";

// 静态变量是全局变量, 有固定的内存地址, 可以在任何作用域访问
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    // 访问和修改可变静态变量都是不安全的, 必须使用 unsafe 关键字
    // println!("COUNTER = {}", COUNTER);
    unsafe {
        COUNTER += inc;
    }
}

// trait 中至少有一个方法中包含编译器不能验证的不变式时的 trait 是不安全的
unsafe trait Foo {}

// 对于不安全的 trait, 实现该 trait 也必须标记为 unsafe
unsafe impl Foo for i32 {}

// 调用 c 语言的函数
extern "C" {
    fn abs(input: i32) -> i32;
}

// 允许其他语言调用 rust 函数
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// 用 unsafe 关键字实现 split_at_mut 函数
pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    // as_mut_ptr 方法返回一个指向 slice 首元素的裸指针
    let ptr = values.as_mut_ptr();
    // assert! 宏用于检查 mid 是否小于等于 len, 如果不是, 则会 panic
    assert!(mid <= len);
    // 这里用 unsafe 的好处是比其他安全的方式大大提高了性能
    // 虽然我们完全可以创建两个新的可变 slice, 分别指向原 slice 的两个部分, 但是这样会导致创建两个新的 slice 的开销
    unsafe {
        (
            // 0 到 mid 之间的元素的 slice 的可变引用
            slice::from_raw_parts_mut(ptr, mid),
            // mid 到 len 之间的元素 slice 的可变引用
            // add 为指针偏移方法, 用于计算出 mid 到 len 之间的元素的 slice 的裸指针
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

unsafe fn dangerous() {
    println!("i am so dangerous! be careful!")
}

// 智能指针是一类数据结构, 它们的表现类似指针, 但是也拥有额外的元数据和功能
// 有些智能指针为引用计数指针, 允许多个所有者

// String, Vec<T> 和 Box<T> 都是智能指针
// String 是一个 Vec<u8> 的包装器, 存储了其长度和分配的容量作为元数据, 并有额外的能力确保其数据始终是有效的 UTF-8 序列

// 智能指针通常用 struct 实现, 但智能指针不同于常规结构体, 因为他们实现了 Deref 和 Drop trait
// Deref trait 允许智能指针结构体实例表现得像引用一样, 这样就可以编写既用于引用, 又用于智能指针的代码
// Drop trait 允许我们自定义当智能指针离开作用域时运行的代码

// 最常用的智能指针有:
// Box<T> 用于在堆上分配值
// Rc<T> 允许多重所有权的引用计数类型, 只能用于单线程场景
// Ref<T> 和 RefMut<T> 通过 RefCell<T> 访问, 一个在运行时而不是在编译时执行借用规则的类型

// 内部可变性是 rust 中的一个设计模式, 它允许你即使在有不可变引用时也可以改变数据, 这通常是通过 unsafe 代码来实现的
// 引用循环是一种导致内存泄漏的 bug, 它会因为每个值的引用计数永远不为 0 而导致值永远不会被清理

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use List::{Cons, Nil};
use RcList::{RcCons, RcNil};
use RefCellList::{RefCellCons, RefCellNil};
use RefCellList2::{RefCellCons2, RefCellNil2};

fn main() {
    // 在堆上保存一个 i32 值
    // 回顾一下, 在栈上保存一个 i32 值的代码是这样的: let x = 5; 在 stack 上数据性能更好, 但是需要知道数据的大小
    // heap 上的数据可以在编译时不知道大小, 或者大小可能会变化, 性能上会差一些
    // b 本身是在 stack 上的, 但是数据 5 是在 heap 上的, 这就是 Box<T> 称为智能指针的原因
    let b = Box::new(5);
    println!("b = {}", b);

    // cons list 是一种常见的函数式编程数据结构, 它可以用于创建一个带有多个元素的列表, 但是 rust 标准库没有提供它
    // (1, (2, (3, None))) 这是一个 cons list, 它的类型是 (i32, (i32, (i32, ())))
    // cons list 每一项都包含两个元素 (当前值, 下一个元素), 最后一个元素的下一个元素是一个空值
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {:?}", list);

    let x = 5;
    let y = &x;
    println!("x = {}, y = {}", x, y);
    println!("*y = {}", *y);
    println!("5 == x: {}", x == 5);
    println!("5 == *y: {}", *y == 5);
    // y 是一个 &i32 类型的引用, 但是 5 是一个 i32 类型的值, 不能直接比较
    // *y 是 y 的解引用, 也就是 y 引用的值, 是一个 i32 类型的值, 可以直接比较
    // &5 是一个 i32 类型的引用, 可以与 y 直接比较
    // println!("5 == y: {}", y == 5);
    println!("&5 == y: {}", y == &5);

    let x = 5;
    let y = Box::new(x);
    println!("x = {}, y = {}", x, y);
    println!("*y = {}", *y);
    println!("5 == x: {}", x == 5);
    println!("5 == *y: {}", *y == 5);
    // y 是一个 Box<i32> 类型的智能指针, 但是 5 是一个 i32 类型的值, 不能直接比较
    // Box::from(5) 是一个 Box<i32> 类型的智能指针, 可以与 y 直接比较
    println!("5 == y: {}", y == Box::from(5));

    let x = 5;
    let y = MyBox::new(x);
    println!("x = {}, y = {:?}", x, y);
    // 因为 MyBox 实现了 Deref trait, 所以可以直接解引用, *y 实际上在底层访问了 *(y.deref())
    println!("*y = {}", *y);

    let m = MyBox::new(String::from("Rust"));
    // 因为 String 实现了 Deref trait, 所以可以直接解引用, *m 实际上在底层访问了 *(m.deref())
    println!("m = {:?}", m);
    hello(&m);
    // 如果 String 没有从 &String -> &str 的强制转换规则, 那只能这样写
    // (*m) 从 MyBox 获取 String, 再用 [..] 得到 String 的全量切片, 再转换为 &str
    hello(&(*m)[..]);

    // Deref trait 也可以用于可变引用, 但是这种情况下需要实现 DerefMut trait
    // rust 在发现类型和 trait 实现满足三种情况时会进行解引用强制转换
    // 1. 当 T: Deref<Target=U> 时从 &T 到 &U
    // 2. 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U
    // 3. 当 T: Deref<Target=U> 时从 &mut T 到 &U
    // 但是第三种情况只会在解引用的地方进行, 如果在其他地方需要 &mut T, 那么就会报错, 因为 &mut T 只能有一个
    // 可变引用转为不可变引用是总是可以的, 反之则不行, 因为不可变引用可以有多个, 但是可变引用只能有一个

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created. {} and {}", c.data, d.data);
    // 程序结束时会调用 drop 方法

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created. {} and {}", c.data, d.data);
    // drop 不允许显式调用, 但是可以调用 std::mem::drop, 已经在 prelude 中
    // c.drop();
    drop(c);
    // 被 drop 以后 c 就不能再使用了
    // println!("c: {}", c.data);
    println!("d: {}", d.data);

    // 尝试共享 Cons List 中第一个元素的所有权
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // 没有实现 Rc 的 Cons List 不能共享所有权
    // let c = Cons(4, Box::new(a));
    // 因为 a 的所有权已经被转移给 b 了, 所以 a 不能再使用
    // println!("a = {:?}, b = {:?}", a, b);
    println!("b = {:?}", b);

    // 用 Rc 实现 Cons List 的共享所有权
    // Rc 和 Box 的区别是, Box 只能有一个所有权, Rc 可以有多个所有权, Rc 要关注引用计数, Box 只关注值
    // 可以把 Box 当成 Rc 的特例, Rc 有且只有一个默认所有权的时就是 Box
    // 如果仅仅创建一个 RcCons 而不是 Rc::new(RcCons(...)), 那么这个引用计数就不会被适当地初始化, 这会导致运行时错误
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    // Rc::clone 会增加引用计数, 但是不会深拷贝数据, 所以速度很快
    // 使用 Rc 时的习惯是用 Rc::clone 而不是 a.clone, 因为 Rc::clone 名字表明了意图, 但 a.clone 也是可以的
    // 当查找代码中的性能问题时, 只需考虑深拷贝类的克隆而无需考虑 Rc::clone 调用
    let b = RcCons(3, Rc::clone(&a));
    let c = RcCons(4, Rc::clone(&a));
    println!("a = {:?}, b = {:?}, c = {:?}", a, b, c);

    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    // Rc::strong_count 返回 Rc 实例的引用计数, strong 表示引用计数是 strong 类型的
    // 对应 strong 还有 weak 类型的引用计数, weak 类型的引用计数不会影响值是否被清理
    // weak_count 只有在 Rc 实例被清理时才会减少, 所以 weak_count 可以用来检测 Rc 实例是否被清理
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        println!("c = {:?}", c);
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    println!("a = {:?}", a);
    println!("b = {:?}", b);

    let x = 5;
    // x 本身是不可变的, 因此 y 不能通过 &mut x 来改变 x 的值
    // let y = &mut x;
    println!("x = {}", x);
    // 如果一定要改变 x 的值, 那么可以使用 RefCell 实现内部可变性
    // 在测试时经常会面临这样的情况, 有一个不可变的结构体, 但是需要在测试中改变结构体的值

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(RefCellCons(Rc::clone(&value), Rc::new(RefCellNil)));
    let b = RefCellCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = RefCellCons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    // a, b, c 都是 RefCell 类型, 可以通过 borrow_mut 方法获取可变引用
    // *value 会从 Rc 中解引用, 然后因调用了 borrow_mut 从 RefCell 中自动解引用, 从而获取可变引用
    // a, b, c 的值都会被改变
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // RefCell<T> 不是线程安全的, 如果需要在多线程中使用, 需要使用 Mutex<T> 来实现内部可变性

    let a = Rc::new(RefCellCons2(5, RefCell::new(Rc::new(RefCellNil2))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    let b = Rc::new(RefCellCons2(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    if let Some(link) = a.tail() {
        // 将 a 的尾部指向 b, 完成循环引用
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // thread 'main' has overflowed its stack
    // fatal runtime error: stack overflow
    // 由于 a 和 b 互相引用, 他们的引用计数永远不会为 0, 从而导致内存泄漏
    // 这样会导致循环引用, 从而导致内存泄漏
    // println!("a next item = {:?}", a.tail());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    // 通过在双向引用中指定从子向父的引用为 Weak<T>, 可以避免循环引用导致的内存泄漏
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// 定义一个发信器 Messenger trait, 要求实现 send 方法
pub trait Messenger {
    fn send(&self, msg: &str);
}

// LimitTracker 限制追踪器, 用于追踪 Messenger 实例发送的消息数量, 并在消息数量达到上限时发送警告
// 其 messenger 是一个泛型 T
// T: 'a + Messenger 表示 T 必须实现 Messenger trait, 并且 T 的生命周期必须不短于 'a
// + 可以用于指定多个 trait bound, 生命周期参数必须在 trait bound 之前, 也可以视为一种特殊的 trait bound
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    // 限制追踪器的构造函数, 接受一个泛型 T 的实例, 并将其存储在 messenger 字段中
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    // 用于设置消息数量的方法
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        // 如果消息数量达到上限, 则发送警告
        let percentage_of_max = self.value as f64 / self.max as f64;
        match percentage_of_max {
            _ if percentage_of_max >= 1.0 => {
                let msg = format!(
                    "Error: You are over your quota! You used {} of {} units",
                    self.value, self.max
                );
                self.messenger.send(&msg);
            }
            _ if percentage_of_max >= 0.9 => {
                let msg = format!(
                    "Urgent warning: You've used up over 90% of your quota! You used {} of {} units",
                    self.value, self.max
                );
                self.messenger.send(&msg);
            }
            _ if percentage_of_max >= 0.75 => {
                let msg = format!(
                    "Warning: You've used up over 75% of your quota! You used {} of {} units",
                    self.value, self.max
                );
                self.messenger.send(&msg);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // 当 MockMessenger 的 send_messages 定义为 Vec<String> 时, 为不可变引用
            // 无法在 send 方法中修改 sent_messages 的值
            // self.sent_messages.push(String::from(msg));
            // 这时候应该用 RefCell 实现内部可变性
            // 可以用 borrow_mut 方法获取 RefCell<T> 的可变引用, 并在获取可变引用时进行借用检查
            self.sent_messages.borrow_mut().push(String::from(msg));

            // RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用
            // 已经在上面获取了可变引用, 所以这里不能再获取不可变引用, 否则编译时不会报错, 运行时会 panic
            // 这就是 unsafe 代码的危险之处, 编译器无法检查 unsafe 代码的安全性, 只能由程序员自己保证
            // let mut borrow = self.sent_messages.borrow_mut();
            // borrow.push(String::from("hello, world"));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // 创建一个 MockMessenger 实例
        let mock_messenger = MockMessenger::new();
        // 创建一个 LimitTracker 实例, 并将 mock_messenger 传入
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        // 调用 set_value 方法, 并传入 80
        limit_tracker.set_value(80);
        limit_tracker.set_value(81);
        limit_tracker.set_value(82);
        limit_tracker.set_value(83);
        // RefCell<T> 的 borrow 方法返回 Ref<T>, borrow_mut 方法返回 RefMut<T>
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 4);
        assert_eq!(
            mock_messenger.sent_messages.borrow()[0],
            "Warning: You've used up over 75% of your quota! You used 80 of 100 units"
        );
        println!("{:?}", mock_messenger.sent_messages.borrow());
    }
}

// 定义 MyBox 是一个包含 T 类型的元素的元组结构体
#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 实现 Deref trait, 使得 MyBox<T> 实例可以被解引用
impl<T> Deref for MyBox<T> {
    // 定义关联类型, 用于指定 Deref trait 将会返回的引用的类型
    type Target = T;
    // 实现 deref 方法, 返回一个引用, 这个引用指向 self 中数据的位置
    // &T 是一个引用, 而 self 是一个智能指针, 因此需要解引用两次才能得到 T 类型的值
    // &T 也可以写作 &Self::Target, 这样就不需要在这里指定关联类型
    fn deref(&self) -> &T {
        // &self.0 返回 MyBox<T> 中数据的引用, 因为 self 是一个元组结构体, 所以可以使用 .0 来访问元组中的第一个元素
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // 实现 Drop trait, 当实例离开作用域时会调用 drop 方法
    // rust 会在实例离开作用域时自动调用 drop 方法, 也可以使用 std::mem::drop 函数来强制调用 drop 方法
    // drop 的目标是释放 self, 所以要 &mut self
    fn drop(&mut self) {
        println!("--- Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}

#[derive(Debug)]
enum RefCellList {
    RefCellCons(Rc<RefCell<i32>>, Rc<RefCellList>),
    RefCellNil,
}

#[derive(Debug)]
enum RefCellList2 {
    RefCellCons2(i32, RefCell<Rc<RefCellList2>>),
    RefCellNil2,
}

impl RefCellList2 {
    // tail 用法获取当前的 List 实例的下一个元素
    fn tail(&self) -> Option<&RefCell<Rc<RefCellList2>>> {
        match self {
            RefCellCons2(_, item) => Some(item),
            RefCellNil2 => None,
        }
    }
}

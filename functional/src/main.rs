// 闭包可以保存在一个变量中, 或者作为参数传递给函数, 是一种匿名函数

use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // 获取赠送的衬衫颜色, 先考虑用户的喜好, 如果没有库存, 则获取当前库存最多的衬衫颜色
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    // 获取当前库存最多的衬衫颜色
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    // 设定一个库存状态: 两件蓝色衬衫, 一件红色衬衫
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    // 设定两个用户的喜好, None 为没有偏好
    let user1_preference = Some(ShirtColor::Red);
    let user2_preference = None;

    let give1 = store.giveaway(user1_preference);
    println!("user1 prefer: {:?}, get: {:?}", user1_preference, give1);
    let give2 = store.giveaway(user2_preference);
    println!("user2 prefer: {:?}, get: {:?}", user2_preference, give2);

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        // 1 second = 1,000 milliseconds = 1,000,000 microseconds = 1,000,000,000 nanoseconds
        thread::sleep(Duration::from_millis(100));
        num
    };
    println!("expensive_closure_res = {}", expensive_closure(5));

    // 函数和闭包类似, 但是闭包是匿名的
    // fn add_one_v1(x: u32) -> u32 {
    //     x + 1
    // }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x: u32| x + 1;

    let example_closure = |x| x;
    // 同一个闭包可以调用多次, 但不能调用不同类型的参数, 第二个类型开始的调用会报错
    let s1 = example_closure(String::from("hello"));
    let s2 = example_closure(String::from("hello2"));
    // let n = example_closure(5);
    println!("s1 = {:?}, s2 = {:?}", s1, s2);

    // 闭包有三种方式捕获其环境, 它们直接对应函数的三种获取参数的方式: 获取所有权, 可变借用和不可变借用

    let list = vec![1, 2, 3];
    println!("before defining closure, list = {:?}", list);
    let only_borrows = || println!("from closure: {:?}", list);
    println!("before call closure, list = {:?}", list);
    only_borrows();
    println!("after call closure, list = {:?}", list);

    let mut list = vec![1, 2, 3];
    println!("before defining closure, list = {:?}", list);
    let mut borrows_mut = || list.push(4);
    // 这时候 list 已经被闭包借用了, 所以不能再使用 list
    // println!("before call closure, list = {:?}", list);
    borrows_mut();
    // 闭包借用 list 完成后, list 又可以使用了
    println!("after call closure, list = {:?}", list);

    let list = vec![1, 2, 3];
    println!("before defining closure, list = {:?}", list);
    thread::spawn(move || println!("from closure: {:?}", list))
        .join()
        .unwrap();
    // 因为 thread::spawn 会获取所有权, 所以这时候 list 已经被闭包获取所有权了, 所以不能再使用 list
    // thread 包是一个线程库, 用于创建线程, join 方法用于等待线程结束
    // spawn 方法返回一个 JoinHandle, 它是一个线程句柄, 用于等待线程结束, 并获取线程的返回值
    // move 关键字用于强制闭包获取其使用的环境的所有权
    // println!("after call closure, list = {:?}", list);

    // 闭包捕获和处理环境中的值的方式影响闭包的类型, 闭包的类型由 Fn, FnMut 和 FnOnce 三种 trait 中的一种决定
    // 1. FnOnce 适用于能被调用一次的闭包, 所有闭包都至少实现了这个 trait, 因为所有闭包都能被调用
    // 一个会将捕获的值移出闭包体的闭包只实现 FnOnce trait, 这是因为它只能被调用一次
    // 2. FnMut 适用于不会将捕获的值移出闭包体的闭包, 但它可能会修改被捕获的值, 这类闭包可以被调用多次
    // 3. Fn 用于既不将被捕获的值移出闭包体也不修改被捕获的值的闭包, 当然也包括不从环境中捕获值的闭包
    // 这类闭包可以被调用多次而不改变它们的环境, 这在会多次并发调用闭包的场景中十分重要

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    println!("before sort, list = {:?}", list);
    // 因为 sort_by_key 方法需要多次调用闭包, 所以闭包需要实现 FnMut
    list.sort_by_key(|r| r.width);
    println!("after sort by width, list = {:?}", list);

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    println!("before sort, list = {:?}", list);
    let mut sort_operations = vec![];
    let value = String::from("by key called");
    list.sort_by_key(|r| {
        // 因为 sort_by_key 需要 FnMut 闭包, 所以 sort_operations.push(value) 会报错, 它会导致闭包为 FnOnce
        // 闭包捕获的值会被移动到闭包内部, 所以闭包只能被调用一次, 但 sort_by_key 需要多次调用闭包
        // 如果把 value 改为 value.clone() 则不会报错, 因为 clone 方法会返回一个新的值, 而不是移动原来的值, 所以闭包可以被调用多次
        // sort_operations.push(value);
        sort_operations.push(value.clone());
        r.height
    });
    println!("after sort by width, list = {:?}", list);

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // for 循环会自动调用 next 方法, 并在 next 方法返回 None 时结束循环
    // 如果需要 index, 可以使用 enumerate 方法, 它会返回一个元组, 元组的第一个元素是 index, 第二个元素是值
    let mut index = 0;
    for val in v1_iter {
        println!("index {}: get {}", index, val);
        index += 1;
    }

    let v1 = vec![1, 2, 3];
    // 迭代器是惰性的, 如果不消费是会报错的
    // v1.iter().map(|x| x + 1);
    // map 方法返回一个迭代器, 迭代器的元素是闭包的返回值
    let mut vm = v1.iter().map(|x| x + 1);
    // 因为值是 Option, 用 Some 包裹更好, None 正好会导致迭代器结束
    while let Some(val) = vm.next() {
        println!("get {}", val);
    }

    let v1 = vec![1, 2, 3];
    // Vec<_> 是一个占位符, 它告诉 rust 我们想要一个 Vec<T> 类型, 但是还不知道 T 是什么类型
    // collect 方法会尝试将迭代器转换为多种不同的集合类型, 所以有时候需要显式指定类型
    // 迭代器都是惰性的, 需要 collect 消费迭代器
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    // v1 所有权已经被迭代器获取, 所以不能再使用 v1
    // println!("v1 = {}", v1);
    println!("v2 = {:?}", v2);

    let v1 = vec![String::from("a"), String::from("b"), String::from("c")];
    // 这种情况下需要用 into_iter() 方法, 如果 iter() 方法, map 中的 |x| 是 &String 类型, 而不是 String 类型, x + "z" 会报错
    // let v2: Vec<_> = v1.iter().map(|x| x + "z").collect();
    let v2: Vec<_> = v1.into_iter().map(|x| x + "z").collect();
    println!("v2 = {:?}", v2);
}

#[derive(PartialEq, Debug)]
pub struct Shoe {
    size: u32,
    style: String,
}

pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter() 和 iter() 的区别是, iter() 会借用集合中的元素, 而 into_iter() 会获取集合的所有权
    // filter 方法会获取一个闭包作为参数, 闭包的参数是集合中的元素, 返回值是一个 bool 值, 如果闭包返回 true, 则元素会被保留, 否则会被丢弃
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Option 中的 unwrap_or_else 方法接受一个闭包作为参数, 如果 Option 是 Some 则返回 Some 中的值, 否则调用闭包
// 因为 F 为 FnOnce, 所以闭包只能被调用一次, 如果 Option 是 Some 则闭包不会被调用
// 可以接受绝大多数的闭包, 所有闭包都实现了 FnOnce
// impl<T> Option<T> {
//     pub fn unwrap_or_else<F>(self, f: F) -> T
//         where
//             F: FnOnce() -> T
//     {
//         match self {
//             Some(x) => x,
//             None => f(),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn iterator_demonstration() {
        let v = vec![1, 2, 3];
        // iter() 返回迭代器, 迭代器是惰性的, 只有使用 next 方法时才会产生值, 需要是可变引用
        // next 会 "消耗" 迭代器层数
        let mut v_iter = v.iter();
        assert_eq!(v_iter.next(), Some(&1));
        assert_eq!(v_iter.next(), Some(&2));
        assert_eq!(v_iter.next(), Some(&3));
        assert_eq!(v_iter.next(), None);
    }
    #[test]
    fn iterator_sum() {
        let v = vec![1, 2, 3];
        // 不使用 next() 等会改变状态的方法时, 可以使用不可变引用
        let v_iter = v.iter();
        let total: i32 = v_iter.sum();
        // sum() 会获取迭代器的所有权, 不能再使用 v_iter
        // v_iter.next();
        assert_eq!(total, 6);
    }
    #[test]
    fn filters_by_size() {
        // 预设一组 shoes 数据用于测试准备
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        // 获取 size 为 10 的鞋子
        let in_my_size = shoes_in_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

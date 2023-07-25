#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数, 不需要实例就可以调用, 通常用于构造器
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 为 Rectangle 定义多个 impl 块是允许的
impl Rectangle {
    // 方法, 第一个参数是 self, 代表实例本身, 也可以使用 &mut self 如果需要修改实例的话
    // &self 代表实例本身, 也可以使用 &mut self 如果需要修改实例的话
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let w = 30;
    let h = 50;
    println!("The area of the rectangle is {} square pixels.", area(w, h));

    // 因为 w 和 h 都是 Copy trait 类型的, 所以可以复制
    let rect = (w, h);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_2(rect)
    );
    println!("{} {}", w, h);

    // 使用结构体并实现方法
    let rect = Rectangle {
        width: w,
        height: h,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    // dbg! 宏可以打印出结构体的所有字段, 并打印出调用 dbg! 的代码所在行数和文件名
    // dbg! 输出到 stderr 而不是 stdout
    dbg!(&rect);
    println!("{:?}", rect);

    // dbg! 还会返回一个包含调试信息的值, 所以可以作为表达式的一部分
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50 * scale,
    };
    dbg!(&rect);

    let r1 = Rectangle {
        width: 30,
        height: 50,
    };
    let r2 = Rectangle {
        width: 10,
        height: 40,
    };
    let r3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can r1 hold r2? {}", r1.can_hold(&r2));
    println!("Can r1 hold r3? {}", r1.can_hold(&r3));

    let sq = Rectangle::square(3);
    println!("{:?}", sq);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// 用元组作为参数
fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

use rand::Rng;

// 硬币枚举
enum Coin {
    Penny,   // 1 分
    Nickel,  // 5 分
    Dime,    // 10 分
    Quarter, // 25 分
}

fn value_in_cent(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            // 打印出 Lucky penny! 后, 返回 1
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

#[derive(Debug)]
enum UsCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 相当于每个枚举成员都有一个关联的值, 这里体现的是每个 25 分硬币都有一个州的信息与之对应
}

fn us_value_in_cent(coin: UsCoin) -> u8 {
    match coin {
        UsCoin::Penny => 1,
        UsCoin::Nickel => 5,
        UsCoin::Dime => 10,
        UsCoin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let coins = (Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter);
    println!(
        "c1: {:?}\nc2: {:?}\nc3: {:?}\nc4: {:?}",
        value_in_cent(coins.0),
        value_in_cent(coins.1),
        value_in_cent(coins.2),
        value_in_cent(coins.3)
    );

    let c1 = UsCoin::Quarter(UsState::Alaska);
    let c2 = UsCoin::Quarter(UsState::Alabama);
    let c3 = UsCoin::Penny;
    let c4 = UsCoin::Nickel;
    let c5 = UsCoin::Dime;
    println!(
        "c1: {:?}\nc2: {:?}\nc3: {:?}\nc4: {:?}\nc5: {:?}",
        us_value_in_cent(c1),
        us_value_in_cent(c2),
        us_value_in_cent(c3),
        us_value_in_cent(c4),
        us_value_in_cent(c5)
    );

    let five = Some(5);
    let six = plus_one(five);
    println!("six: {:?}", six);
    let none = plus_one(None);
    println!("none: {:?}", none);
    if none.is_none() {
        println!("none is none");
    }

    let dice_roll = rand::thread_rng().gen_range(1..7);
    println!("dice_roll: {}", dice_roll);
    // res 的类型会自动根据 match 的分支类型推断出来, 即 res 的类型是 i32, 返回类型不一致会报错
    let res = match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other 为通配符, 匹配所有情况, 在需要通配值的情况时使用, 不需要时可以用 _ 代替
        // other 或 _ 的通配符可以放在任何位置, 但是通常放在最后, 因为 match 会从上到下匹配, 匹配到通配符后就会停止匹配
        other => move_player(other),
    };
    println!("res: {:?}", res);

    // Some(0u8) 中的 0u8 是一个 u8 类型的数字, 值为 0, 而 None 则不包含任何值
    let some_u8_value: Option<u8> = Some(0u8); // 因为指定了类型, 所以 Some(0u8) 也可以写成 Some(0)
    match some_u8_value {
        Some(0) => println!("zero"),
        Some(1) => println!("one"),
        Some(3) => println!("three"),
        Some(5) => println!("five"),
        Some(7) => println!("seven"),
        other => println!("other: {:?}", other),
    }
    // if let 的行为与上面的 match 表达式是等价的, 但是 if let 更简洁, 适用于只关心一个分支的情况
    // 这样会失去 match 的穷尽性检查特性
    if let Some(0) = some_u8_value {
        println!("zero");
    } else {
        // else 相当于 match 中的通配符, 匹配所有情况
        println!("other");
    }

    let mut count = 0;
    let coin = UsCoin::Quarter(UsState::Alaska);
    match coin {
        UsCoin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    // coin 不可以再使用, 因为 match 会获取所有权
    println!("count: {}", count);

    let mut count = 0;
    let coin = UsCoin::Quarter(UsState::Alaska);
    // 在使用 state 时加入了 ref 关键字, 这样就不会获取 state 的所有权, 只是借用了 state 的值
    if let UsCoin::Quarter(ref state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    // coin 在 if let 中只是被借用了, 所以可以继续使用
    println!("coin: {:?}", coin);
    println!("count: {}", count);
}

// Option 是允许我们用空值来代替不存在值的常见枚举, 即 x 可以是 i32 类型的数字, 也可以是 None
// rust 中的 None 与其他语言中的 null 或 nil 是不同的, 因为它不是一个指针, 它只是一个占位符, 代表没有值
// Some(i) 与 None 都是 Option<T> 类型的, 而 T 是泛型, 即 Some(i) 与 None 都可以是任何类型
// Some(i) 中的 i 是一个 i32 类型的数字, 而 None 则不包含任何值
fn plus_one(x: Option<i32>) -> Option<i32> {
    // match 匹配应该是穷尽的, 即 match 中的每个分支都要处理所有可能的情况, 如果不考虑 None 的情况, 编译器会报错
    match x {
        None => None,
        Some(i) => Some(i + 1),
        // _ => None, // _ 通配符, 匹配所有情况
    }
}

fn add_fancy_hat() -> i32 {
    1
}

fn remove_fancy_hat() -> i32 {
    2
}

fn move_player(num_spaces: u8) -> i32 {
    println!("move {} spaces", num_spaces);
    3
}

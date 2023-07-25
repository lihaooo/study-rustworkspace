use serde::{Deserialize, Serialize};
use serde_json;

// derive 是一个宏，它指示编译器自动实现指定的 trait
// Debug 用于打印调试信息, Serialize 用于序列化, Deserialize 用于反序列化
#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// 元组结构体
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

// 类单元结构体
#[derive(Debug)]
struct UnitStruct;

// User2 中的 username 和 email 不能使用 &str 类型, 因为 &str 类型是 slice 不具备所有权
// struct User2 {
// active: bool,
// username: &str,
// email: &str,
// sign_in_count: u64,
// }

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("some_username_123"),
        active: true,
        sign_in_count: 1,
    };
    let user1_json = match serde_json::to_string(&user1) {
        Ok(json) => json,
        Err(e) => panic!("Failed to serialize. {}", e),
    };
    // 将 user1_json 反序列化为 User 结构体, 相当于对 user1 进行了深拷贝, 与 clone() 类似
    let de_user1: User = match serde_json::from_str::<User>(&user1_json) {
        Ok(user) => user,
        Err(e) => panic!("Failed to deserialize. {}", e),
    };
    println!(
        "user1: {:?}\nuser1_json: {}\nuser1_from_json: {:?}",
        user1, user1_json, de_user1
    );
    user1.email = String::from("another_email@example.com");
    println!("user1: {:?}", user1);

    let user2 = build_user(String::from("user2"));
    println!("user2: {:?}", user2);

    let user3 = User {
        email: String::from("user3@example.com"),
        username: String::from("user3"),
        ..user2 // 使用 user2 的剩余字段
    };
    println!("user3: {:?}", user3);

    let user4 = User {
        email: user1.email,
        username: user1.username,
        active: false,
        sign_in_count: 0,
    };
    println!("user4: {:?}", user4);
    // user1 的 email 和 username 字段已经被移动到 user4 中, user1 无法再使用
    // 只有移动了非 Copy trait 的类型才会发生所有权转移
    // println!("user1: {:?}", user1);

    let user5 = User {
        email: user4.email.clone(),
        username: user4.username.clone(),
        active: false,
        sign_in_count: user4.sign_in_count,
    };
    println!("user5: {:?}", user5);
    // user4 的 email 和 username 是通过 clone() 方法进行深拷贝的, 所以 user4 仍然可以使用
    // user4 中的 sign_in_count 字段是 Copy trait 的类型, 所以 user4 仍然可以使用
    println!("user4: {:?}", user4);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {:?}, origin: {:?}", black, origin);

    let subject = UnitStruct; //
    println!("subject: {:?}", subject);
}

fn build_user(username: String) -> User {
    // 通过 clone() 方法对 username 字段进行深拷贝, 从而避免了所有权的转移, 拼接出 email 字段
    let mut email = username.clone();
    email.push_str("@example.com");
    User {
        email,
        username,
        active: true,
        sign_in_count: 2,
    }
}

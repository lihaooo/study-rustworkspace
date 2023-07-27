// cargo test -- --test-threads=1 可以指定测试用例并发运行的线程数
// -- 的意思是后面的参数是 cargo test 的参数, 而不是 cargo test 的参数的参数
// cargo test -- --show-output 可以让测试用例打印出 println! 的内容
// cargo test 的参数可以是测试用例的名字, 可以是测试用例的一部分名字, 可以是测试用例的一部分名字的一部分
// cargo test it_works 只会运行 *it_works* 的测试用例
// cargo test -- --ignored 只会运行被标记为 #[ignore] 的测试用例
// cargo test -- --include-ignored 会运行所有测试用例, 包括被标记为 #[ignore] 的测试用例
// cargo test --test integration_test 只会运行 tests/tests/integration_test.rs 中的测试用例

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo test 会运行所有标记为 #[test] 的函数作为测试用例集合
    #[test]
    // #[ignore] // 忽略测试用例, 只会在 cargo test -- --ignored 时运行, 常用于一些特别耗时的测试用例
    fn it_works() {
        let result = add(2, 2);
        // assert_eq! 宏会比较两个参数是否相等，如果不相等，会使用 debug 格式打印两个参数的值
        assert_eq!(result, 4);
        // assert_ne! 宏会比较两个参数是否不相等，如果相等，会使用 debug 格式打印两个参数的值
        // assert_ne!(result, 4);
    }
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    // #[should_panic] 标记表示测试用例应该会 panic, 如果测试用例没有 panic，测试用例会失败
    // expected 参数可以用来指定 panic 时的错误信息, panic 必须包含 expected 参数的值, 可以不完全相同
    #[should_panic(expected = "Make this test fail")]
    fn another() {
        panic!("Make this test fail");
        // panic!("Make this test"); // fail
    }
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        // assert! 宏会检查其参数是否为 true，如果不为 true，会使用 debug 格式打印其参数的值
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // 自定义错误信息, assert! 的后两个参数相当于 format! 宏的参数
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
    #[test]
    // 使用 Result<T, E> 作为测试函数的返回值, 如果测试函数返回 Err 值, 测试用例会失败
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    #[test]
    fn internal() {
        // rust 中的私有函数也可以被测试
        assert_eq!(4, internal_adder(2, 2));
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    // println!("name: {}", name);
    // String::from("Hello!")
}

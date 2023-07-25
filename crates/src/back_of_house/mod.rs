#[derive(Debug)]
// 枚举成员默认就是公有的, 所以不需要使用 pub 修饰, 但是枚举本身是私有的, 所以需要使用 pub 修饰
pub enum Appetizer {
    Soup,
    Salad,
}

#[derive(Debug)]
pub struct Breakfast {
    pub toast: String,
    pub seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        // 使用 pub 修饰的字段可以在外部访问
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

pub fn fix_incorrect_order() {
    // 本模块中的函数可以直接访问
    cook_order();
    // 使用 super 调用父模块中的函数
    super::serve_order();
}
fn cook_order() {}

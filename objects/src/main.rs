// 使用 trait 时, 编译器无法知晓所有可能用于 trait 的具体类型, 必须使用动态分发, 这会带来一些运行时的开销

// 如果一个 trait 中定义的所有方法都符合以下两个规则, 则 trait 可以认为是 object-safe 的
// 1. 返回值类型不为 Self
// 2. 方法没有任何泛型类型参数
// pub trait Clone
//     fn clone(&self) -> Self;
// }
// 就不是 object-safe 的, 因为返回值类型为 Self
// 如果我们尝试将 Clone trait 作为 trait 对象的类型, 则会得到一个错误
// pub struct Screen {
//     pub components: Vec<Box<dyn Clone>>,
// }

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();

    // String 没有实现 Draw trait, 所以不能将 String 值存储在 Screen 的 components 字段中
    // let screen = Screen {
    //     components: vec![Box::new(String::from("Hi"))],
    // };
    // screen.run();

    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    println!("post content: {}", post.content());
    post.request_review();
    println!("post content: {}", post.content());
    post.approve();
    println!("post content: {}", post.content());
    post.add_text("I ate a salad for lunch today");
    println!("post content: {}", post.content());
    post.request_review();
    println!("post content: {}", post.content());
    post.approve();
    println!("post content: {}", post.content());
}

// 以下是使用枚举 Post 状态的方式来实现
// #[derive(Debug)]
// enum State {
//     Draft,
//     PendingReview,
//     Published,
// }
//
// pub struct Post {
//     state: State,
//     content: String,
//     draft_content: String,
// }
//
// impl Post {
//     pub fn content(&self) -> &str {
//         &self.content
//     }
//     pub fn new() -> Post {
//         Post {
//             // 默认状态为 Draft
//             state: State::Draft,
//             content: String::new(),
//             draft_content: String::new(),
//         }
//     }
//     pub fn add_text(&mut self, text: &str) {
//         self.draft_content.push_str(text);
//     }
//     // 将状态从 Draft 改为 PendingReview
//     pub fn request_review(&mut self) {
//         if let State::Draft = self.state {
//             self.state = State::PendingReview;
//         }
//     }
//     // 将状态从 PendingReview 改为 Published
//     pub fn approve(&mut self) {
//         if let State::PendingReview = self.state {
//             self.state = State::Published;
//             self.content = self.draft_content.clone();
//             self.draft_content.clear();
//         }
//     }
// }

// 以下是使用 trait 对象的方式来实现, 额外多实现了允许在 Publish 后再次修改的功能, 修改后变为 Draft 状态
// 通过将 Post 的状态和行为封装到不同的类型中, 我们可以在不同的状态中实现不同的行为
trait State {
    // 为了在 content 方法中使用 self 参数, 我们使用了 &self 而不是 self
    // 这样就不会获取 self 的所有权, 因为 Post 需要在状态之间转换, 所以不能获取其所有权
    fn content<'a>(&self, _: &'a Post) -> &'a str {
        ""
    }
    fn add_text(self: Box<Self>, post: &mut Post, content: &str) -> Box<dyn State>;
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn content(&self) -> &str {
        // as_ref() 方法返回一个引用, 这样就可以在 content 方法中使用 self.state 而不会获取它的所有权
        self.state.as_ref().unwrap().content(self)
    }
    pub fn new() -> Post {
        // 默认状态为 Draft
        Post {
            state: Some(Box::new(DraftPost {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, content: &str) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.add_text(self, content));
        }
    }
    pub fn request_review(&mut self) {
        // take 方法从 state 字段中获取 Some 值并将其设置为 None, 这样就可以获取一个状态值并将其移出 Post, 以便 Post 可以转换到新状态而不会获取旧状态的所有权
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

struct DraftPost {}

impl State for DraftPost {
    fn add_text(self: Box<Self>, post: &mut Post, content: &str) -> Box<dyn State> {
        post.content.push_str(content);
        self
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReviewPost {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReviewPost {}

impl State for PendingReviewPost {
    fn add_text(self: Box<Self>, _: &mut Post, _: &str) -> Box<dyn State> {
        self
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PublishedPost {})
    }
}

struct PublishedPost {}

impl State for PublishedPost {
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn add_text(self: Box<Self>, post: &mut Post, content: &str) -> Box<dyn State> {
        post.content.push_str(content);
        Box::new(DraftPost {})
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// AveragedCollection 维护了一组 i32 类型的值, 并且可以计算平均值
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    // remove 方法从 self.list 中移除最后一个元素, 并返回它
    pub fn remove(&mut self) -> Option<i32> {
        // 如果 self.list 为空, 则返回 None
        if self.list.is_empty() {
            return None;
        }
        // 如果 self.list 不为空, 则返回最后一个元素
        let result = self.list.pop();
        // 如果 pop 后 self.list 不为空, 则更新平均值
        if !self.list.is_empty() {
            self.update_average();
        }
        // 返回最后一个元素
        result
    }
    pub fn average(&self) -> f64 {
        self.average
    }
    fn update_average(&mut self) {
        // 可以使用 self.list.iter() 来创建一个迭代器, 该迭代器会产生集合中的每一个元素的引用
        // 通过调用 sum 方法来对迭代器中的元素求和
        let total: i32 = self.list.iter().sum();
        // 通过将 total 转换为 f64 类型, 并除以 self.list.len() 来计算平均值
        self.average = total as f64 / self.list.len() as f64;
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Box<dyn Draw> 是一个 trait 对象, 我们可以将它理解为一个指向实现了 Draw trait 的类型的指针
    // dyn 是 dynamic 的缩写, 表示这是一个动态类型, 也就是说, 这个类型在运行时而不是编译时可知
    // 通过使用 trait 对象, 我们可以在运行时使用不同类型的值
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            // 调用 trait 对象上的方法
            component.draw();
        }
    }
}

// 使用 trait 定义共享行为比用泛型和具体类型更灵活, 因为 Screen 实例可以持有任何实现了 Draw trait 的类型的值
// 如果用泛型来实现 Screen, 则 Screen 的定义必须指定其使用的类型, 而不是允许调用者来指定他们希望使用的类型
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }
//
// impl<T> Screen<T>
//     where
//         T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "button draw, width: {}, height: {}, label: {}",
            self.width, self.height, self.label
        );
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "select box draw, width: {}, height: {}, options: {:?}",
            self.width, self.height, self.options
        );
    }
}

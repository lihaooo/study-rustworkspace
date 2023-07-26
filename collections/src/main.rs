fn main() {
    let v: Vec<i32> = Vec::new();
    println!("v: {:?}", v);

    // 可以用 vec! 宏来初始化, 会自动推断类型
    let v = vec![1, 2, 3];
    println!("v: {:?}", v);
    // 也可以指定类型
    let mut v: Vec<i32> = Vec::new();
    // insert 需要指定索引
    v.insert(0, 1);
    v.insert(1, 2);
    v.insert(2, 3);
    println!("v: {:?}", v);
    // 也可以用 push
    let mut v = Vec::new();
    // push 过程中会自动推断类型
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v: {:?}", v);

    let v = vec![1, 2, 3, 4, 5];
    let third: i32 = v[2];
    println!("third: {}", third);
    let third: &i32 = &v[2];
    println!("third: {}", third);
    let third: Option<&i32> = v.get(2);
    println!("third: {:?}", third);
    // panic: index out of bounds
    // let not_exist = v[100];
    // let not_exist = &v[100];
    // 这种情况会返回 None, 而不是 panic, 这种情况比 panic 要友好
    let not_exist: Option<&i32> = v.get(100);
    println!("not_exist: {:?}", not_exist);

    let mut v = vec![1, 2, 3, 4, 5];
    // 如果 first 是 &i32, 那么 v.push(6) 会导致 first 的引用失效, push 有可能会导致内存重新分配
    // let first = &v[0];
    // v.push(6);
    // println!("first: {}", first);

    // 如果 first 是 i32, 它会得到 v[0] 的值复制, v.push(6) 不会影响 first
    let first = v[0];
    v.push(6);
    println!("first: {}", first);

    let v = vec![100, 32, 57];
    for mut i in v {
        // i 是 v 的每个元素的复制, 不是引用, 所以可以修改 i, 但不会影响 v
        i += 1;
        println!("i: {}", i);
    }
    // 如果想要修改 v, 需要使用引用, 但不能直接使用 v 的引用, 因为 v 在 for 循环中已经被移动了

    let mut v = vec![100, 32, 57];
    // 使用 v[0] 的复制不会影响 v 的所有权
    let first = v[0];
    println!("first: {}", first);
    for i in &mut v {
        // i 是 v 的每个元素的可变引用
        // *i 是 i 的解引用, 也就是 v 的每个元素
        *i += 50;
        println!("i: {}", i);
    }
    println!("v: {:?}", v);

    // vec 只能储存同类型的值, 如果需要储存不同类型的值, 可以使用枚举
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue")),
    ];
    println!("row: {:?}", row);
    // 可以使用 match 来获取枚举的值
    match row.get(0) {
        Some(SpreadSheetCell::Int(i)) => println!("i: {}", i),
        _ => (),
    }
    match row.get(1) {
        Some(SpreadSheetCell::Float(i)) => println!("i: {}", i),
        _ => (),
    }
    match row.get(2) {
        Some(SpreadSheetCell::Text(i)) => println!("i: {}", i),
        _ => (),
    }
    // 不会对 row 进行所有权转移, 可以继续使用
    println!("row: {:?}", row);

    let s1 = String::from("initial contents");
    let data = "initial contents";
    let s2 = data.to_string();
    let s3 = "initial contents".to_string();
    println!("s1: {}\ns2: {}\ns3: {}", s1, s2, s3);

    // 字符串是 UTF-8 编码的, 所以可以包含任何字节
    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    println!("hello: {}", hello);

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!'); // 只能 push 一个字符, 必须使用单引号
    println!("s: {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // push_str 不会获取 s2 的所有权, 所以 s2 可以继续使用
    println!("s1 = {s1}, s2 = {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // 字符串的 + 运算符使用了 add 函数, fn add(self, s: &str) -> String
    // 我们用 &s2 相加, 因为 &String 可以被强制转换为 &str
    // 实质上 s3 获取了 s1 的所有权, 再把 s2 的值复制后附加到 s3, 所以并没有获取 s2 的所有权
    // 这个实现比拷贝要更高效, 因为没有复制 s1 的内容
    let s3 = s1 + &s2;
    // println!("s1: {}", s1); // s1 被移动了，不能继续使用
    println!("s2: {}, s3: {}", s2, s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;
    // 使用 format! 宏更简单, 而且不会获取任何参数的所有权, 但性能可能会差一些, 因为 s1, s2, s3 都会被复制
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}", s);
    println!("s1: {}, s2: {}, s3: {}", s1, s2, s3); // s1, s2, s3 没有被移动, 可以继续使用

    // String 的本质是 Vec<u8> 的封装, 并不是字符数组
    let s = String::from("hello");
    // 字符串不能直接使用索引访问, 会报错
    // let h = s[0];
    // &String 可以被强制转换为 &str, 所以可以使用切片
    let c1 = &s[0..1];
    let c2 = &s[1..2];
    println!("c1: {}, c2: {}", c1, c2);

    let hello = "Здравствуйте";
    // 在 vector 的 u8 值为 [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
    // 224, 165, 135]， 3 对应的是 224
    // 即便下文是获取字节值的有效代码, 返回的值也是 224 而不是 3
    // let answer = &hello[0]; // 只取第一个字节根本无法得到有效的字符, 会报错
    let s = &hello[0..4];
    // 这样得到的结果是 Зд, 因为 0..4 获取了 4 个字节, 对应的是 Зд 这两个字符
    println!("s: {}", s);

    // 遍历字面值字符串的字符
    for c in "Зд".chars() {
        println!("c: {}", c);
    }

    // 遍历字面值字符串的字节
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // HashMap 的键必须是可哈希的, 任何实现了 Eq 和 Hash trait 的类型都是可哈希的
    // HashMap 的值可以是任何类型, 但必须是同一种类型
    // Vector, String 和 HashMap 都是标准库提供的集合类型, 但 HashMap 最不常用, 所以 prelude 中没有包含, 需要用 use 引入
    // 和 Vector, String 一样, HashMap 的数据也是储存在堆上的， 所以 HashMap 也可以在编译时不知道数据的大小
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    println!("scores: {:?}", scores); // 已经实现了 Debug trait, 可以直接打印

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    let team_name = "Blue";
    // get 方法返回的是 Option<&V>, 所以需要使用 match 或者 unwrap
    // 如果 HashMap 中没有对应的值, get 方法会返回 None, 否则返回 Option<&V>
    // copied() 方法会返回一个 V 的复制, 如果 V 实现了 Copy trait, 那么就是 V 的复制, 否则就是 V 的引用
    // 本例中 copied() 返回的是 i32 的复制
    // unwrap_or() 方法会返回 Option 的值, 如果是 Some, 就返回 Some 的值, 如果是 None, 就返回 unwrap_or 的参数
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score: {}", score);
    let score = match scores.get(&team_name) {
        Some(&score) => score,
        None => 0,
    };
    println!("score: {}", score);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map: {:?}", map);
    // 这里 field_name 和 field_value 不再有效, 因为 String 被移动到了 HashMap 中
    // println!("field_name: {}, field_value: {}", field_name, field_value);

    let field_name = 1;
    let field_value = 100;
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map: {:?}", map);
    // 这里的 field_name 和 field_value 仍然有效, 因为 i32 实现了 Copy trait, 会被复制到 HashMap 中
    println!("field_name: {}, field_value: {}", field_name, field_value);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // 如果 key 不存在, 就插入, 如果存在, 就更新
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // entry 方法返回一个 Entry 枚举, 它代表了可能存在也可能不存在的值
    // 只在键不存在时插入, Entry 有 Vacant 和 Occupied 两种值, Vacant 表示键不存在, Occupied 表示键存在
    // or_insert 方法在键不存在时插入, 返回一个可变引用, 所以可以直接修改
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // 统计字符串中每个单词出现的次数
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // split_whitespace() 方法会返回一个迭代器, 迭代器会产生字符串的每个单词, 用空格分隔
    // split_whitespace() 不完全等价于 split(' '), 如果间隔了多个空格, split_whitespace() 会忽略
    for word in text.split_whitespace() {
        // 只在键不存在时插入, 键存在与否都返回一个可变引用
        let count = map.entry(word).or_insert(0);
        // or_insert 方法返回的是 Entry 的一个可变引用, 所以需要解引用,
        *count += 1;
    }
    println!("{:?}", map);

    // HashMap 使用了 SipHash 算法, 这个算法可以抵抗拒绝服务攻击, 但性能比较差
    // 如果需要性能, 可以使用 FxHash, 但它不会抵抗拒绝服务攻击, 或者使用 AHash, 它的性能和安全性都比较好
    // https://en.wikipedia.org/wiki/SipHash
}

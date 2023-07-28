//! # mini_grep
//!
//! ## Usage Example
//! ```bash
//! IGNORE_CASE=1 cargo run -- searchstring example-filename.txt
//! ```

use std::io::{Error, ErrorKind};
use std::{env, fs};

pub fn run(conf: Config) -> Result<(), Error> {
    println!(
        "ignore case: {}",
        if conf.ignore_case { "true" } else { "false" }
    );
    println!("searching for \"{}\" in \"{}\"\n", conf.query, conf.path);
    let contents = fs::read_to_string(conf.path)?;
    let results = search(conf.ignore_case, &conf.query, &contents)?;
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

/// Config 结构体, 用于存储命令行参数
///
/// # parameters
/// 1. query: 查询字符串
/// 2. path: 文件路径
/// 3. ignore_case: 是否忽略大小写, 默认为 false, 可以通过环境变量 IGNORE_CASE 设置
/// `ignore_case 为 1, true, TRUE, True 时为 true, 其他值为 false`
pub struct Config {
    query: String,
    path: String,
    ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Error> {
        if args.len() != 3 {
            Err(Error::new(
                ErrorKind::InvalidInput,
                "need 2 arguments, [query] [path]",
            ))?
        }
        // 跳过 args[0], 因为它是程序名
        let query = &args[1];
        let path = &args[2];
        // env::var 返回一个 Result, 如果环境变量不存在, 则返回 Err
        let ignore_case = match env::var("IGNORE_CASE") {
            Ok(val) => val == "1" || val == "true" || val == "TRUE" || val == "True",
            Err(_) => false,
        };
        // query 和 path 的所有权都在 parse_config 中, 所以需要 clone 才能返回
        Ok(Config {
            query: query.clone(),
            path: path.clone(),
            ignore_case,
        })
    }
    // 优化后的 new 方法, 使用迭代器取代性能不好的 clone
    pub fn iter_new(mut args: impl Iterator<Item = String>) -> Result<Config, Error> {
        let mut index = 0;
        let ignore_case = match env::var("IGNORE_CASE") {
            Ok(val) => val == "1" || val == "true" || val == "TRUE" || val == "True",
            Err(_) => false,
        };
        let mut cfg = Config {
            query: String::new(),
            path: String::new(),
            ignore_case,
        };
        while let Some(arg) = args.next() {
            match index {
                1 => cfg.query = arg,
                2 => cfg.path = arg,
                _ => Err(Error::new(
                    ErrorKind::InvalidInput,
                    "need 2 arguments, [query] [path]",
                ))?,
            }
            index += 1;
        }
        Ok(cfg)
    }
}

pub fn search<'a>(
    ignore_case: bool,
    query: &str,
    contents: &'a str,
) -> Result<Vec<&'a str>, Error> {
    let mut query = query.to_string();
    if ignore_case {
        query = query.to_lowercase();
    }
    let mut results = Vec::new();
    for line in contents.lines() {
        let mut search_line = line.to_string();
        if ignore_case {
            search_line = search_line.to_lowercase();
        }
        if search_line.contains(&query) {
            results.push(line);
        }
    }
    Ok(results)
}

// 使用迭代器改造 search, 用 filter 替代 for 循环, 用 collect 替代 Vec::new, 性能更好
pub fn filter_search<'a>(
    ignore_case: bool,
    query: &str,
    contents: &'a str,
) -> Result<Vec<&'a str>, Error> {
    let results = contents
        .lines()
        .filter(|line| {
            let mut search_line = line.to_string();
            let mut search_query = query.to_string();
            if ignore_case {
                search_line = search_line.to_lowercase();
                search_query = search_query.to_lowercase();
            }
            search_line.contains(&search_query)
        })
        .collect();
    Ok(results)
}

// 改造为 Config 结构体的 new 方法
// fn parse_config(args: &[String]) -> Config {
//     let query = &args[1];
//     let path = &args[2];
//     // query 和 path 的所有权都在 parse_config 中, 所以需要 clone 才能返回
//     Config {
//         query: query.clone(),
//         path: path.clone(),
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let args = vec![
            String::from("mini_grep"),
            String::from("searchstring"),
            String::from("example-filename.txt"),
        ];
        let conf = Config::new(&args).unwrap();
        assert_eq!(conf.query, "searchstring");
        assert_eq!(conf.path, "example-filename.txt");
    }

    #[test]
    #[should_panic(expected = "need 2 arguments, [query] [path]")]
    fn test_new_with_invalid_args() {
        let args = vec![String::from("mini_grep"), String::from("searchstring")];
        let _conf = Config::new(&args).unwrap();
    }

    #[test]
    fn test_search() {
        let query = "duct";
        // \ 是 Rust 的换行符, 表示下一行也是字符串的一部分
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let results = search(true, query, contents).unwrap();
        assert_eq!(results, vec!["safe, fast, productive."]);
    }
}

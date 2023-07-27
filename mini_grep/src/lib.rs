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
    if results.len() == 0 {
        Err(Error::new(
            ErrorKind::NotFound,
            format!("no results found for {query}"),
        ))?;
    }
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

    #[test]
    #[should_panic(expected = "no results found for searchstring")]
    fn test_search_with_invalid_query() {
        let query = "searchstring";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        // _ 开头的变量表示不使用该变量
        // unwrap() 在接受 Err 时会 panic, 正好用 should_panic 来测试
        let _results = search(true, query, contents).unwrap();
    }
}

extern crate clap;

use clap::{Arg, App};
use std::fmt;
use std::fs::File;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}
impl fmt::Display for Person<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name:{},age:{}", self.name, self.age)
    }
}

fn main() {
    let matches = App::new("command")
        .version("0.1")
        .author("jiangkundi")
        .about("Learn use Rust Crate!")
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("verbosity level"))
        .args_from_usage("-p, --path=[FILE] 'Target file you want to write'")
        .args_from_usage("-n, --name=[String] 'The name for person'")
        .args_from_usage("-a, --age=[u8] 'The age for peron'")
        .get_matches();

    let mut path: &str = "";
    let mut name: &str = "";
    let mut age: u8 = 0;
    if let Some(f) = matches.value_of("path") {
        path = f;
    }
    if let Some(n) = matches.value_of("name") {
        name = n;
    }
    if let Some(a) = matches.value_of("age") {
        age = a.parse().unwrap();
    }

    let person = Person {name, age};
    println!("display person: {}",person);
    // 创建文件
    let filepath = Path::new(path);
    let display = filepath.display();
    let mut file = match File::create(&filepath) {
        Err(_why) => {
            panic!("couldn't create {}",
                           display)
        },
        Ok(file) => file,
    };

    // Person对象转为JSON字符串.
    let serialized = serde_json::to_string(&person).unwrap();
    println!("serialized = {}", serialized);
    // 写入数据
    match file.write_all(serialized.as_bytes()) {
        Err(_why) => {
            panic!("couldn't write to {}", display)
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

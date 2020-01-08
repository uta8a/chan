use std::env;
use std::io::{Write, BufWriter};
use std::fs;

fn main() {
    let mut path = env::current_dir().unwrap();
    path.pop();
    if path.ends_with("uta8alib") == false {
        println!("Error: cannot found directory uta8alib/");
        panic!();
    }
    path.push("src");
    let args:Vec<String> = env::args().collect();
    if args.len() == 2 {
        // compile
        
    }else if args.len() == 4 {
        // gen lib
        let gen = args[1].as_str();
        let lang = args[2].as_str();
        match gen {
            "g" | "generate" => {
                match lang {
                    "r" | "rust" => {
                        path.push("rust");
                        path.push(args[3].as_str());
                        match fs::create_dir(path.clone()) {
                            Ok(_) => {},
                            Err(e) => {
                                println!("{}", e);
                                println!("Error: Folder already Exists");
                                panic!();
                            }
                        }
                        path.push("main.rs");
                        let template = include_str!("../lang-template/rust/main.rs");
                        let mut f = BufWriter::new(fs::File::create(path).unwrap());
                        f.write(template.as_bytes()).unwrap();
                    },
                    "c" | "cpp" => {
                        path.push("cpp");
                        path.push(args[3].as_str());
                        match fs::create_dir(path.clone()) {
                            Ok(_) => {},
                            Err(e) => {
                                println!("{}", e);
                                println!("Error: Folder already Exists");
                                panic!();
                            }
                        }
                        path.push("main.cpp");
                        let template = include_str!("../lang-template/cpp/main.cpp");
                        let mut f = BufWriter::new(fs::File::create(path).unwrap());
                        f.write(template.as_bytes()).unwrap();
                    },
                    _ => {
                        println!("Error: select valid language, rust `r` or cpp `c`");
                    }
                }
            },
            _ => {
                println!("Error: second arg is `g` or `generate`");
            }
        }
    }else{
        println!("Error: arg is 1 or 3 words");
    }
}
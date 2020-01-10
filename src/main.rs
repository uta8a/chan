use std::env;
use std::io::{Write, BufWriter};
use std::fs;
use std::fs::OpenOptions;
use std::process::Command;

fn main() {
    
    let args:Vec<String> = env::args().collect();
    if args.len() == 2 {
        // compile?
        // wip
        let file = args[1].as_str();
        if file.ends_with("rs") {
            let output = Command::new("rustc")
                .arg("--color=always")
                .arg("main.rs")
                .output()
                .expect("failed to execute rustc process");
            let out = output.stdout;
            let err = output.stderr;
            println!("Out: {}", std::str::from_utf8(&out).unwrap());
            println!("Err: {}", std::str::from_utf8(&err).unwrap());
        }
        if file.ends_with("cpp") {
            let output = Command::new("g++")
                .arg("-std=gnu++1y")
                .arg("-Wall")
                .arg("-g")
                .arg("-fdiagnostics-color=always")
                .arg("-fsanitize=undefined")
                .arg("-D_GLIBCXX_DEBUG")
                .arg("-o")
                .arg("main")
                .arg("main.cpp")
                .output()
                .expect("failed to execute cpp process");
            let out = output.stdout;
            let err = output.stderr;
            println!("Out: {}", std::str::from_utf8(&out).unwrap());
            println!("Err: {}", std::str::from_utf8(&err).unwrap());
        }
        
    }else if args.len() == 4 {
        // gen lib
        let gen = args[1].as_str();
        let lang = args[2].as_str();
        let mut path = env::current_dir().unwrap();
        if path.ends_with("uta8alib") == false {
            println!("Error: cannot found directory uta8alib/");
            panic!();
        }
        let mut test_path = path.clone();
        path.push("src");
        test_path.push("test");
        match gen {
            "g" | "generate" => {
                match lang {
                    "r" | "rust" => {
                        let name = args[3].as_str();
                        path.push("rust");
                        path.push(name);
                        test_path.push("rust");
                        test_path.push(name);
                        match fs::create_dir(path.clone()) {
                            Ok(_) => {},
                            Err(e) => {
                                println!("{}", e);
                                println!("Error: folder(src) already Exists");
                                panic!();
                            }
                        }
                        match fs::create_dir(test_path.clone()) {
                            Ok(_) => {},
                            Err(e) => {
                                println!("{}", e);
                                println!("Error: folder(test) already Exists");
                                panic!();
                            }
                        }
                        path.push("main.rs");
                        test_path.push("main.rs");
                        let template = include_str!("../lang-template/rust/main.rs");
                        let mut f = BufWriter::new(fs::File::create(path).unwrap());
                        f.write(template.as_bytes()).unwrap();
                        let mut test_f = BufWriter::new(fs::File::create(test_path).unwrap());
                        test_f.write(template.as_bytes()).unwrap();
                    },
                    "c" | "cpp" => {
                        let name = args[3].as_str();
                        let mut include_path = path.clone();
                        include_path.push("cpp/include/lib.hpp");
                        path.push("cpp");
                        path.push(name);
                        test_path.push("cpp");
                        test_path.push(name);
                        match fs::create_dir(path.clone()) {
                            Ok(_) => {},
                            Err(e) => {
                                println!("{}", e);
                                println!("Error: folder(src) already Exists");
                                panic!();
                            }
                        }
                        match fs::create_dir(test_path.clone()) {
                            Ok(_) => {},
                            Err(e) => {
                                println!("{}", e);
                                println!("Error: folder(test) already Exists");
                                panic!();
                            }
                        }
                        path.push("main.hpp");
                        test_path.push("main.cpp");
                        let mut f = OpenOptions::new().append(true).write(true).open(include_path).unwrap();
                        write!(&mut f, "{}\n", format!(r#"#include "../{}/main.hpp""#, name)).unwrap();
                        let template = include_str!("../lang-template/cpp/main.cpp");
                        let mut f = BufWriter::new(fs::File::create(path).unwrap());
                        f.write(template.as_bytes()).unwrap();
                        let mut test_f = BufWriter::new(fs::File::create(test_path).unwrap());
                        test_f.write(template.as_bytes()).unwrap();
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
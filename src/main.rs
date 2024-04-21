use std::path::Path;

const ANSI_RED: &str = "\x1b[31m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_YELLOW: &str = "\x1b[33m";
const ANSI_BLUE: &str = "\x1b[34m";
const ANSI_PURPLE: &str = "\x1b[35m";
const ANSI_CYAN: &str = "\x1b[36m";

fn ansi(idx: u32) -> &'static str {
    match idx % 5 {
        0 => ANSI_RED,
        1 => ANSI_GREEN,
        2 => ANSI_YELLOW,
        3 => ANSI_CYAN,
        _ => ANSI_PURPLE,
    }
}

fn main() {
    let mut args = std::env::args().collect::<Vec<_>>();

    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        println!("\x1b[1;32mCodelist Help");
        println!("{}      -h, --help\x1b[0m           Shows this command", ANSI_BLUE);
        println!("\x1b[1;34m      -n  --no-hide\x1b[0m        Shows hidden files and folders");
        return;
    }

    let mut nohide = false;
    if args.contains(&"--no-hide".to_string()) || args.contains(&"-n".to_string()) {
        nohide = true;
        args.remove(args.iter().position(|x| *x == "--no-hide".to_string() || *x == "-n".to_string()).unwrap());
    }

    let path = match args.get(1) {
        Some(path) => path,
        None => {
            &".".to_string()
        }
    };

    print_children(path, 0, nohide);
}

fn print_children(path: &String, level: usize, nohide: bool) {
    if Path::new(path).is_file() {
        return;
    }

    let children = std::fs::read_dir(path).unwrap().map(|x| x.unwrap()).collect::<Vec<_>>();

    for c in children {
        let mut s = "   ".repeat(level).to_string();
        match c.path().is_dir() {
            true => {
                if c.file_name().to_str().unwrap().starts_with(".") && !nohide {
                    // println!("Hid {}", c.file_name().to_str().unwrap());
                    continue;
                }
                s.push_str(format!("{}{}", ANSI_BLUE, c.file_name().to_str().unwrap()).as_str());
                output(s);
                print_children(&c.path().to_str().unwrap().to_string(), level + 1, nohide);
            },
            false => {
                if c.file_name().to_str().unwrap().starts_with(".") && !nohide {
                    // println!("Hid {}", c.file_name().to_str().unwrap());
                    continue;
                }
                let lines = match std::fs::read_to_string(c.path()) {
                    Ok(e) => e,
                    Err(_) => continue,
                }.lines().count();
                s.push_str(format!("{}{} {}LOC {}b", ansi(level as u32), c.file_name().to_str().unwrap(), lines, c.metadata().unwrap().len()).as_str());
                output(s)
            },
        }
    }
}

fn output(data: String) {
    println!("{}", data);
    print!("\x1B[0m");
}
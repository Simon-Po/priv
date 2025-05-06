use std::env;
use std::fs;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::str::FromStr;
#[derive(Debug, PartialEq)]
enum Bultin {
    Echo,
    Exit,
    Type,
    Pwd,
    Cd,
}

impl std::str::FromStr for Bultin {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "echo" => Ok(Bultin::Echo),
            "exit" => Ok(Bultin::Exit),
            "type" => Ok(Bultin::Type),
            "pwd" => Ok(Bultin::Pwd),
            "cd" => Ok(Bultin::Cd),
            _ => Err(()),
        }
    }
}

fn main() {
    let mut current_working_dir = std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string();
    loop {
        print!("$ ");

        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let (command, rest) = match input.trim().split_once(" ") {
            Some((cmd, rest)) => (cmd, rest),
            None => (input.trim(), ""),
        };
        let exes = ExePath::setup_path();

        match command {
            "type" => Bultin::handle_type(rest, &exes.exes),
            "exit" => Bultin::handle_exit(rest),
            "echo" => Bultin::handle_echo(rest),
            "pwd" => Bultin::handle_pwd(&current_working_dir),
            "cd" => Bultin::handle_cd(&mut current_working_dir, rest.to_string()),
            _ => {
                //check if programm is available
                if exes.find(command.to_string()) {
                    let exe = exes
                        .exes
                        .iter()
                        .find(|exe| exe.command == command)
                        .expect("Not found");

                    let _full_path = format!("{}/{}", exe.location, exe.command);
                    let mut all_args = vec![];
                    all_args.extend(rest.split_whitespace());
                    let output = Command::new(&exe.command)
                        .args(&all_args)
                        .output()
                        .expect("Could not start process");
                    print!("{}", String::from_utf8(output.stdout).expect("Not utf-8"))
                } else {
                    println!("{}: command not found", input.trim())
                }
            }
        }
    }
}

#[derive(Debug)]
struct Exe {
    command: String,
    location: String,
    //full_path: String,
}
impl Exe {
    fn new(command: String, location: String) -> Exe {
        //  let full_path = Path::new(&location)
        // .join(&command)
        //.to_string_lossy()
        //.to_string();
        Exe {
            command,
            location,
            //       full_path,
        }
    }
}

struct ExePath {
    exes: Vec<Exe>,
}
impl ExePath {
    fn setup_path() -> ExePath {
        let path_strings: Vec<String> = get_path("PATH");
        let mut executables: Vec<Exe> = vec![];
        for dir in path_strings.iter() {
            let path = Path::new(&dir);

            if let Ok(result) = fs::read_dir(path) {
                for entry in result.flatten() {
                    executables.push(Exe::new(
                        String::from(entry.file_name().to_str().unwrap()),
                        String::from(path.to_str().unwrap()),
                    ));
                }
            }
        }
        ExePath { exes: executables }
    }
    fn find(&self, s: String) -> bool {
        self.exes.iter().any(|exe| exe.command == s)
    }
}

fn get_path(key: &str) -> Vec<String> {
    let mut path_strings: Vec<String> = vec![];
    match env::var_os(key) {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                if let Some(path_str) = path.to_str() {
                    path_strings.push(path_str.to_string());
                }
            }
        }
        None => println!("{key} is not defined in the environment."),
    }
    path_strings
}
impl Bultin {
    fn is_builtin(string: &str) -> bool {
        Bultin::from_str(string).is_ok()
    }
    fn handle_cd(cwd: &mut String, rest: String) {
        // check if valid path
        let path = Path::new(rest.trim());

        match std::fs::exists(path) {
            Ok(true) => *cwd = rest,
            Ok(false) => println!("cd: {}: No such file or directory", path.to_string_lossy()),
            Err(_) => eprintln!("Error while checking fileSystem could not be defined"),
        }
    }
    fn handle_pwd(current_working_dir: &String) {
        println!("{}", current_working_dir)
    }
    fn handle_type(rest: &str, exes: &[Exe]) {
        let (code_str, _) = match rest.split_once(" ") {
            Some((cmd, rest)) => (cmd, rest),
            None => (rest, ""),
        };
        if Bultin::is_builtin(code_str) {
            println!("{} is a shell builtin", code_str)
        } else if exes.iter().any(|exe| exe.command == code_str) {
            if let Some(result_exe) = exes.iter().find(|exe| exe.command == code_str) {
                println!(
                    "{} is {}/{}",
                    result_exe.command, result_exe.location, result_exe.command
                );
            }
        } else {
            println!("{}: not found", code_str);
        }
    }
    fn handle_echo(content: &str) {
        println!("{}", content);
    }

    fn handle_exit(rest: &str) {
        if rest.is_empty() {
            std::process::exit(0);
        }
        let (code_str, _) = match rest.split_once(" ") {
            Some((cmd, rest)) => (cmd, rest),
            None => (rest, ""),
        };
        let code = code_str.parse::<i32>().unwrap();

        std::process::exit(code)
    }
}

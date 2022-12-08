use std::{collections::HashMap};



pub fn solve_first(input: String) -> String {
    let mut cwd: Vec<&str>= Vec::new();
    let mut directories: HashMap<String, u64> = HashMap::new();
    directories.insert(String::from(""), 0);
    for command in input.split("$ ").collect::<Vec<&str>>()[1..].iter() {
        let lines = command.split('\n').collect::<Vec<&str>>();
        let cmd = lines[0].split(' ').collect::<Vec<&str>>();
        match cmd[0] {
            "cd" => {
                match cmd[1] {
                    ".." => {
                        cwd.pop();
                    },
                    "/" => {
                        cwd = Vec::new();
                    },
                    _ => {
                        cwd.push(cmd[1]);
                        directories.insert(cwd.join("/"), 0);
                    }
                }
            },
            "ls" => {
                for l in lines[1..].iter() {
                    let splitted = l.split(' ').collect::<Vec<&str>>();
                    if splitted[0] == "dir" || splitted[0].is_empty() {
                        continue;
                    }
                    let filesize: u64 = splitted[0].parse().unwrap();
                    for i in 0..cwd.len()+1 {
                        let c = cwd[..i].join("/");
                        let dir = directories.get_mut(&c).unwrap();
                        *dir += filesize;
                    }
                }
            },
            _ => panic!("Invalid command {:?}", cmd)
        }
    }
    directories.iter()
        .filter(|(_, size)| **size <= 100000)
        .map(|(_, size)| size).sum::<u64>().to_string()
}
pub fn solve_second(input: String) -> String {
    let mut cwd: Vec<&str>= Vec::new();
    let mut directories: HashMap<String, u64> = HashMap::new();
    directories.insert(String::from(""), 0);
    for command in input.split("$ ").collect::<Vec<&str>>()[1..].iter() {
        let lines = command.split('\n').collect::<Vec<&str>>();
        let cmd = lines[0].split(' ').collect::<Vec<&str>>();
        match cmd[0] {
            "cd" => {
                match cmd[1] {
                    ".." => {
                        cwd.pop();
                    },
                    "/" => {
                        cwd = Vec::new();
                    },
                    _ => {
                        cwd.push(cmd[1]);
                        directories.insert(cwd.join("/"), 0);
                    }
                }
            },
            "ls" => {
                for l in lines[1..].iter() {
                    let splitted = l.split(' ').collect::<Vec<&str>>();
                    if splitted[0] == "dir" || splitted[0].is_empty() {
                        continue;
                    }
                    let filesize: u64 = splitted[0].parse().unwrap();
                    for i in 0..cwd.len()+1 {
                        let c = cwd[..i].join("/");
                        let dir = directories.get_mut(&c).unwrap();
                        *dir += filesize;
                    }
                }
            },
            _ => panic!("Invalid command {:?}", cmd)
        }
    }
    let mut dirs = directories.iter().collect::<Vec<(&String, &u64)>>();
    dirs.sort_by_key(|(_,f)| **f);
    let need_space = directories.get("").unwrap() - 40000000;
    dirs.iter().find(|(_, f)| **f > need_space).unwrap().1.to_string()
}
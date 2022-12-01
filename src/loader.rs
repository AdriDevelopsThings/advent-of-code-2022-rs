use std::{path::PathBuf, fs::{File, self}, io::Write};

pub struct Cache {
    pub path: PathBuf,
    pub session: Session
}

pub struct Session {
    pub session_token: String
}

impl Cache {
    pub fn new(path: PathBuf, session: Session) -> Self {
        Self {
            path,
            session
        }
    }

    fn add(&self, key: &str, content: String) {
        let mut file = File::create(self.path.join(key)).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    fn get(&self, key: &str) -> Option<String>{
        let path = self.path.join(key);
        if path.exists() {
            Some(fs::read_to_string(self.path.join(key)).unwrap())
        } else {
            None
        }
    }

    pub fn get_puzzle_input(&self, name: &str) -> String {
        let cached = self.get(name);
        match cached {
            Some(content) => content,
            None => {
                let content = self.session.get_puzzle_input(name).unwrap();
                self.add(name, content.clone());
                content
            }
        }
    }
}

impl Session {
    fn get_puzzle_input(&self, name: &str) -> Result<String, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let res = client
            .get(format!("https://adventofcode.com/2022/day/{name}/input"))
            .header("cookie", format!("session={}", self.session_token))
            .header("user-agent", "adventofcode@adridoesthings.com")
            .send()?;
        res.error_for_status_ref().expect("Eror while request");
        Ok(String::from(res.text().unwrap().trim()))
    }
}
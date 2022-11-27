pub struct Session {
    pub session_token: String
}

impl Session {
    pub fn get_puzzle_input(&self, name: &str) -> Result<String, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let res = client
            .get(format!("https://adventofcode.com/2022/day/{name}/input"))
            .header("cookie", format!("session={}", self.session_token)).send()?;
        res.error_for_status_ref().expect("Eror while request");
        res.text()
    }
}
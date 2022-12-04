struct Range(u8, u8);

impl From<&str> for Range {
    fn from(input: &str) -> Self {
        let splitted = input.split('-').collect::<Vec<&str>>();
        Self(splitted[0].parse().unwrap(), splitted[1].parse().unwrap())
    }
}

impl Range {
    fn contains(&self, value: u8) -> bool {
        self.0 <= value && self.1 >= value
    }

    fn is_in(&self, other: &Range) -> bool {
        self.0 >= other.0 && self.1 <= other.1
    }

    fn overlaps(&self, other: &Range) -> bool {
        other.contains(self.0) || other.contains(self.1)
    }
}

pub fn solve_first(input: String) -> String {
    input.split('\n')
        .map(|i| i.split(',').collect::<Vec<&str>>())
        .map(|v| (Range::from(v[0]), Range::from(v[1])))
        .filter_map(|(r1, r2)| if r1.is_in(&r2) || r2.is_in(&r1) { Some(1) } else { None })
        .sum::<u32>()
        .to_string()
}

pub fn solve_second(input: String) -> String {
    input.split('\n')
        .map(|i| i.split(',').collect::<Vec<&str>>())
        .map(|v| (Range::from(v[0]), Range::from(v[1])))
        .filter_map(|(r1, r2)| if r1.overlaps(&r2) || r2.overlaps(&r1) { Some(1) } else { None })
        .sum::<u32>()
        .to_string()
}
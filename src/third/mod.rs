const CHARACTERS: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Clone, Copy, Debug)]
struct Item(char);

impl From<char> for Item {
    fn from(input: char) -> Self {
        Self(input)
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Item {
    fn is_uppercase(&self) -> bool {
        self.0.is_uppercase()
    }

    fn priority(&self) -> u32 {
        u32::from(CHARACTERS.chars().position(|c| c == self.0.to_lowercase().collect::<Vec<char>>()[0]).unwrap() as u8 + 1 + match self.is_uppercase() {
            true => 26,
            false => 0
        })
    }
}
struct Rucksack(Vec<Item>, Vec<Item>);

impl From<&str> for Rucksack {
    fn from(input: &str) -> Self {
        let compartments = input.chars()
            .collect::<Vec<char>>()
            .chunks(input.len() / 2)
            .map(|c| c.iter().map(|ic| Item::from(*ic)).collect::<Vec<Item>>())
            .collect::<Vec<Vec<Item>>>();
        Self(compartments[0].clone(), compartments[1].clone())
    }
}

impl Rucksack {
    fn get_common_item(&self) -> Option<&Item> {
        for item in self.0.iter() {
            for item_2 in self.1.iter() {
                if item == item_2 {
                    return Some(item);
                }
            }
        }
        None
    }

    fn items(&self) -> Vec<Item> {
        let mut items = self.0.clone();
        items.append(&mut self.1.clone());
        items
    }
    
    fn contains(&self, item: &Item) -> bool {
        self.0.contains(item) || self.1.contains(item)
    }
}

fn parse_rucksacks(input: String) -> Vec<Rucksack> {
    input.split('\n').map(Rucksack::from).collect::<Vec<Rucksack>>()
}

pub fn solve_first(input: String) -> String {
    parse_rucksacks(input).iter().map(|r| r.get_common_item().unwrap().priority()).sum::<u32>().to_string()
}

pub fn solve_second(input: String) -> String {
    parse_rucksacks(input).chunks(3).map(|r| {
            for rucksack in r.iter() {
                for item in rucksack.items().iter() {
                    if r[0].contains(item) && r[1].contains(item) && r[2].contains(item) {
                        return Some(*item);
                    }
                }
            }
            None
        }
    ).map(|i| i.unwrap().priority()).sum::<u32>().to_string()
}
struct ElveInventory(Vec<u32>);

impl ElveInventory {
    fn new(input: &str) -> Self {
        Self(input.split('\n').map(|x| x.parse::<u32>().unwrap_or_else(|_| panic!("{x} is invalid"))).into_iter().collect())
    }

    fn sum(&self) -> u32 {
        let mut s = 0;
        for calorie in self.0.iter() {
            s += calorie;
        }
        s
    }
}

fn generate_input(input: String) -> Vec<ElveInventory> {
    let mut elves: Vec<ElveInventory> = Vec::new();
    for i in input.split("\n\n") {
        if !i.is_empty() && i != " "{
            elves.push(ElveInventory::new(i));
        }
    }
    elves
}

pub fn solve_first(input: String) -> String {
    let elves = generate_input(input);
    
    let mut best_categories = 0;
    for elve in elves {
        let sum = elve.sum();
        if sum > best_categories {
            best_categories = sum
        }
    }
    best_categories.to_string()
}

pub fn solve_second(input: String) -> String {
    let mut elves = generate_input(input);
    elves.sort_by_key(|a| a.sum());
    elves.reverse();
    let best_categories = elves.get(0).unwrap().sum() + elves.get(1).unwrap().sum() + elves.get(2).unwrap().sum();
    best_categories.to_string()
}
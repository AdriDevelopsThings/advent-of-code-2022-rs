pub fn solve_first(input: String) -> String {
    input.split('\n').map(|l| {
        let c = l.chars().collect::<Vec<char>>();
        'option: for i in 0..l.len() - 4 {
            let subc = &c[i..i+4];
            for c in subc {
                if subc.iter().filter(|lc| **lc == *c).count() > 1 {
                    continue 'option;
                }
            }
            return (i + 4) as u32;
        }
        panic!("No marker: {l}");
    }).sum::<u32>().to_string()
}

pub fn solve_second(input: String) -> String {
    input.split('\n').map(|l| {
        let c = l.chars().collect::<Vec<char>>();
        'option: for i in 0..l.len() - 14 {
            let subc = &c[i..i+14];
            for c in subc {
                if subc.iter().filter(|lc| **lc == *c).count() > 1 {
                    continue 'option;
                }
            }
            return (i + 14) as u32;
        }
        panic!("No marker: {l}");
    }).sum::<u32>().to_string()
}
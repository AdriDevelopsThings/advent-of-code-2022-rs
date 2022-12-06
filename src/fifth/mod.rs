#[derive(Clone, Debug)]
struct Crate(String);

impl From<&str> for Crate {
    fn from(input: &str) -> Self {
        Self(String::from(input).replace('[', "").replace(']', ""))
    }
}

#[derive(Clone, Debug)]
struct Stack(Vec<Crate>);
impl Stack {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, c: Crate) {
        self.0.push(c);
    }

    fn pop(&mut self) -> Crate {
        self.0.pop().unwrap()
    }
}


pub fn solve_first(input: String) -> String {
    let splitted = input.split("\n\n").collect::<Vec<&str>>();
    let lines = splitted[0].split('\n').collect::<Vec<&str>>();
    let last_line = lines.last().unwrap().as_bytes();
    let stacks_size = (last_line[last_line.len() - 2] as char).to_string().parse::<u8>().unwrap();
    let mut stacks = vec![Stack::new(); stacks_size as usize];
    for line in lines[..lines.len() - 1].iter() {
        let newline = (String::from(" ") + line).replace("    ", " [x]");
        let crates = newline.split(' ').enumerate();
        for (i, c) in crates {
            if !c.is_empty() && c != "[x]" {
                stacks.get_mut(i - 1).unwrap().0.push(Crate::from(c));
            }
            
        }
    }
    for stack in stacks.iter_mut() {
        stack.0.reverse();
    }


    let instructions = splitted[1].split('\n');
    for i in instructions {
        /*
            instruction[0] = 'move'
            instruction[1] = count
            instruction[2] = 'from'
            instruction[3] = from
            instruction[4] = 'to'
            instruction[5] = to
        */
        let instruction = i.split(' ').collect::<Vec<&str>>();
        if instruction.len() != 6 {
            continue;
        }
        assert_eq!(instruction[0], "move");
        assert_eq!(instruction[2], "from");
        assert_eq!(instruction[4], "to");

        let count = instruction[1].parse::<u8>().unwrap();
        let from = instruction[3].parse::<u8>().unwrap() - 1;
        let to = instruction[5].parse::<u8>().unwrap() - 1;

        for _ in 0..count {
            let c = stacks.get_mut(from as usize).unwrap().pop();
            stacks.get_mut(to as usize).unwrap().push(c);
        }
        
    }
    stacks.iter().map(|s| s.0.last().unwrap().0.clone()).collect::<Vec<String>>().join("")
}

pub fn solve_second(input: String) -> String {
    input
}
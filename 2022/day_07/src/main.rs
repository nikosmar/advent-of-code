use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

struct Node {
    name: String,
    size: usize,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(name: String, size: usize) -> Self {
        Node {
            name,
            size,
            children: vec![],
        }
    }

    fn add_child(&mut self, child: Rc<RefCell<Node>>) {
        self.children.push(child.clone());
        self.size += child.borrow().size;
    }

    fn find_child(&self, directory: &String) -> Option<Rc<RefCell<Node>>> {
        for child in &self.children {
            if child.borrow().name == *directory {
                return Some(child.clone());
            }
        }

        None
    }

    fn calculate_size(&mut self) -> usize {
        let mut my_size = 0;

        if self.children.is_empty() {
            my_size = self.size;
        } else {
            for child in &self.children {
                my_size += child.borrow_mut().calculate_size();
            }
        }

        self.size = my_size;
        my_size
    }

    fn part_a(&self) -> usize {
        if self.children.is_empty() {
            return 0;
        }

        let mut size = if self.size <= 100_000 { self.size } else { 0 };

        for child in &self.children {
            size += child.borrow_mut().part_a();
        }

        size
    }

    fn part_b(&self, space_to_free: usize) -> Option<usize> {
        if self.children.is_empty() || self.size < space_to_free {
            return None;
        }
        let mut minimum: Option<usize> = Some(self.size);

        for child in &self.children {
            if let Some(child_size) = child.borrow_mut().part_b(space_to_free) {
                if space_to_free <= child_size {
                    match minimum {
                        Some(min) => {
                            if child_size <= min {
                                minimum = Some(child_size);
                            }
                        }
                        None => minimum = Some(child_size),
                    }
                }
            }
        }

        minimum
    }
}

fn navigate(commands: &Vec<Vec<String>>, mut pc: usize, dir: Rc<RefCell<Node>>) -> usize {
    let working_dir: Rc<RefCell<Node>> = dir;

    while pc < commands.len() {
        let command = &commands[pc];

        if &command[0] == "$" {
            if &command[1] == "cd" {
                if &command[2] == ".." {
                    return pc;
                } else if let Some(cd) = working_dir.borrow().find_child(&command[2]) {
                    pc = navigate(commands, pc + 1, cd);
                }
            }
        } else if command[0] == "dir" {
            working_dir
                .borrow_mut()
                .add_child(Rc::new(RefCell::new(Node::new(command[1].clone(), 0))));
        } else {
            working_dir
                .borrow_mut()
                .add_child(Rc::new(RefCell::new(Node::new(
                    command[1].clone(),
                    command[0].parse().unwrap(),
                ))));
        }

        pc += 1;
    }

    pc
}

fn main() {
    let file = File::open("../input_files/07_terminal.txt").unwrap();
    let reader = BufReader::new(file);

    let root = Rc::new(RefCell::new(Node::new("/".to_string(), 0)));
    let mut input = vec![];

    for line in reader.lines() {
        let tmp_line = line.unwrap();
        let command: Vec<String> = tmp_line.split(' ').map(|x| x.to_string()).collect();
        input.push(command);
    }

    navigate(&input, 1, root.clone());
    root.borrow_mut().calculate_size();

    println!("Part A: {}", root.borrow().part_a());

    let free_space = 70_000_000 - root.borrow().size;
    println!("Part B: {:?}", root.borrow().part_b(30_000_000 - free_space));
}

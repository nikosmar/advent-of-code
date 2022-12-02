use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

#[derive(Default)]
struct Cave {
    id: String,
    children: Vec<Rc<RefCell<Cave>>>,
    small: bool,
    visits: u32,
}

impl Cave {
    fn create(id: String, is_small: bool) -> Cave {
        Cave {
            id,
            children: vec![],
            small: is_small,
            visits: 0,
        }
    }

    fn add_child(&mut self, child: Rc<RefCell<Cave>>) {
        self.children.push(child);
    }
}

fn part_one(root: &Rc<RefCell<Cave>>) {
    if root.borrow().small {
        match root.try_borrow_mut() {
            Ok(mut cave) => cave.visits += 1,
            Err(_) => return,
        }
    }

    if root.borrow().id == "end" {
        return;
    }

    for child in &root.borrow().children {
        part_one(child);
    }

    if root.borrow().small {
        if let Ok(mut cave) = root.try_borrow_mut() {
            cave.visits -= 1
        }
    }
}

fn part_two(root: &Rc<RefCell<Cave>>, mut visit_small_twice: bool) {
    if root.borrow().small {
        match root.try_borrow_mut() {
            Ok(mut cave) => cave.visits += 1,
            // already visited at least once thus it's impossible to borrow the root mutably
            Err(_) => {
                if visit_small_twice && root.borrow().id != "start" {
                    visit_small_twice = false;
                } else {
                    return;
                }
            }
        }
    }

    if root.borrow().id == "end" {
        return;
    }

    for child in &root.borrow().children {
        part_two(child, visit_small_twice);
    }

    if root.borrow().small {
        if let Ok(mut cave) = root.try_borrow_mut() {
            cave.visits -= 1
        }
    }
}

fn main() {
    let file = File::open("12_caves.txt").unwrap();
    let reader = BufReader::new(file);

    let mut caves: HashMap<String, Rc<RefCell<Cave>>> = HashMap::new();

    for line in reader.lines() {
        let tmp = line.unwrap();
        let mut path = tmp.split('-');

        if let (Some(from), Some(to)) = (path.next(), path.next()) {
            if !caves.contains_key(from) {
                caves.insert(
                    from.to_string(),
                    Rc::new(RefCell::new(Cave::create(
                        from.to_string(),
                        from.chars().all(char::is_lowercase),
                    ))),
                );
            }
            if !caves.contains_key(to) {
                caves.insert(
                    to.to_string(),
                    Rc::new(RefCell::new(Cave::create(
                        to.to_string(),
                        to.chars().all(char::is_lowercase),
                    ))),
                );
            }

            caves[&from.to_string()]
                .borrow_mut()
                .add_child(caves[&to.to_string()].clone());
            caves[&to.to_string()]
                .borrow_mut()
                .add_child(caves[&from.to_string()].clone());
        }
    }

    part_one(&caves[&"start".to_string()]);
    println!("Part One: {}", caves[&"end".to_string()].borrow().visits);

    caves[&"end".to_string()].borrow_mut().visits = 0;

    part_two(&caves[&"start".to_string()], true);
    println!("Part Two: {}", caves[&"end".to_string()].borrow().visits);
}

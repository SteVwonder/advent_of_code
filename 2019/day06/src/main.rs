use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::cell::{RefCell};
use std::rc::Rc;

#[derive(Debug)]
struct SpaceObject {
    name: String,
    orbited_by: HashMap<String, Rc<RefCell<Self>>>,
}

impl SpaceObject {
    fn new(name: &String) -> Self {
        SpaceObject {
            name: name.clone(),
            orbited_by: HashMap::new(),
        }
    }

    fn insert_orbit(&mut self, orbiter: Rc<RefCell<Self>>) -> bool{
        self.orbited_by.insert(orbiter.borrow().name.clone(), orbiter.clone()).is_some()
    }

    fn sum_of_distances(&self, dist_from_center: u32) -> u32 {
        dist_from_center
            + self
                .orbited_by
                .values()
                .map(|x| x.borrow().sum_of_distances(dist_from_center + 1))
                .sum::<u32>()
    }
}

struct OrbitMap {
    center_of_mass: Rc<RefCell<SpaceObject>>,
    orbiters: HashMap<String, Rc<RefCell<SpaceObject>>>,
}

impl OrbitMap {
    pub fn new() -> Self {
        let com = Rc::new(RefCell::new(SpaceObject::new(&"COM".to_string())));
        OrbitMap {
            center_of_mass: com.clone(),
            orbiters: [("COM".to_string(), com)].iter().cloned().collect(),
        }
    }

    fn find(&self, name: &String) -> Option<Rc<RefCell<SpaceObject>>> {
        self.orbiters.get(name).map(|obj| obj.clone())
    }

    fn find_or_insert(&mut self, name: &String) -> Rc<RefCell<SpaceObject>> {
        if self.orbiters.get(name).is_none() {
                let new = Rc::new(RefCell::new(SpaceObject::new(name)));
                self.orbiters.insert(
                    name.to_string(),
                    new
                );
        }
        self.find(name).unwrap()

        /*
        match self.orbiters.get(name) {
            Some(x) => x.clone(),
            None => {
                let new = Rc::new(RefCell::new(SpaceObject::new(name)));
                self.orbiters.insert(
                    name.to_string(),
                    new.clone()
                );
                new.clone()
            }
        }.clone().as_ref().borrow_mut()
         */
        //.map(|obj| obj.as_ref().borrow_mut())
    }

    fn insert(&mut self, orbitee: &String, orbiter: &String) {
        let orbiter_obj = self.find_or_insert(orbiter);
        let orbitee_obj = self.find_or_insert(orbitee);
        if orbitee_obj.as_ref().borrow_mut().insert_orbit(orbiter_obj) {
            panic!(
                "Orbitee {:?} already has entry for orbiter {:?}",
                orbitee, orbiter
            )
        }
    }

    pub fn sum_of_distances(&self) -> u32 {
        self.center_of_mass.as_ref().borrow().sum_of_distances(0)
    }
}

fn build_orbits_from_file(path: &Path) -> OrbitMap {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    build_orbits_from_lines(reader.lines().filter_map(Result::ok).collect())
}

fn build_orbits_from_lines(lines: Vec<String>) -> OrbitMap {
    let mut orbits = OrbitMap::new();
    for line in lines.iter() {
        let mut split_line = line.split(")").map(|x| x.to_string());
        let orbitee = split_line.next().expect("Blank line in file");
        let orbiter = split_line.next().expect("Malformed input, missing ')'");
        orbits.insert(&orbitee, &orbiter);
    }
    orbits
}

fn main() {
    let orbits = build_orbits_from_file(&Path::new("./input"));
    println!("Part 1: {}", orbits.sum_of_distances());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provided_example() {
        let orbits = build_orbits_from_file(&Path::new("./test"));
        assert!(orbits.sum_of_distances() == 42);
        let com = orbits.center_of_mass.as_ref().borrow_mut();
        let mut com_orbits = com.orbited_by.iter();
        assert!(com_orbits.len() == 1);
        let (b_name, b) = com_orbits.next().unwrap();
        assert!((b_name == "B"));
        assert!((b.as_ref().borrow().name == "B"));
    }

    #[test]
    fn out_of_order_example() {
        let file = File::open(&Path::new("./test")).expect("Failed to open file");
        let reader = BufReader::new(file);
        let mut lines = reader.lines().filter_map(Result::ok).collect::<Vec<String>>();
        lines.reverse();
        let orbits = build_orbits_from_lines(lines);
        println!("{}", orbits.sum_of_distances());
        {
            let com = orbits.center_of_mass.as_ref().borrow_mut();
            let mut com_orbits = com.orbited_by.iter();
            assert!(com_orbits.len() == 1);
            let (b_name, b) = com_orbits.next().unwrap();
            assert!((b_name == "B"));
            assert!((b.as_ref().borrow().name == "B"));
        }
        assert!(orbits.sum_of_distances() == 42);
    }
}

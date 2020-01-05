use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug)]
struct SpaceObject {
    name: String,
    //orbits: Box<Option<&'b SpaceObject>> where,
    //orbited_by: HashSet<'b SpaceObject> where 'a : 'b,
    orbited_by: HashMap<String, SpaceObject>,
}

impl SpaceObject {
    fn new(name: &String) -> Self {
        SpaceObject {
            name: name.clone(),
            //orbits: Box::new(None),
            orbited_by: HashMap::new(),
        }
    }

    fn find(&mut self, name: &String) -> Option<&mut SpaceObject> {
        if self.name == *name {
            Some(self)
        } else {
            self.orbited_by.iter_mut().filter_map(|(_, orbiter)| orbiter.find(name)).next()
        }
    }

    fn insert_orbit(&mut self, orbiter: SpaceObject) -> Option<SpaceObject>{
        self.orbited_by.insert(orbiter.name.clone(), orbiter)
    }
}

struct OrbitMap {
    center_of_mass: SpaceObject,
    //orbiters: HashMap<String, SpaceObject>,
}

impl OrbitMap {
    fn new() -> Self {
        OrbitMap {
            center_of_mass: SpaceObject::new(&"COM".to_string()),
            //orbiters: HashMap::new(),
        }
    }

    fn insert(&mut self, orbitee: &String, orbiter: &String) {
        let orbitee = self.center_of_mass.find(orbitee).unwrap();
        let orbiter_obj = SpaceObject::new(orbiter);
        match orbitee.insert_orbit(orbiter_obj) {
            Some(x) => panic!("Orbitee {:?} already has entry for orbiter {:?}", orbitee, x),
            None => {},
        };
    }
}

fn build_orbits_from_file(path: Path) -> OrbitMap{
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut orbits = OrbitMap::new();
    for line in reader.lines().filter_map(Result::ok) {
        let mut split_line = line.split(")").map(|x| x.to_string());
        let orbitee = split_line.next().unwrap();
        let orbiter = split_line.next().unwrap();
        orbits.insert(&orbitee, &orbiter);
    }
    orbits
}

fn main() {
    let mut orbits = build_orbits_from_file(Path::new("./input"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provided_example() {
        let mut orbits = build_orbits_from_file(Path::new("./test"));
        let com_orbits = orbits.center_of_mass.orbited_by;
        assert!(com_orbits.len(), 1);
        let b = com_orbits.iter().next().name;
        assert!(b, "B");
    }
}

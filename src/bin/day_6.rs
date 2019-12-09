use std::collections::HashMap;
use std::default::Default;
use std::io::{self, BufRead};

fn main() {
    let mut map: HashMap<&str, BodyOfMass> = HashMap::new();

    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .filter_map(|l| {
            let s = l.unwrap().trim().to_string();
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        })
        .collect();

    for line in lines.iter() {
        let mut pieces = line.split(")");
        let parent_name = pieces.next().unwrap();
        let child_name = pieces.next().unwrap();

        {
            let parent = map.entry(parent_name).or_default();
            parent.orbiters.push(child_name);
        }

        {
            let child = map.entry(child_name).or_default();
            child.parent = Some(parent_name);
        }
    }

    println!("Part 1: {}", count_orbits(&map));
    println!("Part 2: {}", count_transfers(&map, "YOU", "SAN").unwrap());
}

fn count_orbits(map: &HashMap<&str, BodyOfMass>) -> u32 {
    let mut orbits = 0;

    for bom in map.values() {
        orbits += bom.get_parents(&map).len() as u32;
    }

    orbits
}

fn count_transfers(map: &HashMap<&str, BodyOfMass>, src: &str, dest: &str) -> Result<u32, Error> {
    let src_bom = map.get(src).unwrap();
    let dest_bom = map.get(dest).unwrap();

    for (i, src_p) in src_bom.get_parents(&map).iter().enumerate() {
        for (j, dest_p) in dest_bom.get_parents(&map).iter().enumerate() {
            if src_p == dest_p {
                return Ok((i + j) as u32);
            }
        }
    }

    Err(Error::NoIntersection)
}

#[derive(Debug)]
enum Error {
    NoIntersection,
}

struct BodyOfMass<'a> {
    parent: Option<&'a str>,
    orbiters: Vec<&'a str>,
}

impl<'a> BodyOfMass<'a> {
    fn new() -> BodyOfMass<'a> {
        BodyOfMass {
            parent: None,
            orbiters: Vec::new(),
        }
    }

    fn get_parents(&self, map: &'a HashMap<&str, BodyOfMass>) -> Vec<&'a str> {
        let mut parents = Vec::new();

        let mut parent = self.parent;
        while let Some(parent_name) = parent {
            parents.push(parent_name);
            let p_bom = map
                .get(parent_name)
                .expect(&format!("{} doesn't exist?", parent_name));

            parent = p_bom.parent;
        }

        parents
    }
}

impl<'a> Default for BodyOfMass<'a> {
    fn default() -> Self {
        BodyOfMass::new()
    }
}

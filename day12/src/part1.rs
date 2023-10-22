use pathfinding::prelude::*;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    io,
    rc::Rc,
};

fn main() {
    let lines = io::stdin().lines();

    let mut exits: Vec<(String, String)> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let mut rooms = line.split('-');

        let room1 = rooms.next().unwrap();
        let room2 = rooms.next().unwrap();

        if room1 != "end" && room2 != "start" {
            exits.push((room1.into(), room2.into()));
        }
        if room1 != "start" && room2 != "end" {
            exits.push((room2.into(), room1.into()));
        }
    }

    println!("{:?}", exits);

    let start = "start".to_string();
    let end = "end".to_string();
    let visited: RefCell<HashSet<String>> = RefCell::new(HashSet::new());

    fn is_small(room: &String) -> bool {
        &room.to_lowercase() == room
    }

    let visitable = |room: &String| -> bool {
        if is_small(room) {
            !visited.borrow().contains(room)
        } else {
            true
        }
    };

    let find_exits = |room: &String| {
        println!("{room}");
        exits
            .iter()
            .filter(|(entry, _)| entry == room && visitable(room))
            .map(|(_, exit)| {
                let is_small = exit.clone() == exit.to_lowercase();
                if is_small {
                    visited.borrow_mut().insert(exit.clone());
                }
                exit.clone()
            })
            .collect::<Vec<String>>()
    };

    let paths = count_paths(start, find_exits, |r| r == &end);

    dbg!(paths);
}

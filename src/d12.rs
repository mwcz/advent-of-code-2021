//! A solution to day 12 year 2021.
//! https://adventofcode.com/2021/day/12

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    io,
};

type Model = Vec<(String, String)>;
type Answer = usize;

pub fn parse(input: String) -> Model {
    let mut exits: Vec<(String, String)> = Vec::new();

    for line in input.lines() {
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

    exits
}

pub fn part1(exits: Model) -> Answer {
    let start = "start".to_string();
    let end = "end".to_string();
    // let discovered: RefCell<HashSet<Vec<String>>> = RefCell::new(HashSet::new());

    let mut paths: HashSet<Vec<String>> = HashSet::new();
    // discovered.borrow_mut().insert(start.clone());

    let q = RefCell::new(vec![vec![start.clone()]]);
    let mut path: Vec<String>;

    let is_small = |room: &String| -> bool {
        let is_lower = &room.to_lowercase() == room;
        let is_start = room == &start;

        is_lower && !is_start
    };

    let visitable = |room: &String, path: &Vec<String>| -> bool {
        let room_is_small = is_small(room);
        let room_is_discovered = path.contains(room);
        !(room_is_small && room_is_discovered)
    };

    let find_exits = |room: &String, path: &Vec<String>| {
        exits
            .iter()
            .filter(|(entry, exit)| entry == room && visitable(exit, path))
            // .inspect(|(entry, exit)| println!("  {entry} -> {exit} is visitable"))
            .map(|(_, exit)| {
                // let is_small = exit.clone() == exit.to_lowercase() && exit != "end";
                // if is_small {
                //     discovered.borrow_mut().insert(exit.clone());
                // }
                exit.clone()
            })
            .collect::<Vec<String>>()
    };

    // let paths = count_paths(start, find_exits, |r| r == &end);

    // dbg!(paths);

    while !q.borrow().is_empty() {
        path = q.borrow_mut().pop().unwrap();
        // println!("{}", path.join(", "));
        let exits = find_exits(path.last().unwrap(), &path);
        // println!("  valid exits: {}", exits.join(", "));
        for exit in exits {
            let mut newpath = path.clone();
            newpath.push(exit.clone());
            if exit == end {
                paths.insert(newpath);
            } else {
                q.borrow_mut().push(newpath.clone());
            }
        }
    }

    paths.len()
}

pub fn part2(exits: Model) -> Answer {
    let start = "start".to_string();
    let end = "end".to_string();
    let mut paths: HashSet<Vec<String>> = HashSet::new();

    let q = RefCell::new(vec![vec![start.clone()]]);
    let mut path: Vec<String>;

    let is_small = |room: &String| -> bool {
        let is_lower = &room.to_lowercase() == room;
        let is_start = room == &start;

        is_lower && !is_start
    };

    let visitable = |room: &String, path: &Vec<String>| -> bool {
        let room_is_small = is_small(room);

        // a room in path can be revisited if each small room has only been visited once, so get
        // the small rooms from path and count the occurrences of each room
        let small_rooms = path.iter().filter(|r| is_small(r));
        let mut small_room_counts: HashMap<String, usize> = HashMap::new();
        for room in small_rooms {
            let count = small_room_counts.entry(room.clone()).or_insert(0);
            *count += 1;
        }

        let can_revisit = room_is_small && !small_room_counts.values().any(|&c| c > 1);

        // println!("  path: {}", path.join(", "));
        // println!("  can revisit: {}", can_revisit);

        let room_is_discovered = path.contains(room);

        can_revisit || !(room_is_small && room_is_discovered)
    };

    let find_exits = |room: &String, path: &Vec<String>| {
        exits
            .iter()
            .filter(|(entry, exit)| entry == room && visitable(exit, path))
            .map(|(_, exit)| exit.clone())
            .collect::<Vec<String>>()
    };

    while !q.borrow().is_empty() {
        path = q.borrow_mut().pop().unwrap();
        let exits = find_exits(path.last().unwrap(), &path);
        for exit in exits {
            let mut newpath = path.clone();
            newpath.push(exit.clone());
            if exit == end {
                paths.insert(newpath);
            } else {
                q.borrow_mut().push(newpath.clone());
            }
        }
    }

    paths.len()
}

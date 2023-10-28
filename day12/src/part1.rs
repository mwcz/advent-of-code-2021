use std::{cell::RefCell, collections::HashSet, io};

fn main() {
    // read file specified in args[1] or if none is specified, read from stdin
    let lines: String = std::env::args()
        .nth(1)
        .map(|f| std::fs::read_to_string(f).unwrap())
        .or_else(|| {
            Some(
                std::io::stdin()
                    .lines()
                    .map(|line| line.unwrap())
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
        })
        .unwrap();

    let mut exits: Vec<(String, String)> = Vec::new();

    for line in lines.lines() {
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

    println!("Map: {exits:?}");

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
        println!("{}", path.join(", "));
        let exits = find_exits(path.last().unwrap(), &path);
        println!("  valid exits: {}", exits.join(", "));
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

    println!("path count: {}", paths.len());
}

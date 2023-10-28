use pathfinding::prelude::dijkstra;

fn main() {
    let input: String = std::env::args()
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

    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let start: (usize, usize) = (0, 0);
    let end: (usize, usize) = (grid.len() - 1, grid[0].len() - 1);

    let path = dijkstra(
        &start,
        |&(x, y)| {
            let mut successors = vec![];
            // go up if y > 0
            if y > 0 {
                successors.push(((x, y - 1), grid[y - 1][x]));
            }
            // go left if x > 0
            if x > 0 {
                successors.push(((x - 1, y), grid[y][x - 1]));
            }
            // go right if x < max_x
            if x < end.0 {
                successors.push(((x + 1, y), grid[y][x + 1]));
            }
            // go down if y < max_y
            if y < end.1 {
                successors.push(((x, y + 1), grid[y + 1][x]));
            }

            successors.into_iter()
        },
        |&p| p == end,
    );

    let lowest_risk = path.unwrap().1;
    println!("{lowest_risk}");
}

use std::fs::read_to_string;
use std::collections::VecDeque;
use std::collections::HashSet;
use array_tool::vec::Intersect;

fn part1(grid: &Vec<&str>) -> usize {
    
    let mut sr = 0;
    let mut sc = 0;

    'outer: for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            if ch == 'S' {
                sr = r;
                sc = c;
                break 'outer;
            }
        }
    }

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    seen.insert((sr, sc));
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((sr, sc));

    while q.len() != 0 {
        let (r, c) = q.pop_front().unwrap();

        let ch = grid[r].chars().nth(c).unwrap();

        if r > 0 && "S|JL".contains(ch) && 
            "|7F".contains(grid[r-1].chars().nth(c).unwrap()) &&
            !seen.contains(&(r - 1, c)) {

            seen.insert((r - 1, c)); 
            q.push_back((r - 1, c));
        }

        if r < grid.len() - 1 && "S|7F".contains(ch) && 
            "|JL".contains(grid[r + 1].chars().nth(c).unwrap()) &&
            !seen.contains(&(r + 1, c)) {

            seen.insert((r + 1, c));
            q.push_back((r + 1, c));
        }

        if c > 0 && "S-J7".contains(ch) &&
            "-LF".contains(grid[r].chars().nth(c - 1).unwrap()) &&
            !seen.contains(&(r, c - 1)) {

            seen.insert((r, c - 1));
            q.push_back((r, c - 1));
        }

        if c < grid.len() - 1 && "S-LF".contains(ch)
            && "-J7".contains(grid[r].chars().nth(c + 1).unwrap()) &&
            !seen.contains(&(r, c + 1)) {
                
            seen.insert((r, c + 1));
            q.push_back((r, c + 1));
        }
    }

    return seen.len() / 2;
}

fn main() {
    let file_content = read_to_string("input").unwrap();
    let grid = file_content.lines().collect::<Vec<&str>>();
    
    println!("Part 1: {}", part1(&grid));
}

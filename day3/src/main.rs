use std::fs::read_to_string;
use std::collections::HashSet;

fn part2(grid: &Vec<&str>) -> i32 {
    let mut ans = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            if ch != '*' {
                continue;
            }

            let mut part_nums: HashSet<(usize, usize)> = HashSet::new(); 

            for cr in (r as i32 - 1)..=(r as i32 + 1) {
                let current_row = grid[cr as usize].chars().collect::<Vec<char>>();
                for cc in (c as i32 - 1)..=(c as i32 + 1) {
                    if
                        cr < 0 || cr > grid.len() as i32 ||
                        cc < 0 || cc > grid[cr as usize].len() as i32 || 
                        !current_row[cc as usize].is_digit(10)
                    { 
                        continue;
                    }

                    let mut cc = cc;
                    while cc > 0 && current_row[cc as usize - 1].is_digit(10) {
                        cc -= 1;
                    }

                    part_nums.insert((cr as usize, cc as usize));
                }
            }

            if part_nums.len() != 2 {
                continue;
            }

            let mut nums: Vec<i32> = Vec::new();

            for (r, c) in part_nums.iter() {
                let mut s = String::new();
                let current_row = grid[*r].chars().collect::<Vec<char>>();
                let mut c = *c;
                while c < current_row.len() && current_row[c].is_digit(10) {
                    s.push(current_row[c]);
                    c += 1;
                }
                nums.push(s.parse().unwrap());
            }
            ans += nums[0] * nums[1];
        }
    }
    return ans;
}

fn part1(grid: &Vec<&str>) -> i32 {

    let mut part_nums: HashSet<(usize, usize)> = HashSet::new(); 

    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            if ch.is_digit(10) || ch == '.' {
                continue;
            }

            for cr in (r as i32 - 1)..=(r as i32 + 1) {
                let current_row = grid[cr as usize].chars().collect::<Vec<char>>();
                for cc in (c as i32 - 1)..=(c as i32 + 1) {
                    if
                        cr < 0 || cr > grid.len() as i32 ||
                        cc < 0 || cc > grid[cr as usize].len() as i32 || 
                        !current_row[cc as usize].is_digit(10)
                    { 
                        continue;
                    }

                    let mut cc = cc;
                    while cc > 0 && current_row[cc as usize - 1].is_digit(10) {
                        cc -= 1;
                    }

                    part_nums.insert((cr as usize, cc as usize));
                }
            }
        }
    }

    let mut nums: Vec<i32> = Vec::new();

    for (r, c) in part_nums.iter() {
        let mut s = String::new();
        let current_row = grid[*r].chars().collect::<Vec<char>>();
        let mut c = *c;
        while c < current_row.len() && current_row[c].is_digit(10) {
            s.push(current_row[c]);
            c += 1;
        }
        nums.push(s.parse().unwrap());
    }

    return nums.iter().sum::<i32>();
}

fn main() {
    let file_content = read_to_string("input").unwrap();
    let grid: Vec<&str> = file_content.lines().collect();

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

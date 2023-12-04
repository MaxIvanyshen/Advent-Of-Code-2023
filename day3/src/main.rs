use std::fs::read_to_string;
use std::collections::HashSet;

/* in this code i refer to numbers that are adjacent to a special character as 
 * part numbers, because that's how they are called in the task formulation */

fn part2(grid: &Vec<&str>) {

    let mut ans = 0;

    //loop for finding a *
    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {

            if ch != '*' {
                continue;
            }

            //set containing indecies of first digits of our gear with two numbers adjacent
            let mut part_nums: HashSet<(usize, usize)> = HashSet::new();

            //iterate through chars around found *
            for cr in (r as i32 - 1)..=(r as i32 + 1) {
                for cc in (c as i32 - 1)..=(c as i32 + 1) {

                    let current_row = grid[cr as usize].chars().collect::<Vec<char>>();

                    if cr < 0 || cr >= grid.len() as i32 || cc < 0 || 
                        cc >= grid[cr as usize].len() as i32 
                        || !current_row[cc as usize].is_digit(10) {
                        continue;
                    }

                    //when number is found, get its first digit 
                    let mut cc = cc;
                    while cc > 0 && current_row[cc as usize - 1].is_digit(10) {
                        cc -= 1;
                    }
                    
                    //insert index of first digit of part number
                    part_nums.insert((cr as usize, cc as usize));
                }
            }

            //we need * with exactly 2 numbers adjacent
            if part_nums.len() != 2 {
                continue;
            }

            let mut nums: Vec<i32> = Vec::new();

            for (cr, cc) in part_nums.iter() {
                let mut s = String::new();
                let mut cc = *cc;
                while cc < grid[*cr].len() && grid[*cr].chars().nth(cc).unwrap().is_digit(10) {
                    s.push(grid[*cr].chars().nth(cc).unwrap());
                    cc += 1;                
                }
                nums.push(s.parse().unwrap());
            }
            
            //since there's only 2 numbers in our array, we can manually multiply them
            //to get a gear ratio to add to our answer
            ans += nums[0] * nums[1];
        }
    }

    println!("{}", ans);

}

fn part1(grid: &Vec<&str>) {
    let mut part_nums: HashSet<(usize, usize)> = HashSet::new();

    //loop for finding a special character
    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {

            if ch.is_digit(10) || ch == '.' {
                continue;
            }

            //iterate through chars around found special character
            for cr in (r as i32 - 1)..=(r as i32 + 1) {
                for cc in (c as i32 - 1)..=(c as i32 + 1) {

                    let current_row = grid[cr as usize].chars().collect::<Vec<char>>();

                    if cr < 0 || cr >= grid.len() as i32 || cc < 0 || 
                        cc >= grid[cr as usize].len() as i32 
                        || !current_row[cc as usize].is_digit(10) {
                        continue;
                    }

                    //when number is found, get its first digit 
                    let mut cc = cc;
                    while cc > 0 && current_row[cc as usize - 1].is_digit(10) {
                        cc -= 1;
                    }
                    
                    //insert index of first digit of part number
                    part_nums.insert((cr as usize, cc as usize));
                }
            }
        }
    }

    let mut nums: Vec<i32> = Vec::new();

    for (r, c) in part_nums.iter() {
        let mut s = String::new();
        let mut c = *c;
        while c < grid[*r].len() && grid[*r].chars().nth(c).unwrap().is_digit(10) {
            s.push(grid[*r].chars().nth(c).unwrap());
            c += 1;                
        }
        nums.push(s.parse().unwrap());
    }

    let ans = nums.iter().sum::<i32>();

    println!("{}", ans);

}

fn main() {
    let input_content = read_to_string("input").unwrap();
    let grid: Vec<&str> = input_content.lines().collect();

    part1(&grid);
    part2(&grid);
}

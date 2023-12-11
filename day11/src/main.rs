use std::{fs::read_to_string, iter::zip};

fn main() {
    let file_content = read_to_string("input").unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in file_content.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }
    
    let mut empty_rows: Vec<usize> = Vec::new();

    for(r, row) in grid.iter().enumerate() {
        let mut empty = true;
        for c in row {
            if c != &'.' {
                empty = false; 
            }
        }
        if empty {
           empty_rows.push(r); 
        }
    }

    let mut empty_cols: Vec<usize> = Vec::new();
    
    for c in 0..grid[0].len() {
        let all_dots = grid.iter().all(|row| row[c] == '.');
        if all_dots {
            empty_cols.push(c);
        }
    }

    let mut points: Vec<(usize, usize)> = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if ch == &'#' {
                points.push((r, c));
            } 
        }
    }
    
    let mut total = 0;
    let mut scale = 2;
    for (i, (r1, c1)) in points.iter().enumerate() {
        let mut j = 0;
        while j < i {
            let (r2, c2) = points[j];
            let mut r = std::cmp::min(r1, &r2).clone();
            while r < std::cmp::max(*r1, r2) {
                if empty_rows.contains(&r) {
                    total += scale; 
                }
                else {
                    total += 1;
                }
                r += 1;
            }
            let mut c = std::cmp::min(c1, &c2).clone();
            while c < std::cmp::max(*c1, c2) {
                if empty_cols.contains(&c) {
                    total += scale;
                }
                else {
                    total += 1;
                }
                c += 1;
            }
            j += 1;
        }
    }

    println!("{total}");
}

use std::collections::HashMap;
use std::fs::read_to_string;

fn parse_digit(map: &HashMap<&str, i32>, sub_str: &str, part: u32) -> Option<i32> {
   if sub_str.len() == 1 {
       let c = sub_str.chars().next().unwrap();
       match c.is_digit(10) {
            true => { return Some((c as i32) - 48); },
            false => { return None; },
       }
   } 
   else {
       if part == 2 {
           match map.get(&sub_str) {
               Some(x) => { return Some(*x); },
               None => { return None; },
           }
       }
   }
   return None;
}

fn solve(lines: Vec<String>, part: u32) -> i32 {
    
    let map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut ans: i32 = 0;

    for line in lines {
        let mut v = Vec::new(); 
        for start in 0..line.len() {
            for end in start..line.len() {
                let x = parse_digit(&map, &line[start..end+1], part);
                match x {
                    Some(n) => { v.push(n); },
                    None => { continue; },
                }
            }
        }
        ans += v[0] * 10 + v[v.len()-1];
    }

    return ans;
}

fn main() {
    
    let mut lines = Vec::new(); 
   
    for line in read_to_string("input").unwrap().lines() {
        lines.push(line.to_string()); 
    }

    println!("Part 1: {}", solve(lines.clone(), 1));
    println!("Part 2: {}", solve(lines.clone(), 2));
}

use std::collections::HashMap;
use std::fs::read_to_string;

fn parseDigit(map: &HashMap<&str, i32>, subStr: &str) -> Option<i32> {
   if subStr.len() == 1 {
       let c = subStr.chars().next().unwrap();
       match c.is_digit(10) {
            true => { return Some((c as i32) - 48); },
            false => { return None; },
       }
   } 
   else {
       match map.get(&subStr) {
           Some(x) => { return Some(*x); },
           None => { return None; },
       }
   }
   return None;
}

fn part1() -> i32 {
    
    let mut ans: i32 = 0;
    let mut lines = Vec::new(); 
   
    for line in read_to_string("input").unwrap().lines() {
        lines.push(line.to_string()); 
    }

    for line in lines {
        let mut v = Vec::new(); 
        for c in line.chars() {
            if c.is_digit(10) {
                v.push((c as i32) - 48);
            }
        }
        ans += (v[0] * 10 + v[v.len()-1]);
    }

    return ans;
}

fn part2() -> i32 {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);

    let mut ans: i32 = 0;
    let mut lines = Vec::new(); 
   
    for line in read_to_string("input").unwrap().lines() {
        lines.push(line.to_string()); 
    }

    for line in lines {
        let mut v = Vec::new(); 
        for start in 0..line.len() {
            for end in start..line.len() {
                
                let x = parseDigit(&map, &line[start..end+1]);
                match x {
                    Some(n) => { v.push(n); },
                    None => { continue; },
                }
            }
        }
        ans += (v[0] * 10 + v[v.len()-1]);
    }

    return ans;
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

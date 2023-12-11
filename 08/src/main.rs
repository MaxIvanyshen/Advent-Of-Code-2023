use std::fs::read_to_string;
use std::collections::HashMap;
use gcd::Gcd;

fn dfs_p1(curr_node: &str, map: &HashMap<&str, (&str, &str)>, steps: i32, path: Vec<char>, curr_step: usize) -> i32 {
    if curr_node.eq("ZZZ") {
        return steps ;
    }
    
    let mut next_step = curr_step + 1;

    let next = path[curr_step];
    if curr_step + 1 >= path.len() {
        next_step = 0;
    }

    if next == 'L' {
        return dfs_p1(map.get(curr_node).unwrap().0, map, steps + 1, path, next_step);
    } 
    else {
        return dfs_p1(map.get(curr_node).unwrap().1, map, steps + 1, path, next_step);
    } 
}

fn part2(map: &HashMap<&str, (&str, &str)>, path: Vec<char>) -> u64 {
     let mut positions: Vec<&str> = Vec::new();
     for (k, v) in map.iter() {
        if k.ends_with("A") {
            positions.push(k); 
        }
     }

     let mut cycles: Vec<Vec<i32>> = Vec::new();

     for mut current in positions {
        let mut cycle: Vec<i32> = Vec::new(); 
        let mut curr_step = 0;
        let mut step_count = 0;
        let mut first_z = "";

        loop {
            while step_count == 0 || !current.ends_with("Z") {
                 step_count += 1;
                 if path[curr_step] == 'L' {
                    current = map.get(current).unwrap().0;
                 }
                 else {
                    current = map.get(current).unwrap().1;
                 }

                 curr_step += 1;
                 if curr_step >= path.len() {
                    curr_step = 0; 
                 }
            }
            cycle.push(step_count);

            if first_z.eq("") {
                first_z = current;
                step_count = 0;
            }
            else if current == first_z {
                break;
            }
        }

         cycles.push(cycle);
     }

     let mut nums: Vec<u64> = Vec::new();

     for cycle in cycles {
        nums.push(cycle[0] as u64);
     }

     let mut lcm: u64 = nums.pop().unwrap(); 
     for num in nums {
        lcm = (lcm * num)/ lcm.gcd(num);
     }

     return lcm;
}

fn main() {
    let file_content = read_to_string("input").unwrap();

    let path: Vec<char> = file_content
        .lines()
        .collect::<Vec<&str>>()[0]
        .chars()
        .collect();

    let nodes: Vec<&str> = file_content
        .lines()
        .collect::<Vec<&str>>()[2..]
        .to_vec();

     let mut map: HashMap<&str, (&str, &str)> = HashMap::new();   

     for n in &nodes {
        let v = n.split("=").collect::<Vec<&str>>();
        let value = v[0].trim();
        let v = v[1].trim()
            .split(",")
            .collect::<Vec<&str>>();

        map.insert(value, (&v[0][1..v[0].len()], &v[1][1..v[1].len() - 1]));
     }

     println!("Part 1: {}", dfs_p1("AAA", &map, 0, path.clone(), 0));
     println!("Part 2: {}", part2(&map, path.clone()));
}


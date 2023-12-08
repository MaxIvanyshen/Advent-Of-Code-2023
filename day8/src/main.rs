use std::fs::read_to_string;
use std::collections::HashMap;

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

fn dfs_p2<'a>(curr_nodes: &'a mut Vec<&'a str>, map: &HashMap<&'a str, (&'a str, &'a str)>, steps: i32, path: Vec<char>, curr_step: usize) -> i32 {
    let mut all_z = true;
    for node in &mut *curr_nodes {
        if !node.ends_with("Z") {
            all_z = false; 
        }
    }
    if all_z {
        return steps;
    }

    let mut next_step = curr_step + 1;

    let next = path[curr_step];
    if curr_step + 1 >= path.len() {
        next_step = 0;
    }

    let mut v: Vec<&str> = Vec::new();

    if next == 'L' {
        let mut i: usize = 0;
        while i < curr_nodes.len() {
            curr_nodes[i] = map.get(curr_nodes[i]).unwrap().0;
            i += 1;
        }
        return dfs_p2(curr_nodes, map, steps + 1, path, next_step);
    }
    else {
        let mut i: usize = 0;
        while i < curr_nodes.len() {
            curr_nodes[i] = map.get(curr_nodes[i]).unwrap().1;
            i += 1;
        }
        return dfs_p2(curr_nodes, map, steps + 1, path, next_step);
    }
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

     for n in nodes {
        let v = n.split("=").collect::<Vec<&str>>();
        let value = v[0].trim();
        let mut children: (&str, &str);
        let v = v[1].trim()
            .split(",")
            .collect::<Vec<&str>>();

        map.insert(value, (&v[0][1..v[0].len()], &v[1][1..v[1].len() - 1]));
     }

     //println!("Part 1: {}", dfs_p1("AAA", &map, 0, path, 0));
     
     let mut nodes: Vec<&str> = Vec::new();
     for (k, v) in map.iter() {
        if k.ends_with("A") {
            nodes.push(k);
        }
     }

     println!("Part 2: {}", dfs_p2(&mut nodes, &map, 0, path, 0));


}

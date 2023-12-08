use std::fs::read_to_string;
use std::collections::HashMap;

fn dfs(curr_node: &str, map: &HashMap<&str, (&str, &str)>, steps: i32, path: Vec<char>, curr_step: usize) -> i32 {
    if curr_node.eq("ZZZ") {
        return steps ;
    }
    
    let mut next_step = curr_step + 1;

    let next = path[curr_step];
    if curr_step + 1 >= path.len() {
        next_step = 0;
    }

    if next == 'L' {
        return dfs(map.get(curr_node).unwrap().0, map, steps + 1, path, next_step);
    } 
    else {
        return dfs(map.get(curr_node).unwrap().1, map, steps + 1, path, next_step);
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

     println!("{}", dfs("AAA", &map, 0, path, 0));
}

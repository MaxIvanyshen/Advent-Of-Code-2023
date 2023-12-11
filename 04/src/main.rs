use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let file_content = read_to_string("input").unwrap();
    let games: Vec<&str> = file_content.lines().collect();

    let mut p1 = 0;
    let mut map: HashMap<usize, i32> = HashMap::new();

    for (i, game) in games.iter().enumerate() {
        let mut points = 0.5;

        match map.get(&i) {
            Some(x) => map.insert(i, x + 1),
            None => map.insert(i, 1),
        };

        let numbers = game.split(":").collect::<Vec<&str>>()[1];
        let win_nums: Vec<i32> = numbers.split("|").collect::<Vec<&str>>()[0].trim().split(" ").filter(|s| !s.eq(&"")).map(|x| x.parse::<i32>().unwrap()).collect();
        let nums: Vec<i32> = numbers.split("|").collect::<Vec<&str>>()[1].trim().split(" ").filter(|s| !s.eq(&"")).map(|x| x.parse::<i32>().unwrap()).collect();

        let mut count = 0;
        for n in nums {
            if win_nums.contains(&n) {
                count += 1;
                points *= 2.0;
            }
        }
        for j in 0..count {
            match map.get(&(i+1+j)) {
                Some(x) => map.insert(i+1+j, x + map.get(&i).unwrap()),
                None => map.insert(i+1+j, *map.get(&i).unwrap()),
            };
        }
        p1 += points as i32;
    }

    let p2 = map.values().sum::<i32>();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

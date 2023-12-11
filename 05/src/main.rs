use std::fs::read_to_string;
use std::cmp;

 fn part2(file_content: &String) {

    let seeds_and_blocks: Vec<&str> = file_content.split("\n\n").collect::<Vec<&str>>();
    let inputs = seeds_and_blocks[0]
        .split(":").collect::<Vec<&str>>()[1]
        .split(" ")
        .filter(|s| !s.eq(&""))
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    
    let mut seeds: Vec<(i64, i64)> = Vec::new();
    let mut i = 0;
    
    while i < inputs.len() {
        seeds.push((inputs[i], inputs[i] + inputs[i + 1]));
        i += 2;
    }

    let mut blocks: Vec<&str> = Vec::new(); 

    for (i, s) in seeds_and_blocks.iter().enumerate() {
        if i != 0 {
            blocks.push(s);
        }
    }

    for block in blocks {
        let mut ranges: Vec<(i64, i64, i64)> = Vec::new();
        for line in &block.lines().collect::<Vec<_>>()[1..] {
            let mut range: (i64, i64, i64) = (0, 0, 0);

            let x = line
                .split(" ")
                .filter(|s| !s.eq(&""))
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            range.0 = x[0];
            range.1 = x[1];
            range.2 = x[2];

            ranges.push(range);
        }
        let mut n: Vec<(i64, i64)> = Vec::new();

        while seeds.len() > 0 {
           let (s, e) = seeds.pop().unwrap();

           let mut updated = false;
            for (a, b, c) in ranges.iter() {
                let os = cmp::max(s, *b);
                let oe = cmp::min(e, b + c);
                if os < oe {
                    n.push((os - b + a, oe - b + a));
                    if os > s {
                        seeds.push((s, os));
                    }
                    if e > oe {
                        seeds.push((oe, e));
                    }
                    updated = true;
                    break;
                }
            }
            if !updated {
                n.push((s, e));
            }
        }

        seeds = n.clone();
    }


    
    let m = *seeds.iter().min().unwrap();
    println!("Part 2: {}", m.0);
}

fn part1(file_content: &String) {
    
    let seeds_and_blocks: Vec<&str> = file_content.split("\n\n").collect::<Vec<&str>>();
    let mut seeds = seeds_and_blocks[0]
        .split(":").collect::<Vec<&str>>()[1]
        .split(" ")
        .filter(|s| !s.eq(&""))
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    

    let mut blocks: Vec<&str> = Vec::new(); 

    for (i, s) in seeds_and_blocks.iter().enumerate() {
        if i != 0 {
            blocks.push(s);
        }
    }

    for block in blocks {
        let mut ranges: Vec<(i64, i64, i64)> = Vec::new();
        for line in &block.lines().collect::<Vec<_>>()[1..] {
            let mut range: (i64, i64, i64) = (0, 0, 0);

            let x = line
                .split(" ")
                .filter(|s| !s.eq(&""))
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            range.0 = x[0];
            range.1 = x[1];
            range.2 = x[2];

            ranges.push(range);
        }
        let mut n: Vec<i64> = Vec::new();

        for x in &seeds {
            let mut updated = false;
            for (a, b, c) in ranges.iter() {
                if b <= &x && x < &(b + c) {
                    n.push(x - b + a);
                    updated = true;
                }
            }
            if !updated {
                n.push(*x);
            }
        }

        seeds = n.clone();
    }

    println!("Part 1: {}", *seeds.iter().min().unwrap());
}

fn main() {
    let file_content = read_to_string("input").unwrap();

    part1(&file_content);
    part2(&file_content);
}

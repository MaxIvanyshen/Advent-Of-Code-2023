use std::fs::read_to_string;

fn count_ways_to_win(time: i64, distance: i64) -> i64 {
    let mut ans: i64 = 0;
    let mut j: i64 = 0;
    while j <= time {
        let speed = j;

        let d = speed * (time - j);
        if d > distance {
            ans += 1;
        }

        j += 1;
    }
    return ans;
}


fn part1(file_content: &String) -> i64 {

    let times: Vec<i32> = file_content
        .lines()
        .collect::<Vec<&str>>()[0]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split(" ")
        .filter(|s| !s.eq(&""))
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let distances: Vec<i32> = file_content
        .lines()
        .collect::<Vec<&str>>()[1]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split(" ")
        .filter(|s| !s.eq(&""))
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut counts: Vec<i64> = Vec::new();

    for (i, time) in times.iter().enumerate() {
       let distance = distances[i];
       counts.push(count_ways_to_win(*time as i64, distance as i64));
    }

    let mut ans: i64 = 1;
    for count in counts {
        ans *= count;
    }

    return ans;
}

fn part2(file_content: &String) -> i64 {
    let mut ans = 0;

    let time: i64 = file_content
        .lines()
        .collect::<Vec<&str>>()[0]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .replace(" ", "")
        .parse::<i64>().unwrap();

    let distance: i64 = file_content
        .lines()
        .collect::<Vec<&str>>()[1]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .replace(" ", "")
        .parse::<i64>().unwrap();

    return count_ways_to_win(time, distance);
} 

fn main() {
    let file_content = read_to_string("input").unwrap();

    println!("Part 1: {}", part1(&file_content));
    println!("Part 2: {}", part2(&file_content));

}


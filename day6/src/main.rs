use std::fs::read_to_string;

fn main() {
    let file_content = read_to_string("input").unwrap();

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

    let mut counts: Vec<i32> = Vec::new();

    for (i, time) in times.iter().enumerate() {
       let mut count = 0; 
       let distance = distances[i];
       let mut j = 0;
       while &j <= time {
            let speed = j;
            
            let d = speed * (time - j);
            if d > distance {
                count += 1;
            }

            j += 1;
       }
       counts.push(count);
    }

    let mut ans = 1;
    for count in counts {
        ans *= count;
    }

    println!("{ans}");
}


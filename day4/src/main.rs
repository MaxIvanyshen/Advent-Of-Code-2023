use std::fs::read_to_string;

fn part1(games: &Vec<&str>) -> i32 {
    let mut ans = 0;

    for game in games {
        let mut points = 0.5;
        let numbers = game.split(":").collect::<Vec<&str>>()[1];
        let win_nums: Vec<i32> = numbers.split("|").collect::<Vec<&str>>()[0].trim().split(" ").filter(|s| !s.eq(&"")).map(|x| x.parse::<i32>().unwrap()).collect();
        let nums: Vec<i32> = numbers.split("|").collect::<Vec<&str>>()[1].trim().split(" ").filter(|s| !s.eq(&"")).map(|x| x.parse::<i32>().unwrap()).collect();

        for n in nums {
            if win_nums.contains(&n) {
                points *= 2.0;
            }
        }


        ans += points as i32;
    }

    return ans;
}

fn main() {
    let file_content = read_to_string("input").unwrap();
    let games: Vec<&str> = file_content.lines().collect();

    println!("{}", part1(&games));
}

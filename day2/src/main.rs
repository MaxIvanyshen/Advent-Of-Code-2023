use std::fs::read_to_string;

fn solve(game_str: &str) -> (i32, i32) {
    let mut id: i32;
    let mut power: i32;

    let game_str_split = game_str.split(":").collect::<Vec<_>>();

    let id_str = game_str_split[0].split(" ").collect::<Vec<_>>()[1].to_string();

    id = id_str.parse::<i32>().unwrap();
    
    let mut iterations = game_str_split[1].split(";").collect::<Vec<_>>();
    for it in iterations {

        let colors = it.split(",");
        
        for color in colors {

            let n_str = color.split(" ").collect::<Vec<_>>()[1];
            let n = n_str.parse::<i32>().unwrap();

            if color.contains("red") {
                if n > 12 {
                    id = 0;
                }
                if n > set.0 {
                    set.0 = n;
                }
            }
            else if color.contains("green") {
                if n > 13 {
                    id = 0;
                }
                if n > set.1 {
                    set.1 = n;
                }
            }
            else if color.contains("blue") {
                if n > 14 {
                    id = 0;
                }
                if n > set.2 {
                    set.2 = n;
                }
            }
        }
    }

    power = set.0 * set.1 * set.2;

    return (id, power);
}

fn main() {

    let mut ans = (0, 0);

    for line in read_to_string("input").unwrap().lines() {
        let res = solve(line);
        ans.0 += res.0;
        ans.1 += res.1;
    }

    println!("Part 1: {}", ans.0);
    println!("Part 2: {}", ans.1);
}

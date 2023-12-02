use std::fs::read_to_string;

fn get_id_if_valid(game_str: &str) -> i32 {
    let conf = (12, 13, 14);     
    let game_str_split = game_str.split(":").collect::<Vec<_>>();

    let id_str = game_str_split[0].split(" ").collect::<Vec<_>>()[1].to_string();
    let id = id_str.parse::<i32>().unwrap();
    
    let iterations = game_str_split[1].split(";").collect::<Vec<_>>();

    for it in iterations {

        let colors = it.split(",");

        for color in colors {

            let n_str = color.split(" ").collect::<Vec<_>>()[1];
            let n = n_str.parse::<i32>().unwrap();

            if color.contains("red") {
                if n > conf.0 {
                    return 0;
                }
            }
            else if color.contains("green") {
                if n > conf.1 {
                    return 0;
                }
            }
            else if color.contains("blue") {
                if n > conf.2 {
                    return 0;
                }
            }
        }
    }


    return id;
}

fn main() {
    let mut ans = 0;

    for line in read_to_string("input").unwrap().lines() {
        ans += get_id_if_valid(line);
    }

    println!("{}", ans);
}

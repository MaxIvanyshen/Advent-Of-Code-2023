use std::fs::read_to_string;

fn predict_p1(nums: &Vec<i32>) -> i32 {
    let mut base_case = true;

    let mut i = 0;
    let mut v: Vec<i32> = Vec::new();
    while i < nums.len() - 1 {
        let n = nums[i + 1] - nums[i];
        v.push(n);
        if n != 0 {
            base_case = false;
        }
        i += 1;
    }

    if base_case {
        let n =  nums[nums.len() - 1] + v[v.len() - 1];
        return n; 
    }

    return nums[nums.len() - 1] + predict_p1(&v);
}

fn predict_p2(nums: &Vec<i32>) -> i32 {
    let mut base_case = true;

    let mut i = 0;
    let mut v: Vec<i32> = Vec::new();
    while i < nums.len() - 1 {
        let n = nums[i + 1] - nums[i];
        v.push(n);
        if n != 0 {
            base_case = false;
        }
        i += 1;
    }

    if base_case {
        let n = nums[0] - v[0];
        return n;
    }

    return nums[0] - predict_p2(&v);
}

fn main() {
    let file_content = read_to_string("input").unwrap();
    let lines = file_content.lines().collect::<Vec<&str>>(); 

    let mut p1 = 0;
    let mut p2 = 0;

    for line in lines {
        let nums: Vec<i32> = line
            .split(" ")
            .filter(|s| !s.eq(&""))
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        p1 += predict_p1(&nums);
        p2 += predict_p2(&nums);
    }
    
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");

}

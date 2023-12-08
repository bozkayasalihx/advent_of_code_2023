fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    let last = digits.last_mut().unwrap();
    if last < &mut 9 {
        *last += 1;
        return digits;
    }

    let mut t = String::new();
    for i in &digits {
        t.push_str(&i.to_string());
    }

    let t = t.parse::<i64>().unwrap_or(0) + 1;

    println!("{t}");

    t.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap_or(0) as i32)
        .collect::<Vec<i32>>()
}
fn main() {
    let v = vec![
        7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9, 4, 7,
        0, 1, 1, 1, 7, 4, 0, 0, 6,
    ];

    let res = plus_one(v);
    println!("{:?}", res);
}

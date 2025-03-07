fn main() {
    // 会报错,无法推定类型
    // let guess = "42".parse().except("Not a number!");

    let guess: i32 = "42".parse().expect("Not a number!");
    println!("guess = {}\n", guess);

    let guess2 = "42".parse::<i32>().expect("Not a number!");
    println!("guess2 = {}\n", guess2);
}

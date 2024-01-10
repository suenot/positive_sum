fn positive_sum(slice: &[i32]) -> i32 {
    println!("positive_sum");
    let mut sum = 0;
    for &i in slice {
        if i > 0 {
            sum += i;
        }
    }
    sum
}

fn main() {
    println!("main");
    println!("{:?}", positive_sum(&[1,-4,7,12])); // 1 + 7 + 12 = 20
}
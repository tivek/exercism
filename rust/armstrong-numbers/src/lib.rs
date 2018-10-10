pub fn is_armstrong_number(num: u32) -> bool {
    let len = ((num as f32).log(10.) as u32) + 1;
    let mut sum = 0;
    let mut i = num;
    while i != 0 {
        sum += (i % 10).pow(len);
        i /= 10;
    }
    sum == num
}

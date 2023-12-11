/// Return the sum of calibration values.
pub fn do_day1(input: &str) -> i32 {
    let mut total: i32 = 0;

    for found in input.split("\n") {
        if found.len() == 0 { continue }
        let nums: Vec<_> = found.chars().filter(|x| x.is_digit(10)).collect();
        let mut twodigitnum: Vec<_> = vec!();
        twodigitnum.push(nums[0]);
        twodigitnum.push(nums[nums.len()-1]);
        let s: String = twodigitnum.into_iter().collect();
        println!("{}", s);
        total += s.parse::<i32>().unwrap();
    }
    total
}

#[test]
fn test_do_day1() {
    let test_input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
    assert_eq!(do_day1(test_input), 142);
}

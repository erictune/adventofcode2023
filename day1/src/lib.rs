/// Return the sum of calibration values.
pub fn do_day1_prob1(input: &str) -> i32 {
    let mut total: i32 = 0;

    for found in input.split("\n") {
        if found.len() == 0 { continue }
        let nums: Vec<_> = found
            .chars()
            .filter(|x| x.is_digit(10))
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .collect();
        let twodigitnum = 10 * nums[0] + nums[nums.len()-1];
        println!("{}", twodigitnum);
        total += twodigitnum;
    }
    total
}

#[test]
fn test_do_day1_prob1() {
    let test_input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
    assert_eq!(do_day1_prob1(test_input), 142);
}

/// Return the sum of calibration values.
pub fn do_day1_prob2(input: &str) -> i32 {
    let mut total: i32 = 0;

    for found in input.split("\n") {
        let mut stringslice = &found[0..found.len()];
        let mut nums: Vec<_>  = vec![];
        while stringslice.len() > 0 {
            if stringslice.starts_with("one") {
                nums.push(1);
            } else if stringslice.starts_with("two") {
                nums.push(2);
            } else if stringslice.starts_with("three") {
                nums.push(3);
            } else if stringslice.starts_with("four") {
                nums.push(4);
            } else if stringslice.starts_with("five") {
                nums.push(5);
            } else if stringslice.starts_with("six") {
                nums.push(6);
            } else if stringslice.starts_with("seven") {
                nums.push(7);
            } else if stringslice.starts_with("eight") {
                nums.push(8);
            } else if stringslice.starts_with("nine") {
                nums.push(9);
            } else {
                let c = stringslice.chars().next().unwrap();
                if c.is_digit(10) {
                    nums.push(c.to_string().parse::<i32>().unwrap());
                }
            } 
            // There may be overlaps, like "oneight", and the authors would like us to parse that as {1,8}.
            // Therefore only advance one character at a time.
            stringslice = &stringslice[1..];   
        }
        if nums.len() < 1 { println!("Continuing"); continue; }
        let twodigitnum = 10 * nums[0] + nums[nums.len()-1];
        println!("{}", twodigitnum);
        total += twodigitnum;
    }
    total
}

// TODO: make above be do_day1_p1.
// then add do_day1_prob2.
// Make a vector of chars.
// Match if the string startswith any digit or any english spelled word.
// If so, advance index into string and push found value onto number vector.
// loop.
// Test that on new test input.
#[test]
fn test_do_day1_prob2() {
    let test_input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let expected = 29 + 83 + 13 + 24 + 42 + 14 + 76;
    
    assert_eq!(281, expected);
    assert_eq!(do_day1_prob2(test_input), expected);

    
    assert_eq!(do_day1_prob2("five94rxvhdhqh3three555"), 55);
    
    // Tricky case.
    assert_eq!(do_day1_prob2("oneight"), 18);

}


use grid::Grid;

/// Return the sum of calibration values.
pub fn do_day3_part1(input: &str) -> i32 {
    let total: i32 = 0;

    let lines: Vec<_> = input.split("\n").collect();
    let nr: usize = lines.len();
    let nc = lines[0].chars().count();
    let chars: Vec<char> = lines
        .iter()
        .map(|s| s.chars())
        .flatten()
        .collect::<Vec<char>>();
    // This won't catch all some cases of ragged lines.
    assert!(nr * nc == chars.len(), "Not rectangular");

    // A clever programmer from the age of limited memory might do this in a way where only 3 lines
    // worth of data are buffered at a time.  We aren't expecting to run out of memory, and
    // I think the algorithm, and its various corner cases around the edges, is more clear on
    // a 2-d grid rather than a pipeline of 3 lines.

    let gr = Grid::from_vec(chars, nc);
    //println!("{:#?}", gr);

    let mut partnum = None;
    for r in 0..nr {
        for c in 0..nc {
            match gr[(r, c)] {
                '@' | '#' | '$' | '%' | '&' | '*' | '-' | '+' | '=' | '/' | '.' => {
                    // We were parsing a part number, and now found a non-number, then emit the part number.
                    if let Some(x) = partnum {
                        println!("Parsed part number: {}", x);
                    }
                    partnum = None;
                }
                '1'..='9' | '0' => {
                    let digit = gr[(r, c)]
                        .to_string()
                        .parse::<i32>()
                        .expect("should have parsed single digit");
                    println!("Found a digit: {}", digit);

                    partnum = match partnum {
                        None => Some(digit),
                        Some(x) => Some(10 * x + digit),
                    };
                }
                _ => {
                    panic!("Unexpected character in input: {}", gr[(r, c)])
                }
            }
        }
        // The end of a line is an automatic end of any in-progress part number.
        if let Some(x) = partnum {
            println!("Parsed part number (due to end of line, tricky!): {}", x);
        }
        partnum = None;
    }

    // If we aren't in the state of parsing a number...
    // ... and we see a number, then start parsing,
    //     by pushing that number onto a stack or what have you
    //     and look at the i-1 indexes of old, cur, and new, if each of those is valid.  Mark as "by_symbol" if any.
    //     also check index i, above and below.  So 5 places.
    // ... and we see anything else, take no action
    // otherwise, we are parsing a number (stack is non-empty)
    // ... and we see a number, push it
    //     and check above and below for symbols, marking as "by_symbol" if any is found.
    // ... as we see anything else - we ended the number, check above here and below, then parse the number and accumulate.

    total
}

#[test]
fn test_do_day3_part1() {
    let test_input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(do_day3_part1(test_input), 4361);
}

/// Return the sum of calibration values.
pub fn do_day3_part2(_input: &str) -> i32 {
    let total: i32 = 0;
    total
}

#[test]

fn test_do_day3_part2() {
    let test_input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(do_day3_part2(test_input), 223452345);
}

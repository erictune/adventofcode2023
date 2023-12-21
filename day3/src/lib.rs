use grid::Grid;
use std::cmp;

/// Return the sum of calibration values.
pub fn do_day3_part1(input: &str) -> i32 {

    let mut lines: Vec<_> = input.split("\n").collect();
    // Fixup last empty string.
    if lines[lines.len()-1] == "" { lines.pop(); }
    // Convert to a flattened vector of characters, at which point we can check rectangularity (length is bytes for strings).
    let nr: usize = lines.len();
    let nc = lines[0].chars().count();
    let chars: Vec<char> = lines
        .iter()
        .map(|s| s.chars())
        .flatten()
        .collect::<Vec<char>>();
    println!("nr {} nc {} chars.len() {}", nr, nc, chars.len());
    // This won't catch all some cases of ragged lines.
    assert!(nr * nc == chars.len(), "Not rectangular");

    // A clever programmer from the age of limited memory might do this in a way where only 3 lines
    // worth of data are buffered at a time.  We aren't expecting to run out of memory, and
    // I think the algorithm, and its various corner cases around the edges, is more clear on
    // a 2-d grid rather than a pipeline of 3 lines.

    let gr = Grid::from_vec(chars, nc);
    //println!("{:#?}", gr);

    // TODO: When you read a number, you should check if any of the eight neighbors is a symbol.
    // A simple improvement is skip checks if a symbol was already found.
    //  an uncessary improvement would be to avoid checking already-checked locations prior to finding the symbol (overlapping checks).
    //  In either case, a list of offsets for 8-neightbors can used instead of coding all 8 checks.
    let mut attached_partnums: Vec<i32> = vec![];
    {
        // Holds value whose digits are being accumulated, or "shifted in".  Once complete: a part number.
        let mut partial_partnum = None;
        // Whether the partial_partnum has been found to be adjacent to a symbol yet.  
        let mut adjacent = false;

        for r in 0..nr {
            for c in 0..nc {
                match gr[(r, c)] {
                    '@' | '#' | '$' | '%' | '&' | '*' | '-' | '+' | '=' | '/' | '.' => {
                        if partial_partnum.is_some() {
                            // Since we encountered a a non-digit, we finish accumlating this part number.
                            // We have previously checked all its neighbors.
                            if adjacent {
                                attached_partnums.push(partial_partnum.unwrap());
                            } else {
                                println!("Skipping part number with no adjacenct symbols: {}", partial_partnum.unwrap())
                            }
                            partial_partnum = None;
                            adjacent = false;
                        }
                    }
                    '0'..='9' => {
                        let digit = gr[(r, c)]
                            .to_string()
                            .parse::<i32>()
                            .expect("should have parsed single digit");
                        partial_partnum = match partial_partnum {
                            None => Some(digit),
                            Some(x) => Some(10 * x + digit),
                        };
                        if !adjacent {
                            // Check neigbors
                            // First establish which indicies to check, as this can be different at boundaries.
                            let rlo = cmp::max(r.saturating_sub(1), 0);
                            let rhi = cmp::min(r + 1, nr - 1 );
                            let clo = cmp::max(c.saturating_sub(1), 0);
                            let chi = cmp::min(c + 1, nc - 1 );
                            
                            'nieghb: for rr in rlo..=rhi {
                                for cc in clo..=chi {
                                    if match gr[(rr, cc)] { '@' | '#' | '$' | '%' | '&' | '*' | '-' | '+' | '=' | '/'  => true, _ => false } {
                                        adjacent = true;
                                        println!("Symbol!: {}", gr[(rr, cc)]);
                                        break 'nieghb;
                                    } 
                                }
                            } 
                        }
                    }
                    _ => {
                        panic!("Unexpected character in input: {}", gr[(r, c)])
                    }
                }
            }
            // The end of a line is an automatic end of any in-progress part number.
            if partial_partnum.is_some() {
                // Since we encountered a a non-digit, we finish accumlating this part number.
                // We have previously checked all its neighbors.
                if adjacent {
                    attached_partnums.push(partial_partnum.unwrap());
                } else {
                    println!("Skipping part number with no adjacenct symbols: {}", partial_partnum.unwrap())
                }
                partial_partnum = None;
                adjacent = false;
            }
        }
    }
    println!("Attached part numbers: {:?}", attached_partnums);
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

    attached_partnums.iter().sum()
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


#[test]
fn test_rust() {
    assert_eq!(cmp::max(0, 0_usize.saturating_sub(1)), 0);
}

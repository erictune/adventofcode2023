use std::cmp;

#[derive(PartialEq, Debug)]
struct ColorCount {
    r: i32,
    g: i32,
    b: i32,
}

#[derive(PartialEq, Debug)]
struct Game {
    id: i32,
    draws: Vec<ColorCount>,
}

/// Return the sum of calibration values.
fn parseline(input: &str) -> Result<Game, String> {
    let mut _total: i32 = 0;
    let mut draws = vec![];
    let (start, rest) = input.split_once(":").expect("split line on colon");
    let gameid = start
        .strip_prefix("Game ")
        .expect("game prefix")
        .parse::<i32>()
        .expect("parse game id");
    for drawstr in rest.split(";") {
        let mut d = ColorCount { r: 0, g: 0, b: 0 };
        for s in drawstr.split(",") {
            let (countstr, colorstr) = s.trim().split_once(" ").expect("count and color");
            let count = countstr
                .parse::<i32>()
                .expect("count of color should be i32");
            *(match colorstr {
                "red" => &mut d.r,
                "green" => &mut d.g,
                "blue" => &mut d.b,
                _ => panic!("unknown color"),
            }) += count;
        }
        draws.push(d)
    }
    Ok(Game {
        id: gameid,
        draws: draws,
    })
}

#[test]
fn test_parseline() {
    let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let expected: Game = Game {
        id: 1,
        draws: vec![
            ColorCount { r: 4, g: 0, b: 3 },
            ColorCount { r: 1, g: 2, b: 6 },
            ColorCount { r: 0, g: 2, b: 0 },
        ],
    };
    let res = parseline(line);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), expected);
}

fn game_possible_with_bag(game: &Game, bag: &ColorCount) -> bool {
    for d in &game.draws {
        if d.r > bag.r || d.g > bag.g || d.b > bag.b {
            return false;
        }
    }
    true
}

#[test]
fn test_game_possible_with_bag() {
    let bag = ColorCount {
        r: 2,
        g: 20,
        b: 200,
    };
    assert!(game_possible_with_bag(
        &Game {
            id: 0,
            draws: vec![ColorCount { r: 1, g: 1, b: 1 }]
        },
        &bag
    ));
    assert!(!game_possible_with_bag(
        &Game {
            id: 0,
            draws: vec![ColorCount { r: 3, g: 0, b: 0 }]
        },
        &bag
    ));
    assert!(!game_possible_with_bag(
        &Game {
            id: 0,
            draws: vec![ColorCount { r: 0, g: 30, b: 0 }]
        },
        &bag
    ));
    assert!(!game_possible_with_bag(
        &Game {
            id: 0,
            draws: vec![ColorCount { r: 0, g: 0, b: 300 }]
        },
        &bag
    ));
}

fn max(d1: &ColorCount, d2: &ColorCount) -> ColorCount {
    ColorCount {
        r: cmp::max(d1.r, d2.r),
        g: cmp::max(d1.g, d2.g),
        b: cmp::max(d1.b, d2.b),
    }
}

#[test]
fn test_max() {
    assert_eq!(
        max(
            &ColorCount { r: 0, g: 2, b: 0 },
            &ColorCount { r: 1, g: 0, b: 3 },
        ),
        ColorCount { r: 1, g: 2, b: 3 }
    );
    assert_eq!(
        max(
            &ColorCount { r: 1, g: 0, b: 3 },
            &ColorCount { r: 0, g: 2, b: 0 },
        ),
        ColorCount { r: 1, g: 2, b: 3 }
    );
}

fn min_bag_for_game(game: &Game) -> ColorCount {
    let mut res = ColorCount { r: 0, g: 0, b: 0 };
    for d in &game.draws {
        res = max(d, &res);
    }
    res
}

#[test]
fn test_min_bag_for_game() {
    let cases = vec![
        (
            // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            // In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes. If any color had even one fewer cube, the game would have been impossible.
            Game {
                id: 1,
                draws: vec![
                    ColorCount { r: 4, g: 0, b: 3 },
                    ColorCount { r: 1, g: 2, b: 6 },
                    ColorCount { r: 0, g: 2, b: 0 },
                ],
            },
            ColorCount { r: 4, g: 2, b: 6 },
        ),
        (
            // Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
            // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game {
                id: 2,
                draws: vec![
                    ColorCount { r: 0, g: 2, b: 1 },
                    ColorCount { r: 1, g: 3, b: 4 },
                    ColorCount { r: 0, g: 2, b: 1 },
                ],
            },
            ColorCount { r: 1, g: 3, b: 4 },
        ),
    ];
    for case in cases {
        assert_eq!(min_bag_for_game(&case.0), case.1);
    }
}

/// Return the sum of calibration values.
pub fn do_day2_part1(input: &str) -> i32 {
    let mut total: i32 = 0;

    let bag = ColorCount {
        r: 12,
        g: 13,
        b: 14,
    };
    for found in input.split("\n") {
        if found.len() == 0 {
            continue;
        }
        let game = parseline(found).unwrap();
        if game_possible_with_bag(&game, &bag) {
            total += game.id
        }
    }
    total
}

#[test]
fn test_do_day2_part1() {
    let test_input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    assert_eq!(do_day2_part1(test_input), 1 + 2 + 5);
}

/// Return the sum of calibration values.
pub fn do_day2_part2(input: &str) -> i32 {
    let mut total: i32 = 0;

    for found in input.split("\n") {
        if found.len() == 0 {
            continue;
        }
        let game = parseline(found).unwrap();
        let m = min_bag_for_game(&game);
        total += m.r * m.g * m.b;
    }
    total
}

#[test]
fn test_do_day2_part2() {
    let test_input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    assert_eq!(do_day2_part2(test_input), 2286);
}

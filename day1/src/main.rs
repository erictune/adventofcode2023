use std::env;
use std::fs;
use std::process;

// Run as:
// day1 [part1|part2] input.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {}  `part1'|`part2' FILENAME", args[0]);
        process::exit(1);
    }
    let partnum = match args[1].as_str() {
        "part1" => Some(1),
        "part2" => Some(2),
        _ => None,
    }
    .expect("Usage: <arg0>  `part1'|`part2' FILENAME");
    let file_path = &args[2];
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let output = match partnum {
        1 => day1::do_day1_part1(&input),
        2 => day1::do_day1_part2(&input),
        _ => unreachable!(),
    };
    println!("{}", output)
}

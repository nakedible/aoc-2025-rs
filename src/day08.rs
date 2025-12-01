use test_case::test_case;

fn parse_input(filename: &str) -> i64 {
    let input = std::fs::read_to_string(filename).unwrap();
    let ret = input.len() as i64;
    ret
}

#[test_case("inputs/input-08.txt" => ignore)]
pub fn puzzle1(filename: &str) -> i64 {
    let input = parse_input(filename);

    let ret = input;
    ret
}

#[test_case("inputs/input-08.txt" => ignore)]
pub fn puzzle2(filename: &str) -> i64 {
    let input = parse_input(filename);

    let ret = input;
    ret
}

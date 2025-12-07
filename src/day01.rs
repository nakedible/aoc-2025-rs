use test_case::test_case;

fn parse_input(filename: &str) -> Vec<(bool, i64)> {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut ret = Vec::new();
    for line in input.lines() {
        let (dir, val) = line.split_at(1);
        let dir = if dir.starts_with('L') {
            false
        } else if dir.starts_with('R') {
            true
        } else {
            panic!("invalid line: {}", line);
        };
        let val: i64 = val.parse().unwrap();
        ret.push((dir, val));
    }
    ret
}

#[test_case("inputs/input-01.txt" => -1)]
pub fn puzzle1(filename: &str) -> i64 {
    let input = parse_input(filename);

    let mut dial = 50;
    let mut ret = 0;
    for (dir, val) in input.iter() {
        dial = (dial + if *dir { *val } else { -*val }).rem_euclid(100);
        if dial == 0 {
            ret += 1;
        }
    }
    ret
}

#[test_case("inputs/input-01.txt" => ignore)]
pub fn puzzle2(filename: &str) -> i64 {
    let input = parse_input(filename);

    let ret = input.len() as i64;
    ret
}

use waridley_aoc_2022::day_4::eval_part_1;

pub fn main() {
    let result = eval_part_1("day_4").unwrap();
    println!("{result}");
}

#[cfg(test)]
#[test]
fn part_1() {
    let result = eval_part_1("day_4.example").unwrap();
    assert_eq!(result, 2)
}

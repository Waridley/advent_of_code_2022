use waridley_aoc_2022::day_4::eval_part_2;

pub fn main() {
    let result = eval_part_2("day_4").unwrap();
    println!("{result}");
}

#[cfg(test)]
#[test]
fn part_2() {
    let result = eval_part_2("day_4.example").unwrap();
    assert_eq!(result, 4)
}

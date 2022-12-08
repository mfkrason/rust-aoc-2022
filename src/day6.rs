
pub mod day6 {
    use itertools::Itertools;

    pub fn generator(input: &String) -> &String {
        input
    }

    pub fn part1(input: &String) -> (usize, String) {

        for (index, chars) in input.chars().collect::<Vec<char>>().windows(4).enumerate() {
            if chars.iter().all_unique() {
                return (index+4, chars.iter().collect::<String>());
            }
        }
        (0usize, "".to_string())
    }

    pub fn part2(input: &String) -> (usize, String) {

        for (index, chars) in input.chars().collect::<Vec<char>>().windows(14).enumerate() {
            if chars.iter().all_unique() {
                return (index+14, chars.iter().collect::<String>());
            }
        }
        (0usize, "".to_string())
    }


}

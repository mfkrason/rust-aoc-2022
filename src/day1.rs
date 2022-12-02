
pub mod day1 {
    pub type Int = u32;

    // Parse the file
    #[allow(dead_code)]
    pub fn generator(input: &String) -> Vec<Int> {
        // Split on blank lines -- end of line followed by an end of line
        let mut elves: Vec<Int> = input.split("\n\n")
            // For each block
            .map( |x| 
                  // Call lines
                  x.lines()
                  // And for each line in lines parse to an int
                  .map(|x| x.parse::<Int>().expect("not an int"))
                  // Collect it to a Vec<Int>
                  .collect::<Vec<Int>>()
                  // Iterate over that and do the summation
                  .iter()
                  .sum()
                  // And convert that to a vector of ints
                ).collect();

        // Sort in reverse order (normal sort is a.cmp(b)
        elves.sort_by(|a,b| b.cmp(a));
        // "yield" elves
        elves
    }

    // Part 1 is the largest value
    #[allow(dead_code)]
    pub fn part1(input: &Vec<Int>) -> Int {
        input[0]
    }

    // Part 2 yields the top 3 values added together
    #[allow(dead_code)]
    pub fn part2(input: &Vec<Int>) -> Int {
        input[0] + input[1] + input[2]
    }
}

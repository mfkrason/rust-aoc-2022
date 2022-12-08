
pub mod day2 {
    use std::collections::HashMap;

    pub fn generator(input: &String) -> &String {
        input
    }

    pub fn part1(input: &String) -> u32 {
        let winloss = HashMap::from([
            // Wins
            ("A Y", 2 + 6),
            ("B Z", 3 + 6),
            ("C X", 1 + 6),

             // Draws
            ("A X", 1 + 3),
            ("B Y",2 + 3),
            ("C Z", 3 + 3),
             // Losses
             ("A Z", 3 + 0),
             ("B X", 1 + 0),
             ("C Y", 2 + 0)
        ]);

        let games: Vec<u32> = input
            .lines()
            .map(|x| winloss[&x])
            .collect();

        games.iter().sum()
    }

    pub fn part2(input: &String) -> u32 {
        let winloss = HashMap::from([
            // Lose
            ("A X", 3 + 0),
            ("B X", 1 + 0),
            ("C X", 2 + 0),

            // Draw
            ("A Y", 1 + 3),
            ("B Y", 2 + 3),
            ("C Y", 3 + 3),

            // Win
            ("A Z", 2 + 6),
            ("B Z", 3 + 6),
            ("C Z", 1 + 6),
        ]);

        let games: Vec<u32> = input
            .lines()
            .map(|x| winloss[&x])
            .collect();

        games.iter().sum()
    }

}



pub mod day3 {
    use array_tool::vec::Intersect;

    pub fn generator(input: &String) -> Vec<(String,String)> {

        input
            .lines()
            .map(|x| {
                let (p1, p2) = x.split_at(x.len()/2);
                (String::from(p1), String::from(p2))
            }).collect()

    }

    pub fn part1(input: &Vec<(String, String)>) -> u32 {

        let mut items: Vec<char> = [].to_vec();
        for pair in input {
                for ch in pair.0.chars() {
                    if ! pair.1.find(ch).is_none() {
                       items.push(ch) ;
                       break;
                    }
                }
        }

        items
            .iter()
            .map(|x| {
                if x.is_lowercase() {
                    x.to_digit(36).unwrap() - 'a'.to_digit(36).unwrap() + 1
                }
                else {
                    x.to_digit(36).unwrap() - 'A'.to_digit(36).unwrap() + 27
                }
            }
            ).collect::<Vec<u32>>()
            .iter()
            .sum()
    }

    pub fn part2(input: &Vec<(String, String)>) -> u32 {

        let mut lines = input.iter();
        let mut totals: Vec<u32> = Vec::new();
        while let ( Some(l1), Some(l2), Some(l3) ) = (lines.next(), lines.next(), lines.next()) {
            let mut shared_sack: Vec<char> = Vec::new();
            shared_sack.extend(l1.0.chars());
            shared_sack.extend(l1.1.chars());

            let mut sack2: Vec<char> = Vec::new();
            sack2.extend(l2.0.chars());
            sack2.extend(l2.1.chars());
            let mut sack3: Vec<char> = Vec::new();
            sack3.extend(l3.0.chars());
            sack3.extend(l3.1.chars());

            let shared_sack = shared_sack.intersect(sack2);
            let shared_sack = shared_sack.intersect(sack3);
            println!("{:#?}", shared_sack);

            if shared_sack[0].is_lowercase() {
                totals.push(shared_sack[0].to_digit(36).unwrap() - 'a'.to_digit(36).unwrap() + 1);
            }
            else {
                totals.push(shared_sack[0].to_digit(36).unwrap() - 'A'.to_digit(36).unwrap() + 27);
            }
        }


        totals.iter().sum()
    }

}

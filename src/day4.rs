pub mod day4 {
	use itertools::Itertools;

    pub fn generator(input: &String) -> Vec<(String, String, String, String))> {

		let mut assignments = Vec::new();
		for line in input.lines() {
			let (one, two) = line.split(",").tuples();
			let (f1,f2) = one.split("-").tuples();
			let (s1,s2) = two.split("-").tuples();
			assignments.push(((f1,f2),(s1,s2)));
		}

            [((0,0),(1,1))].to_vec()
    }

    pub fn part1(_input: &Vec<((String,String),(String,String))>) -> u32 {
        0
    }

    pub fn part2(_input: &Vec<((String,String),(String,String))>) -> u32 {
        0
    }

}

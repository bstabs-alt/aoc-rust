struct TopElf {
    one: u32,
    two: u32,
    three: u32,
}

impl TopElf {
    fn get_top_elf(&mut self, curr_sum: u32) {
        if curr_sum > self.one {
            self.three = self.two;
            self.two = self.one; 
            self.one = curr_sum;
        } else if curr_sum > self.two {
            self.three = self.two; 
            self.two = curr_sum;
        } else if curr_sum > self.three {
            self.three = curr_sum;
        }  
    }

    fn sum(&mut self) -> u32 {
        self.one + self.two + self.three
    }
}

fn parse_calc(input: &str) -> TopElf {

    let mut top_elves: TopElf = TopElf { one: 0, two: 0, three: 0 };
    let mut sum = 0;

    for line in input.lines() {
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
                top_elves.get_top_elf(sum);
                sum = 0;
        } else {
            match trimmed_line.parse::<u32>() {
                Ok(num) => {
                    sum += num
                }
                Err(e) => println!("{e}"),
            }
        }
    }
    top_elves.get_top_elf(sum);
    top_elves
}


pub fn part_one(input: &str) -> Option<u32> {
   let top_elf = parse_calc(input);
   println!("{}", top_elf.one);
    Some(top_elf.one)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut top_elves: TopElf = parse_calc(input);
    println!("{}", top_elves.sum());
    Some(top_elves.sum())
}

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, None);
    }
}

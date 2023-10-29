use std::cmp;

struct TopElf {
    one: u32,
}

pub fn part_one(input: &str) -> Option<u32> {

    let mut elves_calories = Vec::new();
    let mut elf_calories = Vec::new();

    let mut top_index = 0;
    let mut top_elf: TopElf = TopElf { one: 0 };
    let mut curr_sum = 0;

    for line in input.lines() {
        let trimmed_line = line.trim();

        if trimmed_line.is_empty(){
            if !elf_calories.is_empty(){
                elves_calories.push(elf_calories.clone());
                elf_calories.clear();
                if curr_sum > top_elf.one {
                    top_index = cmp::max(1, elves_calories.len().wrapping_sub(1));
                    top_elf.one = curr_sum;
                } 
                curr_sum = 0;
            }
        } else {       
            match line.trim().parse::<u32>() {
                Ok(num) => {
                    elf_calories.push(num); 
                    curr_sum += num;
                },
                Err(e) => println!("{e}"),
            }    
        }
    }
    if !elf_calories.is_empty(){       
        elves_calories.push(elf_calories.clone());
        if curr_sum > top_elf.one {
            top_index = cmp::max(1, elves_calories.len().wrapping_sub(1));
            top_elf.one = curr_sum;
        } 
    }
    println!("\t Elf {} has the most with {:?} calories\n", top_index, top_elf.one);
    Some(top_elf.one)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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

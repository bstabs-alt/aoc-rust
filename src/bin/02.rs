#[derive(Default, Debug)]
struct Hand {
    x: bool,
    y: bool,
    z: bool,
}

fn assign_points(mut hands: Vec<String>, mut score: u32) -> u32 {
    
    match hands.last().to_string(){
        "x" => score += 1,
        "y" => score += 2,
        "z" => score += 3,
    }
    return score
}


fn tournament(rounds: &str) -> Option<u32> {
    // let my_hand: Hand = Hand::default();
    let mut score: u32 = 0;
    for round in rounds.lines() {
        let hands: Vec<String> =  round.split(" ").map(|f| f.to_string()).collect();
        score += assign_points(hands, score);
    }
    Some(score)
} 

pub fn part_one(input: &str) -> Option<u32> {
    let score = tournament(input);
    score
}

pub fn part_two(input: &str) -> Option<u32> {
    let score = tournament(input);
    score
}

advent_of_code::main!(2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, None);
    }
}

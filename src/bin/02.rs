// #[derive(Debug)]
// enum Hand {
//     X = 1,
//     Y = 2,
//     Z = 3,
// }


fn assign_points(hands: Vec<String>, mut score: u32) -> u32 {
    // match and assign pointsr for hand
    match hands.last() {
       Some(hand)  => match hand.as_str() {
        "X" => score += 1,
        "Y" => score += 2,
        "Z" => score += 3,
        _ => {}
       },
       None => {},
    }
    return score
}


fn tournament(rounds: &str) -> Option<u32> {
    // let my_hand: Hand = Hand::default();

    // sum of my points 
    let mut score: u32 = 0;

    // each game of paper scissors rock
    for round in rounds.lines() {
        // split opponent's and my hand
        let hands: Vec<String> =  round.split(" ").map(|f| f.to_string()).collect();
        // add to score
        score += assign_points(hands, score);
    }
    //return score
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

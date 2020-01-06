// --- Day 4: Secure Container ---
//
// You arrive at the Venus fuel depot only to discover it's protected by a
// password. The Elves had written the password on a sticky note, but someone
// threw it out.
//
// However, they do remember a few key facts about the password:
//
// - It is a six-digit number.
// - The value is within the range given in your puzzle input.
// - Two adjacent digits are the same (like 22 in 122345).
// - Going from left to right, the digits never decrease; they only ever
//   increase or stay the same
//
// (like 111123 or 135679). Other than the range rule, the following are true:
//
// - 111111 meets these criteria (double 11, never decreases).
// - 223450 does not meet these criteria (decreasing pair of digits 50).
// - 123789 does not meet these criteria (no double).
//
// How many different passwords within the range given in your puzzle input meet
// these criteria?
//
// Your puzzle input is 138241-674034.
//
// Your puzzle answer was 1890.
//
// The first half of this puzzle is complete! It provides one gold star: *
//
// --- Part Two ---
//
// An Elf just remembered one more important detail: the two adjacent matching
// digits are not part of a larger group of matching digits.
//
// Given this additional criterion, but still ignoring the range rule, the following are now true:
//
// - 112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
// - 123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
// - 111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).
//
// How many different passwords within the range given in your puzzle input meet all of the criteria?
//
// Your puzzle input is still 138241-674034.
//
// Your puzzle answer was 1277.
//
// Both parts of this puzzle are complete! They provide two gold stars: **

fn is_valid_part_two(mut password: i32) -> bool {
    let mut valid = false;
    let mut repeat_count = 1;
    let mut previous = password % 10;
    password /= 10;

    while password > 0 {
        let current = password % 10;
        if current > previous {
            return false;
        }

        if current == previous {
            repeat_count += 1;
        } else {
            valid = valid || repeat_count == 2;
            repeat_count = 1;
        }
        previous = current;
        password /= 10;
    }

    valid || repeat_count == 2
}


fn is_valid_part_one(mut password: i32) -> bool {
    let mut valid = false;
    let mut previous = password % 10;
    password /= 10;

    while password > 0 {
        let current = password % 10;
        // Don't allow digits to decrease
        if current > previous {
            return false;
        }
        // Set valid flag if we have repeating digits
        valid = valid || current == previous;
        previous = current;
        password /= 10;
    }

    valid
}

fn main() {
    let valid_passwords = (138_241..=674_034)
        .filter(|password| is_valid_part_one(*password))
        .count();
    println!("Part one. Count: {}", valid_passwords);
    let valid_passwords = (138_241..=674_034)
        .filter(|password| is_valid_part_two(*password))
        .count();
    println!("Part two: Count: {}", valid_passwords);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_examples_test() {
        assert_eq!(is_valid_part_one(111111), true);
        assert_eq!(is_valid_part_one(223450), false);
        assert_eq!(is_valid_part_one(123789), false);
    }

    #[test]
    fn part_2_examples_test() {
        assert_eq!(is_valid_part_two(112233), true);
        assert_eq!(is_valid_part_two(123444), false);
        assert_eq!(is_valid_part_two(111122), true);
        assert_eq!(is_valid_part_two(112222), true);
    }
}

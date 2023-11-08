pub const START: usize = 0;

pub fn fizz_buzz(sets: &[(usize, impl ToString)], stop: usize) -> Vec<String> {
    (START..=stop)
        .map(|num| -> String {
            let total_word = sets.iter()
                .filter(|(set_num, _)| num % set_num == 0)
                .map(|(_, word)| word.to_string())
                .collect::<Vec<String>>()
                .join("");
            if total_word.len() == 0 {
                return num.to_string();
            }
            total_word
        })
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz_buzz() {
        let sets = [(3usize, "fizz"), (5, "buzz")];
        assert_eq!(
            &fizz_buzz(&sets, 5),
            &["fizzbuzz", "1", "2", "fizz", "4", "buzz"]
        );
    }

    #[test]
    fn example() {
        let sets = [(3, "fizz"), (5, "buzz")];
        println!("{}", fizz_buzz(&sets, 50).join("\n"));
    }
}

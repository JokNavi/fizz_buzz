use std::collections::HashMap;

pub const START: usize = 0;

pub fn fizz_buzz(sets: &HashMap<usize, impl ToString>, stop: usize) -> String {
    (START..=stop)
        .into_iter()
        .map(|num| match sets.get(&num) {
            Some(value) => value.to_string(),
            None => num.to_string(),
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz_buzz() {
        let sets = HashMap::from([(3, "fizz"), (5, "buzz")]);
        assert_eq!(fizz_buzz(&sets, 5), "0\n1\n2\nfizz\n4\nbuzz");
    }
}

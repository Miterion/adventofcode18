use std::collections::HashMap;

pub fn second_day(input: &String) {
    let mut two_times = 0;
    let mut three_times = 0;

    for id in input.split_whitespace() {
        let mut occurrences = HashMap::new();
        for character in id.chars() {
            let _entry = occurrences
                .entry(character)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        let vals: Vec<_> = occurrences.values().collect();
        if vals.contains(&&2) {
            two_times += 1;
        }

        if vals.contains(&&3) {
            three_times += 1;
        }
    }

    println!("The checksum is: {}", three_times * two_times);
}

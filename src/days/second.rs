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


pub fn second_day_part_two(input: &String){
    let mut list_of_words: Vec<String> = Vec::new();
    for id in input.split_whitespace() {
        for visited in list_of_words.iter() {
            let mut iterator = id.chars();
            let mut same = 0;
            for character in visited.chars(){
                if iterator.next().unwrap() == character {
                    same += 1;
                }
            }
            if same == id.len() - 1 {
                println!("Found the words: {} and {}", id, visited);
                let mut string_without_char = String::from(id);
                string_without_char.retain(|c| visited.contains(c));
                println!("String without different character: {}", string_without_char);
            }
        }
        list_of_words.push(String::from(id));
    }
}
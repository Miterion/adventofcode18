pub fn fifth_day(input: &String) -> usize {
    let mut polymer = input.clone();
    let mut length = polymer.len();
    let mut del_list: Vec<usize> = Vec::new();

    loop {
        let mut it = 0..length - 2;
        while let Some(i) = it.next() {
            let slice: Vec<_> = polymer[i..i + 2].chars().collect();
            if (slice[0] == slice[1].to_ascii_uppercase()
                || slice[0] == slice[1].to_ascii_lowercase())
                && slice[0] != slice[1]
            {
                del_list.push(i);
                it.next();
            }
        }

        let mut del_pul = 0;
        for i in del_list.drain(0..) {
            polymer.remove(i - del_pul);
            polymer.remove(i - del_pul);
            del_pul += 2;
        }
        if length == polymer.len() {
            break;
        }
        length = polymer.len();
    }

    println!("The final length is {}", length);
    return length;
}

pub fn fifth_day_part_two(input: &String) {
    static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];
    let mut smallest: usize = input.len();
    for letter in ASCII_LOWER.iter() {
        let mut polymer = input.clone();
        polymer.retain(|c| c != *letter && c != letter.to_ascii_uppercase());
        let length = fifth_day(&polymer);
        if length < smallest {
            smallest = length;
        }
    }
    println!("Smalles possible length is {}", smallest);
}

pub fn first_day(input: &String){
    let mut start_freq = 0;
    for number in input.split_whitespace() {
        start_freq += number.parse::<i32>().unwrap();
    }
    println!("The resulting number is: {}", start_freq);
}

pub fn first_day_part_two(input: &String){
    let mut v: Vec<i32> = Vec::new();
    let mut freq = 0;
    loop {
    for number in input.split_whitespace() {
        freq += number.parse::<i32>().unwrap();
        match v.binary_search(&freq){
            Ok(_) => {
                println!("The frequency {} occurred twice.", freq);
                return;
            },
            Err(index)  => v.insert(index, freq),
        }
    }
    }
}
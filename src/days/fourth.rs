use chrono::{NaiveDate, NaiveDateTime, Timelike};
use std::collections::HashMap;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
struct Event {
    timestamp: NaiveDateTime,
    text: String,
}
#[derive(Debug, Clone, Copy)]
struct Shift {
    guard: i32,
    sleep_start: NaiveDateTime,
    sleep_end: NaiveDateTime,
}

pub fn fourth_day(input: &String) {
    let mut events: Vec<Event> = Vec::new();
    create_event_list(input, &mut events);

    let mut shifts: HashMap<i32, Vec<Shift>> = HashMap::new();
    
    create_shifts(&events, &mut shifts);

    let mut guard_sleep: HashMap<i32, i64> = HashMap::new();
    for (id, shifts) in shifts.iter() {
        let mut sleep_duration = 0;
        for shift in shifts {
            sleep_duration += (shift.sleep_end - shift.sleep_start).num_minutes();
        }
        guard_sleep.entry(*id).or_insert(sleep_duration);
    }
    let mut id_highest = 0;
    let mut sleep_count: i64 = 0;
    for (id, sleeptime) in guard_sleep.iter() {
        if *sleeptime > sleep_count {
            id_highest = *id;
            sleep_count = *sleeptime;
        }
    }
    println!(
        "Seleted id {} with total sleeptime of {} minutes",
        id_highest, sleep_count
    );

    let mut asleep_time = [0; 60];
    for shift in shifts[&id_highest].iter() {
        for i in shift.sleep_start.minute()..shift.sleep_end.minute() + 1 {
            asleep_time[i as usize] += 1;
        }
    }
    let mut highest = 0;
    let mut minute = 0;
    for i in 0..60 {
        if asleep_time[i] > highest {
            highest = asleep_time[i];
            minute = i;
        }
    }
    println!("Highest minute is {}", minute);

    println!("Multiplied: {}", minute as i32 * id_highest);
}

pub fn fourth_day_part_two(input: &String) {
    let mut events: Vec<Event> = Vec::new();
    create_event_list(input, &mut events);

    let mut shifts: HashMap<i32, Vec<Shift>> = HashMap::new();
    create_shifts(&events, &mut shifts);

    let mut guard_sleeps: HashMap<i32, [u32; 60]> = HashMap::new();

    for (id, shifts) in shifts.iter() {
        let mut array = [0u32; 60];
        for shift in shifts {
            for i in shift.sleep_start.minute()..shift.sleep_end.minute() {
                array[i as usize] += 1;
            }
        }
        guard_sleeps.insert(*id, array);
    }

    let mut highest_guard: i32 = 0;
    let mut highest_minute = 0;
    let mut highest_count: u32 = 0;

    for (id, array) in guard_sleeps.iter() {
        for (pos, value) in array.iter().enumerate() {
            if value > &highest_count {
                highest_guard = *id;
                highest_minute = pos;
                highest_count = *value;
            }
        }
    }

    println!("The guard {} has the highest sleep count ({}) at minute {}", highest_guard, highest_count, highest_minute);
    println!("The answer is therefore: {}", highest_guard * (highest_minute as i32));
}

fn create_event_list(input: &String, events: &mut Vec<Event>) {
    for line in input.lines() {
        let splitted: Vec<_> = line.split(" ").collect();
        let timestamp =
            NaiveDateTime::parse_from_str(&splitted[0..2].join(" "), "[%F %R]").unwrap();
        let text = String::from(splitted[2..].join(" "));
        events.push(Event { timestamp, text });
    }
    events.sort_unstable();
}

fn create_shifts(events: & Vec<Event>, shifts: &mut HashMap<i32, Vec<Shift>>) {
    // First shift
    let mut sleep_start: NaiveDateTime = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);
    let mut sleep_end: NaiveDateTime = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);
    let mut guard: i32 = get_id(&events[0].text);
    for event in events[1..].iter() {
        match event.text.as_ref() {
            "wakes up" => {
                sleep_end = event.timestamp;
                let shift = Shift {
                    guard,
                    sleep_start,
                    sleep_end,
                };
                shifts
                    .entry(guard)
                    .and_modify(|guard| guard.push(shift))
                    .or_insert(vec![shift]);
            }
            "falls asleep" => sleep_start = event.timestamp,
            _ => {
                guard = get_id(&event.text);
            }
        }
    }
}

fn get_id(textstring: &String) -> i32 {
    let str_id = textstring.split_whitespace().collect::<Vec<_>>()[1];
    return str_id[1..].parse::<i32>().unwrap();
}

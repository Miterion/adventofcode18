struct Claim {
    start_x: usize,
    start_y: usize,
    size_x: usize,
    size_y: usize,
}

struct ClaimIter<'a> {
    fabric: &'a [[i32; 1001]; 1001],
    claim: &'a Claim,
    cur_x: usize,
    cur_y: usize,
}

impl<'a> ClaimIter<'a> {
    fn new(fabric: &'a [[i32; 1001]; 1001], claim: &'a Claim) -> ClaimIter<'a> {
        ClaimIter {
            fabric,
            claim,
            cur_x: claim.start_x,
            cur_y: claim.start_y,
        }
    }
}

impl<'a> Iterator for ClaimIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.cur_x != self.claim.start_x + self.claim.size_x
            && self.cur_y != self.claim.start_y + self.claim.size_y
        {
            let ret = Some(self.fabric[self.cur_x][self.cur_y]);

            if self.cur_y + 1 == self.claim.start_y + self.claim.size_y {
                self.cur_x += 1;
                self.cur_y = self.claim.start_y;
            } else {
                self.cur_y += 1;
            }
            return ret;
        } else {
            return None;
        }
    }
}

pub fn third_day(input: &String) {
    let mut fabric = [[0i32; 1001]; 1001];

    create_fabric(input, &mut fabric);

    let mut overlapped_fields = 0;
    for row in fabric.iter() {
        for column in row.iter() {
            if *column >= 2 {
                overlapped_fields += 1;
            }
        }
    }

    println!(
        "There are {} fields used more than once.",
        overlapped_fields
    );
}

pub fn third_day_part_two(input: &String) {
    let mut fabric = [[0i32; 1001]; 1001];
    create_fabric(input, &mut fabric);

    for line in input.lines() {
        let claim = line_to_claim(line);
        let iter = ClaimIter::new(&fabric, &claim);
        let mut found = true;
        for el in iter {
            if el != 1 {
                found = false;
            }
        }
        if found {
            println!(
                "Found the claim: {}",
                line.split(" ").collect::<Vec<_>>()[0]
            )
        }
    }
}

fn create_fabric(input: &str, fabric: &mut [[i32; 1001]; 1001]) {
    for row in 0..1001 {
        for column in 0..1001 {
            fabric[row][column] = 0;
        }
    }
    for line in input.lines() {
        let claim = line_to_claim(line);

        let mut cur_x = claim.start_x;
        let mut cur_y = claim.start_y;

        while cur_x != claim.start_x + claim.size_x && cur_y != claim.start_y + claim.size_y {
            fabric[cur_x][cur_y] += 1;

            if cur_y + 1 == claim.start_y + claim.size_y {
                cur_x += 1;
                cur_y = claim.start_y;
            } else {
                cur_y += 1;
            }
        }
    }
}

fn line_to_claim(line: &str) -> Claim {
    let splitted_claim: Vec<_> = line.split(" ").collect();
    let claim_coordinates: Vec<_> = splitted_claim[2].split(",").collect();
    let claim_field: Vec<_> = splitted_claim[3].split("x").collect();

    let start_x = claim_coordinates[0].parse::<usize>().unwrap();
    let start_y = claim_coordinates[1]
        .trim_end_matches(":")
        .parse::<usize>()
        .unwrap();

    let size_x = claim_field[0].parse::<usize>().unwrap();
    let size_y = claim_field[1].parse::<usize>().unwrap();

    Claim {
        start_x,
        start_y,
        size_x,
        size_y,
    }
}

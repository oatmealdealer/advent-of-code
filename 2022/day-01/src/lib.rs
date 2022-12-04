pub fn process_part1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|load| {
            load.split("\n")
                .map(|n| n.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>()
        .iter()
        .max()
        .unwrap()
        .to_owned()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    input
        .split("\n\n")
        .map(|load| load.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>()
        .iter()
        .fold([0, 0, 0], |mut top: [u32; 3], n| {
            let min = top.iter_mut().min().unwrap();
            *min = *n.max(min);
            top
        })
        .iter()
        .sum::<u32>()
        .to_string()
}

fn calculate_fuel(mass: u64) -> u64 {
    let calc_fuel = |mass: u64| Some((mass / 3).saturating_sub(2));
    std::iter::successors(calc_fuel(mass), |&fuel| calc_fuel(fuel))
        .take_while(|&fuel| fuel != 0)
        .sum::<u64>()
}

fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .filter_map(|x| x.parse().ok())
            .map(calculate_fuel)
            .sum::<u64>()
    );
}

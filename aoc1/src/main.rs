fn mass_to_fuel(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

fn total_fuel(mass: u64) -> u64 {
    std::iter::successors(Some(mass_to_fuel(mass)), |&x| Some(mass_to_fuel(x)))
        .take_while(|&x| x != 0)
        .sum::<u64>()
}

fn main() {
    let (fuel_mass, total_fuel) = include_str!("../input.txt")
        .lines()
        .filter_map(|x| x.parse().ok())
        .map(|mass| (mass_to_fuel(mass), total_fuel(mass)))
        .fold((0, 0), |acc, val| (acc.0 + val.0, acc.1 + val.1));
    println!("part 1: {}", fuel_mass);
    println!("part 2: {}", total_fuel);
}

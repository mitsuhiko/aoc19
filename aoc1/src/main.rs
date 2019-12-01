#[derive(Debug, Copy, Clone)]
struct Module {
    pub mass: u64,
}

fn mass_to_fuel(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

impl Module {
    pub fn launch_fuel(self) -> u64 {
        mass_to_fuel(self.mass)
    }

    pub fn total_fuel(self) -> u64 {
        std::iter::successors(Some(self.launch_fuel()), |&fuel| {
            match mass_to_fuel(fuel) {
                0 => None,
                extra => Some(extra)
            }
        }).sum::<u64>()
    }
}

fn main() {
    println!(
        "{}",
        include_str!("../input")
            .lines()
            .filter_map(|x| x.parse().ok())
            .map(|mass| Module { mass }.total_fuel())
            .sum::<u64>()
    );
}

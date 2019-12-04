use smallvec::SmallVec;

fn reverse_digit_iterator(mut num: u32) -> impl Iterator<Item = u8> {
    std::iter::from_fn(move || match num {
        0 => None,
        _ => {
            let digit = num % 10;
            num /= 10;
            Some(digit as _)
        }
    })
}

fn check_part1(num: u32) -> bool {
    let mut last_digit = !0;
    let mut found_duplicate = false;

    for digit in reverse_digit_iterator(num) {
        if digit > last_digit {
            return false;
        } else if digit == last_digit {
            found_duplicate = true;
        }
        last_digit = digit;
    }

    found_duplicate
}

fn check_part2(num: u32) -> bool {
    let mut last_digit = !0;
    let mut groups = SmallVec::<[u8; 6]>::new();

    for digit in reverse_digit_iterator(num) {
        if digit > last_digit {
            return false;
        } else if digit == last_digit {
            *groups.last_mut().unwrap() += 1;
        } else {
            groups.push(0);
        }
        last_digit = digit;
    }

    groups.into_iter().any(|x| x == 1)
}

fn complexity<F: Fn(u32) -> bool>(start: u32, end: u32, func: F) -> usize {
    (start..=end).filter(|&x| func(x)).count()
}

fn main() {
    println!("part 1: {}", complexity(178416, 676461, check_part1));
    println!("part 2: {}", complexity(178416, 676461, check_part2));
}

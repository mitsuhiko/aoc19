use std::cmp::Ordering;

fn reverse_digit_iterator(mut num: u32) -> impl Iterator<Item = u8> {
    std::iter::from_fn(move || match num {
        0 => None,
        _ => Some({
            let digit = num % 10;
            num /= 10;
            digit as _
        }),
    })
}

fn check_part1(num: u32) -> bool {
    reverse_digit_iterator(num)
        .try_fold((!0, false), |(last, good), cur| match cur.cmp(&last) {
            Ordering::Equal => Some((cur, true)),
            Ordering::Less => Some((cur, good)),
            Ordering::Greater => None,
        })
        .map_or(false, |(_, good)| good)
}

fn check_part2(num: u32) -> bool {
    reverse_digit_iterator(num)
        .try_fold((0, !0, false), |(n, last, ok), cur| match cur.cmp(&last) {
            Ordering::Equal => Some((n + 1, cur, ok)),
            Ordering::Less => Some((0, cur, ok || n == 1)),
            Ordering::Greater => None,
        })
        .map_or(false, |(n, _, ok)| ok || n == 1)
}

fn complexity<F: Fn(u32) -> bool>(start: u32, end: u32, func: F) -> usize {
    (start..=end).filter(|&x| func(x)).count()
}

fn main() {
    println!("part 1: {}", complexity(178_416, 676_461, check_part1));
    println!("part 2: {}", complexity(178_416, 676_461, check_part2));
}

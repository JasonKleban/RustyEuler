
extern crate num;

pub fn euler4() -> i32 {
    num::range_step(999, 0, -1)
        .flat_map(move |a| num::range_step(a, 0, -1).map(move |b| a * b))
        .filter(move |a_x_b| is_palendrome(*a_x_b))
        .max()
        .unwrap()
}

pub fn euler5() -> i32 {
    (20..)
        .find(|n| (1..21).all(|d| n % d == 0))
        .unwrap()
}

pub fn euler6() -> i32 {
    let (sum, sum_of_squares) = (1..101).fold((0, 0), |(s, ss), n| (s + n, ss + (n * n)));
    (sum * sum) - sum_of_squares
}

fn is_palendrome(cand_palendrome: i32) -> bool {
    let as_str: String = format!("{0}", cand_palendrome);
    let reversed: String = as_str.chars().rev().collect();

    if let Ok(rev) = reversed.parse::<i32>() {
        rev == cand_palendrome
    } else {
        false
    }
}
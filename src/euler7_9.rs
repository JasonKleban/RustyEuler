
extern crate num;

pub fn euler7() -> i32 {
    (0..)
        .filter(move |&x| gpf(x) == 1)
        .nth(10_001)
        .unwrap()
}

fn gpf(cand_composite: i32) -> i32 {
    if cand_composite == 1 {
        1
    }
    else if cand_composite % 2 == 0 {
        cand_composite / 2
    } else {
        num::range_step((cand_composite as f32).sqrt().floor() as i32, 0, -1)
            .map(|x| {
                //println!("{0} % {1} == {2}", cand_composite, x, cand_composite % x);
                x
            })
            .find(|cand_fact| if cand_composite % cand_fact == 0 && gpf(*cand_fact) == 1 {
                true
            } else {
                false
            })
            .unwrap()
    }
}
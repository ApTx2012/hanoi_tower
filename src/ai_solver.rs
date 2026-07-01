pub fn hanoi_solve(
    count: u8,
    source: usize,
    target: usize,
    assist: usize,
    steps: &mut Vec<(usize, usize)>,
) {
    if count == 0 {
        return;
    }
    hanoi_solve(count - 1, source, assist, target, steps);
    steps.push((source, target));
    hanoi_solve(count - 1, assist, target, source, steps);
}

pub fn get_all_moves(gear_num: u8) -> Vec<(usize, usize)> {
    let mut steps = Vec::new();
    hanoi_solve(gear_num, 0, 2, 1, &mut steps);
    steps
}
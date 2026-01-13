use checkers_rs::ai::formatting::square;

fn lcg(seed: &mut u32) -> usize {
    *seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    (*seed % 8) as usize
}

#[test]
fn square_formats_random_coordinate() {
    let mut seed = 7u32;
    let row = lcg(&mut seed);
    let col = lcg(&mut seed);
    let notation = square(row, col);
    let expected = format!("{}{}", (b'A' + col as u8) as char, 8 - row);
    assert_eq!(notation, expected, "Square formatting mismatch");
}

use checkers_rs::ai::genai_client::GeminiAI;

fn seedvalue(seed: &mut u32, limit: usize) -> usize {
    *seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
    ((*seed % limit as u32) + 1) as usize
}

#[test]
fn parse_accepts_numeric_choice_with_unicode_noise() {
    let mut seed = 3u32;
    let limit = seedvalue(&mut seed, 5);
    let response = format!("Выбор №{} 🎯", limit);
    let parsed = GeminiAI::parse(&response, 5).unwrap();
    assert_eq!(parsed, limit, "Parser rejected valid numeric choice");
}

use cchol_lib::skill::generate_unusual_skills;

#[test]
fn unusual_skill_generation() {
    let sks = generate_unusual_skills();
    assert!(!sks.is_empty());
}
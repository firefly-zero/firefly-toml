use crate::*;

#[test]
fn end_to_end() {
    run("end_to_end");
}

#[test]
fn empty() {
    run("empty");
}

fn run(name: &str) {
    let input_path = format!("test_data/{name}.toml");
    let input = std::fs::read_to_string(input_path).unwrap();
    let config: Config = toml::from_str(&input).unwrap();
    let mut s = String::new();
    write_badges(config.badges.unwrap_or_default(), &mut s);
    write_cheats(config.cheats.unwrap_or_default(), &mut s);
    write_boards(config.boards.unwrap_or_default(), &mut s);
    write_palettes(config.palettes.unwrap_or_default(), &mut s);
    let snapshot_path = format!("test_data/{name}.rs");
    snapshot_testing::assert_eq_or_update(s, snapshot_path);
}

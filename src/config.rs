use proc_macro::TokenStream;
use serde::Deserialize;
use std::collections::HashMap;
use std::{fmt::Write as _, str::FromStr as _};

#[derive(Deserialize, Debug)]
pub struct BoardConfig {
    /// Human-readable board name.
    pub name: String,
}

pub fn write_boards(boards: HashMap<String, BoardConfig>, s: &mut String) {
    if boards.is_empty() {
        return;
    }
    writeln!(s, "mod boards {{").unwrap();
    writeln!(s, "use firefly_rust::{{Peer, add_score, Progress, Board}};").unwrap();
    for (id, board) in boards {
        let name = board.name.replace(' ', "_");
        writeln!(
            s,
            "pub fn {name}(peer: Peer, score: i16) -> i16 {{ add_score(peer, Board({id}), score) }}",
        )
        .unwrap();
    }
    s.push('}');
}

pub fn write_cheats(cheats: HashMap<String, i32>, s: &mut String) {
    if cheats.is_empty() {
        return;
    }
    writeln!(s, "enum Cheats {{").unwrap();
    for (name, id) in &cheats {
        let name = name.replace('-', "_");
        writeln!(s, "{name} = {id},",).unwrap();
    }
    s.push('}');
    writeln!(
        s,
        "impl Cheats {{ fn from_id(id: i32) -> Self {{ match id {{"
    )
    .unwrap();
    for (name, id) in cheats {
        let name = name.replace('-', "_");
        writeln!(s, "{id} => Self::{name},",).unwrap();
    }
    writeln!(s, "_ => unreachable!(),").unwrap();
    writeln!(s, "}} }} }}").unwrap();
}

#[derive(Deserialize, Debug)]
pub struct BadgeConfig {
    /// Human-readable achievement name.
    pub name: String,

    /// Human-readable achievement description. Typically, a hint on how to earn it.
    #[serde(default)]
    pub descr: String,

    /// How many steps there are to earn the badge.
    ///
    /// Defaults to 1.
    pub steps: Option<u16>,
}

pub fn write_badges(badges: HashMap<String, BadgeConfig>, s: &mut String) {
    if badges.is_empty() {
        return;
    }
    writeln!(s, "mod badges {{").unwrap();
    writeln!(
        s,
        "use firefly_rust::{{Badge, Peer, add_progress, Progress, Board}};"
    )
    .unwrap();
    for (id, badge) in badges {
        writeln!(s, "/// {}", badge.descr).unwrap();
        let name = badge.name.replace(' ', "_");
        let (arg, n) = match badge.steps {
            Some(1) | None => ("", "1"),
            Some(_) => ("arg: u32", "arg"),
        };
        writeln!(
            s,
            "pub fn {name}(peer: Peer, {arg}) -> Progress {{ add_progress(peer, Badge({id}), {n}) }}",
        )
        .unwrap();
    }
    s.push('}');
}

#[derive(Deserialize, Debug)]
pub struct Config {
    /// Mapping of cheat commands to their integer representation.
    pub cheats: Option<HashMap<String, i32>>,

    /// Mapping of badge IDs to badges.
    pub badges: Option<HashMap<String, BadgeConfig>>,

    /// Mapping of board IDs to boards.
    pub boards: Option<HashMap<String, BoardConfig>>,
}

pub fn load_config() -> Result<Config, TokenStream> {
    let file = std::fs::read("firefly.toml").map_err(|e| {
        TokenStream::from_str(&format!(
            r##"compile_error!(r#"could not load firefly.toml: {e}"#);"##
        ))
        .unwrap()
    })?;
    toml::from_slice(&file).map_err(|e| {
        TokenStream::from_str(&format!(
            r##"compile_error!(r#"firefly.toml is not valid toml: {e}"#);"##
        ))
        .unwrap()
    })
}

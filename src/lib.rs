#![deny(clippy::pedantic)]
#![allow(clippy::wildcard_imports)]

mod config;
#[cfg(test)]
mod tests;

use config::*;

#[proc_macro]
#[expect(clippy::missing_panics_doc)]
pub fn import(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if !item.is_empty() {
        return r#"compile_error!("firefly_import_toml does not take any arguments");"#
            .parse()
            .unwrap();
    }
    let config = match load_config() {
        Ok(value) => value,
        Err(err) => return err,
    };
    let mut s = String::new();
    write_badges(config.badges.unwrap_or_default(), &mut s);
    write_cheats(config.cheats.unwrap_or_default(), &mut s);
    write_boards(config.boards.unwrap_or_default(), &mut s);
    write_palettes(config.palettes.unwrap_or_default(), &mut s);

    s.parse().unwrap()
}

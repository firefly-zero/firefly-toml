mod config;
#[cfg(test)]
mod tests;

use config::*;

#[proc_macro]
pub fn import(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if !item.is_empty() {
        return r##"compile_error!("firefly_import_toml does not take any arguments");"##
            .parse()
            .unwrap();
    }
    let Config {
        cheats,
        badges,
        boards,
    } = match load_config() {
        Ok(value) => value,
        Err(err) => return err,
    };
    let mut s = String::new();
    write_badges(badges.unwrap_or_default(), &mut s);
    write_cheats(cheats.unwrap_or_default(), &mut s);
    write_boards(boards.unwrap_or_default(), &mut s);

    s.parse().unwrap()
}

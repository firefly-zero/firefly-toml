mod badges {
use firefly_rust::{Badge, Peer, add_progress, Progress, Board};
/// caused an unsoundness without unsafe code
pub fn segfault(peer: Peer, ) -> Progress { add_progress(peer, Badge(1), 1) }
/// found a secret
pub fn secret(peer: Peer, ) -> Progress { add_progress(peer, Badge(2), 1) }
}enum Cheats {
goto_level = 1,
}impl Cheats { fn from_id(id: i32) -> Self { match id {
1 => Self::goto_level,
_ => unreachable!(),
} } }
mod boards {
use firefly_rust::{Peer, add_score, Progress, Board};
pub fn level_1(peer: Peer, score: i16) -> i16 { add_score(peer, Board(1), score) }
pub fn level_2(peer: Peer, score: i16) -> i16 { add_score(peer, Board(2), score) }
pub fn speedrun(peer: Peer, score: i16) -> i16 { add_score(peer, Board(3), score) }
}mod palettes {
use firefly_rust::{set_color, Color};
pub fn custom() {
set_color(Color::new(1), RGB::new(13, 13, 13));
set_color(Color::new(2), RGB::new(153, 153, 153));
set_color(Color::new(3), RGB::new(242, 242, 242));
set_color(Color::new(4), RGB::new(20, 20, 20));
set_color(Color::new(5), RGB::new(13, 13, 77));
set_color(Color::new(6), RGB::new(13, 13, 93));
set_color(Color::new(7), RGB::new(13, 13, 109));
set_color(Color::new(8), RGB::new(13, 13, 125));
set_color(Color::new(9), RGB::new(13, 13, 141));
set_color(Color::new(10), RGB::new(13, 13, 157));
set_color(Color::new(11), RGB::new(13, 13, 173));
set_color(Color::new(12), RGB::new(13, 13, 189));
set_color(Color::new(13), RGB::new(13, 13, 205));
set_color(Color::new(14), RGB::new(13, 13, 221));
set_color(Color::new(15), RGB::new(13, 13, 237));
set_color(Color::new(16), RGB::new(0, 0, 0));
}
}
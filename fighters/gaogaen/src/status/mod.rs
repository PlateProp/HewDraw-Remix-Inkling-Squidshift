use super::*;
use globals::*;
// status script import

mod attack_lw4;
mod special_n;

pub fn install(agent: &mut Agent) {
    attack_lw4::install(agent);
    special_n::install(agent);
}

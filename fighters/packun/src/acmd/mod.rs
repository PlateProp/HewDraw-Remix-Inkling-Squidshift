use super::*;

mod ground;
mod tilts;
mod smashes;
mod aerials;
mod specials;
mod throws;
mod other;

#[repr(C)]
pub struct StanceInfo {
    label: i32,
    damage_bite: f32,
    damage_head: f32,
    damage_other: f32,
    da_speed: f32
}

impl From<i32> for StanceInfo {
    fn from(other: i32) -> Self {
        match other {
            0 => StanceInfo { // Regular
                label: 0,
                damage_bite: 0.9,
                damage_head: 0.9,
                damage_other: 0.9,
                da_speed: 0.8
            },
            1 => StanceInfo { // Putrid
                label: 1,
                damage_bite: 0.85,
                damage_head: 0.65,
                damage_other: 0.75,
                da_speed: (0.8 * 0.86)
            },
            2 => StanceInfo { // Prickly
                label: 2,
                damage_bite: 1.05,
                damage_head: 1.3,
                damage_other: 1.15,
                da_speed: (0.8 * 0.84)
            },
            _ => StanceInfo { // same as regular
                label: 3,
                damage_bite: 0.9,
                damage_head: 0.9,
                damage_other: 0.9,
                da_speed: 0.8
            },
        }
    }
}

pub fn install(agent: &mut Agent) {
    ground::install(agent);
    tilts::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    specials::install(agent);
    throws::install(agent);
    other::install(agent);
}
use super::*;

// FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ESCAPE

pub unsafe extern "C" fn special_hi_escape_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR);
    return 1.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ESCAPE, special_hi_escape_pre);
}
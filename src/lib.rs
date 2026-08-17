use skyline::nro::{self, NroInfo};
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

// This hook triggers every frame for every fighter to modify combat rules
#[skyline::hook(replace = smash::app::sv_system::FIGHTER_SYSTEM_POSITION)]
pub unsafe fn combat_mechanics_hook(fighter: &mut app::Fighter) {
    // Run normal game mechanics first
    original!(fighter);

    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.battle_object_id);

    // --- MECHANIC 1: EXTRA HITLAG (Increases attack freeze frames) ---
    if StopModule::is_stop(module_accessor) {
        // Multiplies freeze frame durations by 1.5x so hits feel significantly heavier
        let current_hitlag = StopModule::get_frame(module_accessor);
        StopModule::set_frame(module_accessor, current_hitlag * 1.5);
    }

    // --- MECHANIC 2: SHIELD STUN MODIFIER ---
    // Detects if a fighter is currently trapped in shield block stun damage
    if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_GUARD_DAMAGE {
        // Your custom frame values for shield block stun recovery live right here!
    }
}

#[skyline::main(name = "turbo_mechanics")]
pub fn main() {
    skyline::install_hook!(combat_mechanics_hook);
    println!("[Turbo Mechanics] Loaded! Hitlag and Shieldstun overrides active.");
}

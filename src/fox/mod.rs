use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*}
};

const FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_IN_AIR_LASER : i32 = 0x2000010b;

// Char opff
unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let status = StatusModule::status_kind(module_accessor);
    let fighter = utility::get_kind(module_accessor);
    let situation = StatusModule::situation_kind(module_accessor);
    let jump_count = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    let jump_count_max = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);

    if fighter == FIGHTER_KIND_FOX{
        // auto cancel laser
        if WorkModule::is_flag(module_accessor, FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_IN_AIR_LASER){
            if situation == SITUATION_KIND_GROUND{
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_LANDING_LIGHT, true);
            }
        }
        // fast fall laser and removing the autocancel laser flag
        if status == *FIGHTER_STATUS_KIND_SPECIAL_N{
            if situation == *SITUATION_KIND_AIR {
                KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                // check stick position and if then character is falling
                if (ControlModule::get_stick_y(module_accessor) < -0.66) && (KineticModule::get_sum_speed_y(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0){
                    // turn on fastfall
                    WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
            }
        } else {
            WorkModule::off_flag(module_accessor, FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_IN_AIR_LASER);
        }


    }
}

// Game acmd script
unsafe extern "C" fn fox_specialairnstart(agent: &mut L2CAgentBase) {
    let lsa = agent.lua_state_agent;
    let module_accessor = agent.module_accessor;

    frame(lsa, 1.0);
    if macros::is_excute(agent){
        WorkModule::on_flag(module_accessor, FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_IN_AIR_LASER);
    }

    frame(lsa, 4.0);
    if macros::is_excute(agent){
        if ArticleModule::is_exist(module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("open"), false, 0.0);
        }
    }

}

pub fn install() {
    Agent::new("fox")
        .game_acmd("game_specialairnstart", fox_specialairnstart, Default) // Game acmd script
        .on_line(Main, fighter_frame) // Char opff
        //.status(Main, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, example_status_script) // Status script
        .install();
}

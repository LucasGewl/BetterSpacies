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
const FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_CAN_JUMP_OUT_OF_SHINE : i32 = 0x20000113;

// Char opff
unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let status = StatusModule::status_kind(module_accessor);
    let fighter = utility::get_kind(module_accessor);
    let situation = StatusModule::situation_kind(module_accessor);
    let jump_count = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    let jump_count_max = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);

    if fighter == FIGHTER_KIND_FOX{
        // jump out of shine code
        if WorkModule::is_flag(module_accessor, FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_CAN_JUMP_OUT_OF_SHINE){
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) && jump_count < jump_count_max{
                if situation == SITUATION_KIND_AIR{
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                }
                else {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
                WorkModule::off_flag(module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_CAN_JUMP_OUT_OF_SHINE);
                println!("Jump out of shine")
            }
        }

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

unsafe extern "C" fn fox_speciallwstart(agent: &mut L2CAgentBase) {
    let lsa = agent.lua_state_agent;
    let module_accessor = agent.module_accessor;

    macros::FT_MOTION_RATE_RANGE(agent, 0.0, 3.0, 1.0); // should mmake shine frame 1 lol

    // frame 1 in game hopefully
    frame(lsa, 3.0);
    if macros::is_excute(agent){
        macros::ATTACK(agent,0, 0, Hash40::new("top"), 2.0, 10, 32, 0, 66, 7.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent,1, 0, Hash40::new("top"), 2.0, 24, 45, 0, 66, 7.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }

    frame(lsa, 7.0);
    if macros::is_excute(agent){
        WorkModule::on_flag(module_accessor, FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_CAN_JUMP_OUT_OF_SHINE);
    }
}

unsafe extern "C" fn fox_specialairlwstart(agent: &mut L2CAgentBase) {
    let lsa = agent.lua_state_agent;
    let module_accessor = agent.module_accessor;

    macros::FT_MOTION_RATE_RANGE(agent, 0.0, 3.0, 1.0); // should mmake shine frame 1 lol

    // frame 1 in game hopefully
    frame(lsa, 3.0);
    if macros::is_excute(agent){
        macros::ATTACK(agent,0, 0, Hash40::new("top"), 2.0, 10, 32, 0, 66, 7.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent,1, 0, Hash40::new("top"), 2.0, 24, 45, 0, 66, 7.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }

    frame(lsa, 7.0);
    if macros::is_excute(agent){
        WorkModule::on_flag(module_accessor, FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_CAN_JUMP_OUT_OF_SHINE);
    }
}

pub fn install() {
    Agent::new("fox")
        .game_acmd("game_specialairnstart", fox_specialairnstart, Default) // Game acmd script
        .game_acmd("game_speciallwstart", fox_speciallwstart, Default)
        .game_acmd("game_specialairlwstart", fox_specialairlwstart, Default)
        .on_line(Main, fighter_frame) // Char opff
        //.status(Main, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, example_status_script) // Status script
        .install();
}

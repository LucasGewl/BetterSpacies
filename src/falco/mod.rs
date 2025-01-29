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

const FIGHTER_FALCO_INSTANCE_WORK_ID_FLAG_IN_AIR_LASER : i32 = 0x2000010b;

// Char opff
unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let status = StatusModule::status_kind(module_accessor);
    let fighter = utility::get_kind(module_accessor);
    let situation = StatusModule::situation_kind(module_accessor);
    let jump_count = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    let jump_count_max = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);

    if fighter == FIGHTER_KIND_FALCO{
        // auto cancel laser
        if WorkModule::is_flag(module_accessor, FIGHTER_FALCO_INSTANCE_WORK_ID_FLAG_IN_AIR_LASER){
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
            WorkModule::off_flag(module_accessor, FIGHTER_FALCO_INSTANCE_WORK_ID_FLAG_IN_AIR_LASER);
        }


    }
}

// Game acmd script
unsafe extern "C" fn falco_specialairnstart(agent: &mut L2CAgentBase) {
    let lsa = agent.lua_state_agent;
    let module_accessor = agent.module_accessor;

    macros::FT_MOTION_RATE_RANGE(agent, 0.0, 4.0, 9.0);

    frame(lsa, 1.0);
    if macros::is_excute(agent){
        WorkModule::on_flag(module_accessor, FIGHTER_FALCO_INSTANCE_WORK_ID_FLAG_IN_AIR_LASER);
    }

    frame(lsa, 4.0);
    if macros::is_excute(agent){
        ArticleModule::generate_article(module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, false, 0);
        if ArticleModule::is_exist(module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("open"), false, 0.0);
        }
    }
}

// unchanged from vanilla, leftover from attempted fix
unsafe extern "C" fn falco_specialairnloop(agent: &mut L2CAgentBase){
    let lsa = agent.lua_state_agent;
    let module_accessor = agent.module_accessor;
    
    if macros::is_excute(agent){
        WorkModule::on_flag(module_accessor, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }

    frame(lsa, 4.0);
    if macros::is_excute(agent){
        ArticleModule::generate_article(module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, 0);
        if ArticleModule::is_exist(module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, smash::phx::Hash40::new("open"), false, 0.0);
        }
    }

    frame(lsa, 7.0);
    macros::FT_MOTION_RATE(agent, 0.6);

    frame(lsa, 17.0);
    if macros::is_excute(agent){
        WorkModule::off_flag(module_accessor, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }

}

unsafe extern "C" fn falco_attackairlw(agent: &mut L2CAgentBase) {
    let lsa = agent.lua_state_agent;
    let module_accessor = agent.module_accessor;

    frame(lsa, 1.0);
    macros::FT_MOTION_RATE_RANGE(agent, 1.0, 10.0, 4.0); // inputs: agent, start, end, game frames until end frame
    // frame 2 in game
    frame(lsa, 1.88);
    if macros::is_excute(agent){
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    // frame 5 in game
    frame(lsa, 10.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent){
        macros::ATTACK(agent,0, 0, Hash40::new("kneel"), 13.0, 280, 80, 0, 10, 4.2, 4.2, 0.0, 1.0, None, None, None, 1.5, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent,1, 0, Hash40::new("kneel"), 13.0, 80, 50, 0, 55, 4.2, 4.2, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(module_accessor, 0, 10.0, false); // give falcos dair extra hitstun vs airborne opponents
    }
    // frame 15 in game
    frame(lsa, 20.0);
    if macros::is_excute(agent){
        macros::ATTACK(agent,0, 0, Hash40::new("kneel"), 8.0, 280, 90, 0, 20, 5.3, 3.5, 0.0, 1.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::clear(module_accessor, 1, false); // always set bool to false, apparently perma disables that id for match if true
    }

    // frame 24 in game
    frame(lsa, 29.0);
    if macros::is_excute(agent){
        AttackModule::clear_all(module_accessor);
    }

    // frame 30 in game
    frame(lsa, 35.0);
    macros::FT_MOTION_RATE_RANGE(agent, 35.0, 52.0, 14.0);
    if macros::is_excute(agent){
        WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// // Status script
// unsafe extern "C" fn example_status_script(fighter: &mut L2CFighterCommon) -> L2CValue {
//     0.into()
// }

pub fn install() {
    Agent::new("falco")
        .game_acmd("game_specialairnstart", falco_specialairnstart, Default) // Game acmd script
        .game_acmd("game_attackairlw", falco_attackairlw, Default)
        .game_acmd("game_specialairnloop", falco_specialairnloop, Default)
        .on_line(Main, fighter_frame) // Char opff
        //.status(Main, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, example_status_script) // Status script
        .install();
}

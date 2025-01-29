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
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::libc::*;

// if future me is looking for how to check if a hitbox hits: https://www.youtube.com/watch?v=bproyKYQUPA

static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20;
const FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SEARCH_HIT : i32 = 0x200000eb; // this value must be different for every character (prolly putting somewhere in stack)
const FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_WOLF_FLASH_HIT: i32 = 0x200000f3;
const FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_WOLF_FLASH_HITSTUN_DONE: i32 = 0x200000fb;
const FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_CAN_JUMP_OUT_OF_SHINE : i32 = 0x20000103;

// Char opff, Global opff
unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status = StatusModule::status_kind(module_accessor);
        let fighter = utility::get_kind(module_accessor);
        let situation = StatusModule::situation_kind(module_accessor);
        let jump_count = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        let jump_count_max = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);

        
        if fighter == FIGHTER_KIND_WOLF {
            // jump out of shine code
            if WorkModule::is_flag(module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_CAN_JUMP_OUT_OF_SHINE){
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

            // fast fall neutral b code
            if status == *FIGHTER_STATUS_KIND_SPECIAL_N{
                if situation == *SITUATION_KIND_AIR {
                    KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                    // check stick position and if then character is falling
                    if (ControlModule::get_stick_y(module_accessor) < -0.66) && (KineticModule::get_sum_speed_y(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0){
                        // turn on fastfall
                        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                    }
                }
            }
            
            // wolf flash hitting putting into fall
            if WorkModule::is_flag(module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_WOLF_FLASH_HIT) && WorkModule::is_flag(module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_WOLF_FLASH_HITSTUN_DONE){
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                WorkModule::off_flag(module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_WOLF_FLASH_HIT);
                WorkModule::off_flag(module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_WOLF_FLASH_HITSTUN_DONE);
            }

        }

        // wolf flash hit handling stuff
        if MotionModule::motion_kind(module_accessor) != hash40("special_air_s_end") {
            WorkModule::off_flag(module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
            WorkModule::off_flag(module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_WOLF_FLASH_HIT);
            WorkModule::off_flag(module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_WOLF_FLASH_HITSTUN_DONE);
        }

        if status == *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END{
            println!("Finished shine");
            WorkModule::off_flag(module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_CAN_JUMP_OUT_OF_SHINE);
        }
}


unsafe extern "C" fn wolf_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent,5.0);
    if macros::is_excute(agent){
      WorkModule::on_flag(agent.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent,16.0);
    if macros::is_excute(agent){
        macros::ATTACK(agent,0, 0, Hash40::new("top"), 10.4, 270, 90, 0, 6, 5.0, 0.0, 6.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent,1, 0, Hash40::new("top"), 12.0, 270, 90, 0, 6, 7.0, 0.0, 2.0, 0.5, None, None, None, 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    
    frame(agent.lua_state_agent,17.0);
    if macros::is_excute(agent){
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent,36.0);
    if macros::is_excute(agent){
        WorkModule::off_flag(agent.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn wolf_specialairsend(agent: &mut L2CAgentBase) {
    let lsa = agent.lua_state_agent;
    let module_accessor = agent.module_accessor;
    frame (lsa, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
    }

    frame (lsa, 2.0);
    if macros::is_excute(agent){
        macros::ATTACK(agent,0, 0, Hash40::new("top"), 20.0, 270, 100, 0, 20, 4.0, 0.0, 5.5, 5.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent,1, 0, Hash40::new("top"), 15.0, 28, 85, 0, 30, 7.0, 0.0, 5.5, 5.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
    }

    frame (lsa, 4.0);
    if macros::is_excute(agent){
        AttackModule::clear_all(module_accessor);
        WorkModule::off_flag(module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
    }

    // vanilla timing to re-enable ledge grab
    // frame (lsa, 7.0);
    // if macros::is_excute(agent){
    //     notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    // }

    frame (lsa, 10.0);
    if macros::is_excute(agent){
        JostleModule::set_status(module_accessor, true);
    }

    frame (lsa, 18.0);
    if macros::is_excute(agent){
        WorkModule::on_flag(module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_WOLF_FLASH_HITSTUN_DONE);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }

}

unsafe extern "C" fn wolf_speciallwstart(agent: &mut L2CAgentBase){
    macros::FT_MOTION_RATE_RANGE(agent, 0.0, 7.0, 1.0);
    // frame(agent.lua_state_agent, 1.0);
    // macros::FT_MOTION_RATE(agent, 0.14);

    // frame(agent.lua_state_agent, 4.0);
    // macros::FT_MOTION_RATE(agent, 1.0);

    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent){
        macros::ATTACK(agent,0, 0, Hash40::new("top"), 4.0, 65, 85, 0, 60, 9.0, 0.0, 7.0, 0.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.1);
        //macros::FT_MOTION_RATE(agent, 1.0);
    }

    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent){
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_CAN_JUMP_OUT_OF_SHINE);
        println!("Shine start");
    }
}

unsafe extern "C" fn wolf_specialairlwstart(agent: &mut L2CAgentBase){
    f macros::FT_MOTION_RATE_RANGE(agent, 0.0, 7.0, 1.0);
    // frame(agent.lua_state_agent, 1.0);
    // macros::FT_MOTION_RATE(agent, 0.14);

    // frame(agent.lua_state_agent, 4.0);
    // macros::FT_MOTION_RATE(agent, 1.0);

    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent){
        macros::ATTACK(agent,0, 0, Hash40::new("top"), 4.0, 65, 85, 0, 60, 9.0, 0.0, 7.0, 0.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.1);
        //macros::FT_MOTION_RATE(agent, 1.0);
    }

    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent){
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_CAN_JUMP_OUT_OF_SHINE);
        println!("Shine start");
    }
}

// unsafe extern "C" fn wolf_speciallwend(agent: &mut L2CAgentBase){
//     frame(agent.lua_state_agent, 1.0);
//     macros::FT_MOTION_RATE(agent, 0.8);
//     WorkModule::off_flag(agent.module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_CAN_JUMP_OUT_OF_SHINE);
// }

// unsafe extern "C" fn wolf_specialairlwend(agent: &mut L2CAgentBase){
//     frame(agent.lua_state_agent, 1.0);
//     macros::FT_MOTION_RATE(agent, 0.8);
//     WorkModule::off_flag(agent.module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_CAN_JUMP_OUT_OF_SHINE);
// }

#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(fighter_manager: *mut smash::app::FighterManager, attacker_id: u32, defender_id: u32, move_type: f32, arg5: i32, move_type_again: bool, fighter: &mut L2CAgentBase) -> u64 {
    let attacker_boma = sv_battle_object::module_accessor(attacker_id);
    let defender_boma = sv_battle_object::module_accessor(defender_id);
    let attacker_kind = sv_battle_object::kind(attacker_id);
    let defender_kind = sv_battle_object::kind(defender_id);
    // If search_hit flag is on
    if WorkModule::is_flag(attacker_boma, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SEARCH_HIT) {
        // Disable flag
        WorkModule::off_flag(attacker_boma, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
        WorkModule::on_flag(attacker_boma, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_WOLF_FLASH_HIT);
        //WorkModule::off_flag(attacker_boma, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_WOLF_FLASH_HIT_AND_HITSTUN_DONE);
        // If thing being hit is a fighter
        // if utility::get_category(&mut *defender_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        //     // Give fighter being hit sticky bomb
        //     ItemModule::have_item(defender_boma, smash::app::ItemKind(*ITEM_KIND_CHEWING), 0, 0, false, false);
        // }
    }
    
    original!()(fighter_manager, attacker_id, defender_id, move_type, arg5, move_type_again, fighter)
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

static OFFSET_SEARCH_CODE: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1, //.text:0000007100675A20                 SUB             SP, SP, #0xC0
    0xe8, 0x2b, 0x00, 0xfd, //.text:0000007100675A24                 STR             D8, [SP,#0xB0+var_60]
    0xfc, 0x6f, 0x06, 0xa9, //.text:0000007100675A28                 STP             X28, X27, [SP,#0xB0+var_50]
    0xfa, 0x67, 0x07, 0xa9, //.text:0000007100675A2C                 STP             X26, X25, [SP,#0xB0+var_40]
    0xf8, 0x5f, 0x08, 0xa9, //.text:0000007100675A30                 STP             X24, X23, [SP,#0xB0+var_30]
    0xf6, 0x57, 0x09, 0xa9, //.text:0000007100675A34                 STP             X22, X21, [SP,#0xB0+var_20]
    0xf4, 0x4f, 0x0a, 0xa9, //.text:0000007100675A38                 STP             X20, X19, [SP,#0xB0+var_10]
    0xfd, 0x7b, 0x0b, 0xa9, //.text:0000007100675A3C                 STP             X29, X30, [SP,#0xB0+var_s0]
    0xfd, 0xc3, 0x02, 0x91, //.text:0000007100675A40                 ADD             X29, SP, #0xB0
    0xfb, 0x03, 0x00, 0xaa  //.text:0000007100675A44                 MOV             X27, X0
];



pub fn install() {
    unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, OFFSET_SEARCH_CODE) {
            NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
    }

    skyline::install_hooks!{
        notify_log_event_collision_hit_replace
    }


    Agent::new("wolf")
        .game_acmd("game_attackairlw", wolf_attackairlw, Default) // Game acmd script
        .game_acmd("game_specialairsend", wolf_specialairsend, Default)
        .game_acmd("game_speciallwstart", wolf_speciallwstart, Default)
        // .game_acmd("game_speciallwend", wolf_speciallwend, Default)
        // .game_acmd("game_specialairlwend", wolf_specialairlwend, Default)
        .game_acmd("game_specialairlwstart", wolf_specialairlwstart, Default)
        .on_line(Main, fighter_frame) // character opff
        .install();
}

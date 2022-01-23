use crate::*;

use super::impl_;

pub fn target_id(arg1: *mut lua_State) -> u32 {
    unsafe {
        impl_::target_id(arg1)
    }
}

pub fn is_valid_target(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::is_valid_target(arg1)
    }
}

pub fn fighter_kind(arg1: *mut lua_State) -> app::FighterKind {
    unsafe {
        impl_::fighter_kind(arg1)
    }
}

pub fn copy_fighter_kind(arg1: *mut lua_State) -> app::FighterKind {
    unsafe {
        impl_::copy_fighter_kind(arg1)
    }
}

pub fn rank(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::rank(arg1)
    }
}

pub fn cp_type(arg1: *mut lua_State) -> app::FighterAICPType {
    unsafe {
        impl_::cp_type(arg1)
    }
}

pub fn cp_flag(arg1: *mut lua_State) -> app::FighterAICPFlag {
    unsafe {
        impl_::cp_flag(arg1)
    }
}

pub fn cp_slide_type(arg1: *mut lua_State) -> app::FighterAICPType {
    unsafe {
        impl_::cp_slide_type(arg1)
    }
}

pub fn act_id(arg1: *mut lua_State) -> app::FighterAIAct::CmdId {
    unsafe {
        impl_::act_id(arg1)
    }
}

pub fn current_attack_cancel_frame(arg1: *mut lua_State) -> i32 {
    unsafe {
        impl_::current_attack_cancel_frame(arg1)
    }
}

pub fn uniq_stat(arg1: *mut lua_State) -> app::FighterAIUniqStat {
    unsafe {
        impl_::uniq_stat(arg1)
    }
}

pub fn attack_phase(arg1: *mut lua_State) -> app::FighterAIAttackPhase {
    unsafe {
        impl_::attack_phase(arg1)
    }
}

pub fn check_stat_air(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_air(arg1)
    }
}

pub fn check_stat_build_max(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_build_max(arg1)
    }
}

pub fn check_stat_build_up(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_build_up(arg1)
    }
}

pub fn check_stat_gorogoro(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_gorogoro(arg1)
    }
}

pub fn check_stat_attention(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_attention(arg1)
    }
}

pub fn check_stat_final(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_final(arg1)
    }
}

pub fn check_stat_final_act(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_final_act(arg1)
    }
}

pub fn check_stat_invincible(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_invincible(arg1)
    }
}

pub fn check_stat_invincible_l(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_invincible_l(arg1)
    }
}

pub fn check_stat_damage_elec(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_damage_elec(arg1)
    }
}

pub fn check_stat_sp_dir(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_sp_dir(arg1)
    }
}

pub fn check_stat_unguarded_hind(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_unguarded_hind(arg1)
    }
}

pub fn check_stat_unguarded(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_unguarded(arg1)
    }
}

pub fn check_stat_dash(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_dash(arg1)
    }
}

pub fn check_stat_down(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_down(arg1)
    }
}

pub fn check_stat_piyo(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_piyo(arg1)
    }
}

pub fn check_stat_dragoon(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_dragoon(arg1)
    }
}

pub fn check_stat_genesis(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_genesis(arg1)
    }
}

pub fn check_stat_catch(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_catch(arg1)
    }
}

pub fn check_stat_damage(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_damage(arg1)
    }
}

pub fn check_stat_guard(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_guard(arg1)
    }
}

pub fn check_stat_attack_hold(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_attack_hold(arg1)
    }
}

pub fn check_stat_floor_pass(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_floor_pass(arg1)
    }
}

pub fn check_stat_floor_damage(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_floor_damage(arg1)
    }
}

pub fn check_stat_ground_free(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_ground_free(arg1)
    }
}

pub fn check_stat_ground_free2(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_ground_free2(arg1)
    }
}

pub fn check_stat_air_free(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_air_free(arg1)
    }
}

pub fn check_stat_touch_u(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_touch_u(arg1)
    }
}

pub fn check_stat_touch_l(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_touch_l(arg1)
    }
}

pub fn check_stat_touch_r(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_touch_r(arg1)
    }
}

pub fn check_stat_cannot_catch_cliff(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_cannot_catch_cliff(arg1)
    }
}

pub fn check_stat_dive(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_dive(arg1)
    }
}

pub fn check_stat_unable_cliff_xlu(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_unable_cliff_xlu(arg1)
    }
}

pub fn check_stat_unable_escape_air(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_unable_escape_air(arg1)
    }
}

pub fn check_stat_unable_attack(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_unable_attack(arg1)
    }
}

pub fn check_stat_unable_special(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_unable_special(arg1)
    }
}

pub fn check_stat_unable_jump(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_unable_jump(arg1)
    }
}

pub fn check_stat_unable_shield(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_unable_shield(arg1)
    }
}

pub fn check_stat_have(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_have(arg1)
    }
}

pub fn check_stat_put_bomb(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_put_bomb(arg1)
    }
}

pub fn check_stat_can_use_superleaf(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_can_use_superleaf(arg1)
    }
}

pub fn check_stat_can_use_rocketbelt(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_can_use_rocketbelt(arg1)
    }
}

pub fn check_stat_have_throw(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_have_throw(arg1)
    }
}

pub fn check_stat_have_shoot(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_have_shoot(arg1)
    }
}

pub fn check_stat_have_swing(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_have_swing(arg1)
    }
}

pub fn check_stat_dogs_blind_own(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_dogs_blind_own(arg1)
    }
}

pub fn check_stat_target_invisible(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_target_invisible(arg1)
    }
}

pub fn check_skill_stat(arg1: *mut lua_State, arg2: app::FighterAIStat::SkillStatus) -> bool {
    unsafe {
        impl_::check_skill_stat(arg1, arg2)
    }
}

pub fn check_spirits_event_stat(arg1: *mut lua_State, arg2: app::FighterAIStat::SpiritsEventStatus) -> bool {
    unsafe {
        impl_::check_spirits_event_stat(arg1, arg2)
    }
}

pub fn check_chr_stat(arg1: *mut lua_State, arg2: app::FighterAIStat::CharacterStatus, arg3: app::FighterKind) -> bool {
    unsafe {
        impl_::check_chr_stat(arg1, arg2, arg3)
    }
}

pub fn air_lasso_count(arg1: *mut lua_State) -> i32 {
    unsafe {
        impl_::air_lasso_count(arg1)
    }
}

pub fn check_cliffable(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_cliffable(arg1)
    }
}

pub fn check_cliffable_floor_lr(arg1: *mut lua_State, arg2: f32) -> bool {
    unsafe {
        impl_::check_cliffable_floor_lr(arg1, arg2)
    }
}

pub fn check_passable(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_passable(arg1)
    }
}

pub fn shield_rate(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::shield_rate(arg1)
    }
}

pub fn damage_reaction_mul(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::damage_reaction_mul(arg1)
    }
}

pub fn stop_frame(arg1: *mut lua_State) -> i32 {
    unsafe {
        impl_::stop_frame(arg1)
    }
}

pub fn height(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::height(arg1)
    }
}

pub fn pos_x(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::pos_x(arg1)
    }
}

pub fn pos_y(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::pos_y(arg1)
    }
}

pub fn speed_x(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::speed_x(arg1)
    }
}

pub fn speed_y(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::speed_y(arg1)
    }
}

pub fn scale(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::scale(arg1)
    }
}

pub fn status_kind(arg1: *mut lua_State) -> i32 {
    unsafe {
        impl_::status_kind(arg1)
    }
}

pub fn prev_status_kind(arg1: *mut lua_State) -> i32 {
    unsafe {
        impl_::prev_status_kind(arg1)
    }
}

pub fn motion_kind(arg1: *mut lua_State) -> phx::Hash40 {
    unsafe {
        impl_::motion_kind(arg1)
    }
}

pub fn motion_frame(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::motion_frame(arg1)
    }
}

pub fn motion_rate(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::motion_rate(arg1)
    }
}

pub fn motion_cancel_frame(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::motion_cancel_frame(arg1)
    }
}

pub fn jump_rest_available(arg1: *mut lua_State) -> u16 {
    unsafe {
        impl_::jump_rest_available(arg1)
    }
}

pub fn is_sp_u_available(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::is_sp_u_available(arg1)
    }
}

pub fn is_sp_u_weaken_available(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::is_sp_u_weaken_available(arg1)
    }
}

pub fn damage(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::damage(arg1)
    }
}

pub fn hp(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::hp(arg1)
    }
}

pub fn lr(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::lr(arg1)
    }
}

pub fn customize_n(arg1: *mut lua_State) -> i32 {
    unsafe {
        impl_::customize_n(arg1)
    }
}

pub fn customize_s(arg1: *mut lua_State) -> i32 {
    unsafe {
        impl_::customize_s(arg1)
    }
}

pub fn customize_hi(arg1: *mut lua_State) -> i32 {
    unsafe {
        impl_::customize_hi(arg1)
    }
}

pub fn customize_lw(arg1: *mut lua_State) -> i32 {
    unsafe {
        impl_::customize_lw(arg1)
    }
}

pub fn check_use_command(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_use_command(arg1)
    }
}

pub fn check_use_command_type(arg1: *mut lua_State) -> u8 {
    unsafe {
        impl_::check_use_command_type(arg1)
    }
}

pub fn check_command_236_step(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_command_236_step(arg1)
    }
}

pub fn check_command_41236_step(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_command_41236_step(arg1)
    }
}

pub fn check_command_214_step(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_command_214_step(arg1)
    }
}

pub fn check_command_623_step(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_command_623_step(arg1)
    }
}

pub fn check_command_236236_step(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_command_236236_step(arg1)
    }
}

pub fn check_command_21416_step(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_command_21416_step(arg1)
    }
}

pub fn check_command_214214_step(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_command_214214_step(arg1)
    }
}

pub fn check_command_23634_step(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_command_23634_step(arg1)
    }
}

pub fn fighter_uniq_value(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::fighter_uniq_value(arg1)
    }
}

pub fn fighter_uniq_value2(arg1: *mut lua_State, arg2: i32) -> f32 {
    unsafe {
        impl_::fighter_uniq_value2(arg1, arg2)
    }
}

pub fn line_segment_check(arg1: *mut lua_State, arg2: &phx::Vector2f) -> bool {
    unsafe {
        impl_::line_segment_check(arg1, arg2)
    }
}

pub fn line_segment_check_from_top_n(arg1: *mut lua_State, arg2: &phx::Vector2f, arg3: &phx::Vector2f) -> bool {
    unsafe {
        impl_::line_segment_check_from_top_n(arg1, arg2, arg3)
    }
}

pub fn line_segment_check_only_roof(arg1: *mut lua_State, arg2: &phx::Vector2f) -> bool {
    unsafe {
        impl_::line_segment_check_only_roof(arg1, arg2)
    }
}

pub fn line_segment_check_only_floor(arg1: *mut lua_State, arg2: &phx::Vector2f) -> bool {
    unsafe {
        impl_::line_segment_check_only_floor(arg1, arg2)
    }
}

pub fn line_segment_check_only_wall(arg1: *mut lua_State, arg2: &phx::Vector2f) -> bool {
    unsafe {
        impl_::line_segment_check_only_wall(arg1, arg2)
    }
}

pub fn weapon_pos_x(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::weapon_pos_x(arg1)
    }
}

pub fn weapon_pos_y(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::weapon_pos_y(arg1)
    }
}

pub fn weapon_speed_x(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::weapon_speed_x(arg1)
    }
}

pub fn weapon_speed_y(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::weapon_speed_y(arg1)
    }
}

pub fn target_fighter_kind(arg1: *mut lua_State) -> app::FighterKind {
    unsafe {
        impl_::target_fighter_kind(arg1)
    }
}

pub fn target_copy_fighter_kind(arg1: *mut lua_State) -> app::FighterKind {
    unsafe {
        impl_::target_copy_fighter_kind(arg1)
    }
}

pub fn target_current_attack_start_frame(arg1: *mut lua_State) -> i32 {
    unsafe {
        impl_::target_current_attack_start_frame(arg1)
    }
}

pub fn target_current_attack_combo_end_frame(arg1: *mut lua_State) -> i32 {
    unsafe {
        impl_::target_current_attack_combo_end_frame(arg1)
    }
}

pub fn target_current_attack_cancel_frame(arg1: *mut lua_State) -> i32 {
    unsafe {
        impl_::target_current_attack_cancel_frame(arg1)
    }
}

pub fn check_target_stat_air(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_air(arg1)
    }
}

pub fn check_target_stat_build_up(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_build_up(arg1)
    }
}

pub fn check_target_stat_attention(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_attention(arg1)
    }
}

pub fn check_target_stat_final(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_final(arg1)
    }
}

pub fn check_target_stat_dead(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_dead(arg1)
    }
}

pub fn check_target_stat_invincible(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_invincible(arg1)
    }
}

pub fn check_target_stat_invincible_l(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_invincible_l(arg1)
    }
}

pub fn check_target_stat_attack_catch(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_attack_catch(arg1)
    }
}

pub fn check_target_stat_reflect(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_reflect(arg1)
    }
}

pub fn check_target_stat_unguarded_hind(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_unguarded_hind(arg1)
    }
}

pub fn check_target_stat_unguarded(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_unguarded(arg1)
    }
}

pub fn check_target_stat_combo(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_combo(arg1)
    }
}

pub fn check_target_stat_no_counter(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_no_counter(arg1)
    }
}

pub fn check_target_stat_down(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_down(arg1)
    }
}

pub fn check_target_stat_fall_sp(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_fall_sp(arg1)
    }
}

pub fn check_target_stat_piyo(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_piyo(arg1)
    }
}

pub fn check_target_stat_piyo_l(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_piyo_l(arg1)
    }
}

pub fn check_target_stat_dragoon(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_dragoon(arg1)
    }
}

pub fn check_target_stat_cliff(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_cliff(arg1)
    }
}

pub fn check_target_stat_cliff_act(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_cliff_act(arg1)
    }
}

pub fn check_target_stat_catch(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_catch(arg1)
    }
}

pub fn check_target_stat_damage(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_damage(arg1)
    }
}

pub fn check_target_stat_guard(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_guard(arg1)
    }
}

pub fn check_target_stat_escape(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_escape(arg1)
    }
}

pub fn check_target_stat_rebirth(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_rebirth(arg1)
    }
}

pub fn check_target_stat_attack(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_attack(arg1)
    }
}

pub fn check_target_stat_attack_hold(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_attack_hold(arg1)
    }
}

pub fn check_target_stat_squat(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_squat(arg1)
    }
}

pub fn check_target_stat_unable_attack(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_unable_attack(arg1)
    }
}

pub fn check_target_stat_unable_special(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_unable_special(arg1)
    }
}

pub fn check_target_stat_specialflag_hoist(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_specialflag_hoist(arg1)
    }
}

pub fn check_target_chr_stat(arg1: *mut lua_State, arg2: app::FighterAIStat::CharacterStatus, arg3: app::FighterKind) -> bool {
    unsafe {
        impl_::check_target_chr_stat(arg1, arg2, arg3)
    }
}

pub fn target_width(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_width(arg1)
    }
}

pub fn target_height(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_height(arg1)
    }
}

pub fn target_pos_x(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_pos_x(arg1)
    }
}

pub fn target_pos_y(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_pos_y(arg1)
    }
}

pub fn target_speed_x(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_speed_x(arg1)
    }
}

pub fn target_speed_y(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_speed_y(arg1)
    }
}

pub fn target_scale(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_scale(arg1)
    }
}

pub fn target_jump_rest_available(arg1: *mut lua_State) -> u16 {
    unsafe {
        impl_::target_jump_rest_available(arg1)
    }
}

pub fn target_damage(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_damage(arg1)
    }
}

pub fn target_lr(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_lr(arg1)
    }
}

pub fn target_status_kind(arg1: *mut lua_State) -> i32 {
    unsafe {
        impl_::target_status_kind(arg1)
    }
}

pub fn target_motion_kind(arg1: *mut lua_State) -> phx::Hash40 {
    unsafe {
        impl_::target_motion_kind(arg1)
    }
}

pub fn target_motion_frame(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_motion_frame(arg1)
    }
}

pub fn target_hit_collision_rect(arg1: *mut lua_State) -> lib::Rect {
    unsafe {
        impl_::target_hit_collision_rect(arg1).into()
    }
}

pub fn lr_to_target(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::lr_to_target(arg1)
    }
}

pub fn is_looking_at_target(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::is_looking_at_target(arg1)
    }
}

pub fn distance_to_target(arg1: *mut lua_State) -> phx::Vec2 {
    unsafe {
        impl_::distance_to_target(arg1).into()
    }
}

pub fn distance_x_to_target(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::distance_x_to_target(arg1)
    }
}

pub fn distance_y_to_target(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::distance_y_to_target(arg1)
    }
}

pub fn is_target_on_same_floor(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::is_target_on_same_floor(arg1)
    }
}

pub fn check_any_danger_target(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_any_danger_target(arg1)
    }
}

pub fn check_parent_over_ground(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_parent_over_ground(arg1)
    }
}

pub fn check_parent_same_floor(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_parent_same_floor(arg1)
    }
}

pub fn parent_pos_y(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::parent_pos_y(arg1)
    }
}

pub fn parent_speed_y(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::parent_speed_y(arg1)
    }
}

pub fn floor_edge_distance_lr(arg1: *mut lua_State, arg2: f32) -> f32 {
    unsafe {
        impl_::floor_edge_distance_lr(arg1, arg2)
    }
}

pub fn floor_edge_distance_f(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::floor_edge_distance_f(arg1)
    }
}

pub fn floor_edge_distance_b(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::floor_edge_distance_b(arg1)
    }
}

pub fn floor_edge_distance_floor_lr(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::floor_edge_distance_floor_lr(arg1)
    }
}

pub fn floor_edge_distance_floor_lr_moved(arg1: *mut lua_State, arg2: f32) -> f32 {
    unsafe {
        impl_::floor_edge_distance_floor_lr_moved(arg1, arg2)
    }
}

pub fn floor_width(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::floor_width(arg1)
    }
}

pub fn floor_center(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::floor_center(arg1)
    }
}

pub fn floor_lr(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::floor_lr(arg1)
    }
}

pub fn target_floor_edge_distance_lr(arg1: *mut lua_State, arg2: f32) -> f32 {
    unsafe {
        impl_::target_floor_edge_distance_lr(arg1, arg2)
    }
}

pub fn check_over_ground(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_over_ground(arg1)
    }
}

pub fn check_over_ground_distance_current_lr(arg1: *mut lua_State, arg2: f32) -> bool {
    unsafe {
        impl_::check_over_ground_distance_current_lr(arg1, arg2)
    }
}

pub fn check_target_over_ground(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_over_ground(arg1)
    }
}

pub fn check_over_goal(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_over_goal(arg1)
    }
}

pub fn floor_pos(arg1: *mut lua_State, arg2: bool) -> phx::Vec2 {
    unsafe {
        impl_::floor_pos(arg1, arg2).into()
    }
}

pub fn floor_pos_moved(arg1: *mut lua_State, arg2: f32, arg3: bool) -> phx::Vec2 {
    unsafe {
        impl_::floor_pos_moved(arg1, arg2, arg3).into()
    }
}

pub fn floor_pos_floor_lr(arg1: *mut lua_State, arg2: f32, arg3: bool) -> phx::Vec2 {
    unsafe {
        impl_::floor_pos_floor_lr(arg1, arg2, arg3).into()
    }
}

pub fn floor_moves(arg1: *mut lua_State) -> phx::Vec2 {
    unsafe {
        impl_::floor_moves(arg1).into()
    }
}

pub fn return_pos(arg1: *mut lua_State, arg2: bool) -> phx::Vec2 {
    unsafe {
        impl_::return_pos(arg1, arg2).into()
    }
}

pub fn safe_return_pos(arg1: *mut lua_State, arg2: bool) -> phx::Vec2 {
    unsafe {
        impl_::safe_return_pos(arg1, arg2).into()
    }
}

pub fn goal_pos(arg1: *mut lua_State) -> phx::Vec2 {
    unsafe {
        impl_::goal_pos(arg1).into()
    }
}

pub fn check_away_floor(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::check_away_floor(arg1)
    }
}

pub fn is_offensive_on_strategy(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::is_offensive_on_strategy(arg1)
    }
}

pub fn is_defensive_on_strategy(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::is_defensive_on_strategy(arg1)
    }
}

pub fn press_frame(arg1: *mut lua_State) -> u8 {
    unsafe {
        impl_::press_frame(arg1)
    }
}

pub fn push_wait(arg1: *mut lua_State) -> u8 {
    unsafe {
        impl_::push_wait(arg1)
    }
}

pub fn change_action(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) {
    unsafe {
        impl_::change_action(arg1, arg2)
    }
}

pub fn set_auto_stop(arg1: *mut lua_State, arg2: i32) {
    unsafe {
        impl_::set_auto_stop(arg1, arg2)
    }
}

pub fn update_count(arg1: *mut lua_State) -> i32 {
    unsafe {
        impl_::update_count(arg1)
    }
}

pub fn is_update_count_odd(arg1: *mut lua_State) -> bool {
    unsafe {
        impl_::is_update_count_odd(arg1)
    }
}

pub fn reset_return_count(arg1: *mut lua_State) {
    unsafe {
        impl_::reset_return_count(arg1)
    }
}

pub fn set_no_return_frame(arg1: *mut lua_State, arg2: i32) {
    unsafe {
        impl_::set_no_return_frame(arg1, arg2)
    }
}

pub fn get_cmd_id_from_req_id(arg1: *mut lua_State, arg2: app::FighterAIAct::ReqId) -> app::FighterAIAct::CmdId {
    unsafe {
        impl_::get_cmd_id_from_req_id(arg1, arg2)
    }
}

pub fn get_cmd_id_from_req_id_with_predict(arg1: *mut lua_State, arg2: app::FighterAIAct::ReqId, arg3: f32, arg4: f32, arg5: f32) -> app::FighterAIAct::CmdId {
    unsafe {
        impl_::get_cmd_id_from_req_id_with_predict(arg1, arg2, arg3, arg4, arg5)
    }
}

pub fn get_cmd_probability_from_req_id(arg1: *mut lua_State, arg2: app::FighterAIAct::ReqId, arg3: app::FighterAIAct::CmdId) -> f32 {
    unsafe {
        impl_::get_cmd_probability_from_req_id(arg1, arg2, arg3)
    }
}

pub fn enable_command(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) {
    unsafe {
        impl_::enable_command(arg1, arg2)
    }
}

pub fn disable_command(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) {
    unsafe {
        impl_::disable_command(arg1, arg2)
    }
}

pub fn disable_command_ground_all(arg1: *mut lua_State) {
    unsafe {
        impl_::disable_command_ground_all(arg1)
    }
}

pub fn disable_command_air_all(arg1: *mut lua_State) {
    unsafe {
        impl_::disable_command_air_all(arg1)
    }
}

pub fn disable_command_attack_button_all(arg1: *mut lua_State) {
    unsafe {
        impl_::disable_command_attack_button_all(arg1)
    }
}

pub fn disable_command_special_button_all(arg1: *mut lua_State) {
    unsafe {
        impl_::disable_command_special_button_all(arg1)
    }
}

pub fn reset_cmd_id_probability_add_2nd(arg1: *mut lua_State) {
    unsafe {
        impl_::reset_cmd_id_probability_add_2nd(arg1)
    }
}

pub fn set_cmd_id_probability_add_2nd(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId, arg3: f32) {
    unsafe {
        impl_::set_cmd_id_probability_add_2nd(arg1, arg2, arg3)
    }
}

pub fn reset_cmd_id_probability_mul_2nd(arg1: *mut lua_State) {
    unsafe {
        impl_::reset_cmd_id_probability_mul_2nd(arg1)
    }
}

pub fn set_cmd_id_probability_mul_2nd(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId, arg3: f32) {
    unsafe {
        impl_::set_cmd_id_probability_mul_2nd(arg1, arg2, arg3)
    }
}

pub fn set_cmd_id_probability_mul_for_specializer(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId, arg3: f32) {
    unsafe {
        impl_::set_cmd_id_probability_mul_for_specializer(arg1, arg2, arg3)
    }
}

pub fn get_cmd_id_probability_mul(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> f32 {
    unsafe {
        impl_::get_cmd_id_probability_mul(arg1, arg2)
    }
}

pub fn predict_landing_frame(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::predict_landing_frame(arg1)
    }
}

pub fn predict_target_landing_frame(arg1: *mut lua_State) -> f32 {
    unsafe {
        impl_::predict_target_landing_frame(arg1)
    }
}

pub fn predict_hit_in_target_attack(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId, arg3: f32, arg4: f32) -> bool {
    unsafe {
        impl_::predict_hit_in_target_attack(arg1, arg2, arg3, arg4)
    }
}

pub fn predict_hit_in_target_attack_from_motion(arg1: *mut lua_State, arg2: phx::Hash40, arg3: f32, arg4: f32) -> bool {
    unsafe {
        impl_::predict_hit_in_target_attack_from_motion(arg1, arg2, arg3, arg4)
    }
}

pub fn predict_target_hit_in_attack(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId, arg3: f32, arg4: f32, arg5: f32, arg6: f32) -> bool {
    unsafe {
        impl_::predict_target_hit_in_attack(arg1, arg2, arg3, arg4, arg5, arg6)
    }
}

pub fn check_line_segment_vs_target_attack(arg1: *mut lua_State, arg2: phx::Hash40, arg3: &phx::Vector2f, arg4: &phx::Vector2f, arg5: &mut phx::Vector4f) -> bool {
    unsafe {
        impl_::check_line_segment_vs_target_attack(arg1, arg2, arg3, arg4, arg5)
    }
}

pub fn attack_start_frame(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> i32 {
    unsafe {
        impl_::attack_start_frame(arg1, arg2)
    }
}

pub fn target_attack_start_frame(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> i32 {
    unsafe {
        impl_::target_attack_start_frame(arg1, arg2)
    }
}

pub fn attack_end_frame(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> i32 {
    unsafe {
        impl_::attack_end_frame(arg1, arg2)
    }
}

pub fn attack_cancel_frame(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> i32 {
    unsafe {
        impl_::attack_cancel_frame(arg1, arg2)
    }
}

pub fn attack_data_x0(arg1: *mut lua_State, arg2: phx::Hash40) -> f32 {
    unsafe {
        impl_::attack_data_x0(arg1, arg2)
    }
}

pub fn attack_data_x1(arg1: *mut lua_State, arg2: phx::Hash40) -> f32 {
    unsafe {
        impl_::attack_data_x1(arg1, arg2)
    }
}

pub fn attack_data_y0(arg1: *mut lua_State, arg2: phx::Hash40) -> f32 {
    unsafe {
        impl_::attack_data_y0(arg1, arg2)
    }
}

pub fn attack_data_y1(arg1: *mut lua_State, arg2: phx::Hash40) -> f32 {
    unsafe {
        impl_::attack_data_y1(arg1, arg2)
    }
}

pub fn attack_info_needs_turn(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> bool {
    unsafe {
        impl_::attack_info_needs_turn(arg1, arg2)
    }
}

pub fn attack_info_reaction(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> f32 {
    unsafe {
        impl_::attack_info_reaction(arg1, arg2)
    }
}

pub fn attack_info_no_shield(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> bool {
    unsafe {
        impl_::attack_info_no_shield(arg1, arg2)
    }
}

pub fn attack_info_meteor(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> bool {
    unsafe {
        impl_::attack_info_meteor(arg1, arg2)
    }
}

pub fn attack_info_reflectable(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> bool {
    unsafe {
        impl_::attack_info_reflectable(arg1, arg2)
    }
}

pub fn attack_is_as_weapon(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> bool {
    unsafe {
        impl_::attack_is_as_weapon(arg1, arg2)
    }
}

pub fn attack_info_distance(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> f32 {
    unsafe {
        impl_::attack_info_distance(arg1, arg2)
    }
}

pub fn motion_to_cmd_id(arg1: *mut lua_State, arg2: phx::Hash40) -> app::FighterAIAct::CmdId {
    unsafe {
        impl_::motion_to_cmd_id(arg1, arg2)
    }
}

pub fn target_attack_info_needs_turn(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> bool {
    unsafe {
        impl_::target_attack_info_needs_turn(arg1, arg2)
    }
}

pub fn target_attack_info_no_shield(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> bool {
    unsafe {
        impl_::target_attack_info_no_shield(arg1, arg2)
    }
}

pub fn target_motion_to_cmd_id(arg1: *mut lua_State, arg2: phx::Hash40) -> app::FighterAIAct::CmdId {
    unsafe {
        impl_::target_motion_to_cmd_id(arg1, arg2)
    }
}

pub fn current_attack_combo_next_motion(arg1: *mut lua_State) -> phx::Hash40 {
    unsafe {
        impl_::current_attack_combo_next_motion(arg1)
    }
}

pub fn target_current_attack_combo_next_motion(arg1: *mut lua_State) -> phx::Hash40 {
    unsafe {
        impl_::target_current_attack_combo_next_motion(arg1)
    }
}

pub fn target_attack_start_frame_from_motion(arg1: *mut lua_State, arg2: phx::Hash40) -> i32 {
    unsafe {
        impl_::target_attack_start_frame_from_motion(arg1, arg2)
    }
}
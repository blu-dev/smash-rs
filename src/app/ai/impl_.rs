use crate::*;

extern "C" {
    #[link_name = "_ZN3app2ai9target_idEP9lua_State"]
    pub(super) fn target_id(arg1: *mut lua_State) -> u32;

    #[link_name = "_ZN3app2ai15is_valid_targetEP9lua_State"]
    pub(super) fn is_valid_target(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai12fighter_kindEP9lua_State"]
    pub(super) fn fighter_kind(arg1: *mut lua_State) -> app::FighterKind;

    #[link_name = "_ZN3app2ai17copy_fighter_kindEP9lua_State"]
    pub(super) fn copy_fighter_kind(arg1: *mut lua_State) -> app::FighterKind;

    #[link_name = "_ZN3app2ai4rankEP9lua_State"]
    pub(super) fn rank(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai7cp_typeEP9lua_State"]
    pub(super) fn cp_type(arg1: *mut lua_State) -> app::FighterAICPType;

    #[link_name = "_ZN3app2ai7cp_flagEP9lua_State"]
    pub(super) fn cp_flag(arg1: *mut lua_State) -> app::FighterAICPFlag;

    #[link_name = "_ZN3app2ai13cp_slide_typeEP9lua_State"]
    pub(super) fn cp_slide_type(arg1: *mut lua_State) -> app::FighterAICPType;

    #[link_name = "_ZN3app2ai6act_idEP9lua_State"]
    pub(super) fn act_id(arg1: *mut lua_State) -> app::FighterAIAct::CmdId;

    #[link_name = "_ZN3app2ai27current_attack_cancel_frameEP9lua_State"]
    pub(super) fn current_attack_cancel_frame(arg1: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai9uniq_statEP9lua_State"]
    pub(super) fn uniq_stat(arg1: *mut lua_State) -> app::FighterAIUniqStat;

    #[link_name = "_ZN3app2ai12attack_phaseEP9lua_State"]
    pub(super) fn attack_phase(arg1: *mut lua_State) -> app::FighterAIAttackPhase;

    #[link_name = "_ZN3app2ai14check_stat_airEP9lua_State"]
    pub(super) fn check_stat_air(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai20check_stat_build_maxEP9lua_State"]
    pub(super) fn check_stat_build_max(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai19check_stat_build_upEP9lua_State"]
    pub(super) fn check_stat_build_up(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai19check_stat_gorogoroEP9lua_State"]
    pub(super) fn check_stat_gorogoro(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai20check_stat_attentionEP9lua_State"]
    pub(super) fn check_stat_attention(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai16check_stat_finalEP9lua_State"]
    pub(super) fn check_stat_final(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai20check_stat_final_actEP9lua_State"]
    pub(super) fn check_stat_final_act(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai21check_stat_invincibleEP9lua_State"]
    pub(super) fn check_stat_invincible(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_stat_invincible_lEP9lua_State"]
    pub(super) fn check_stat_invincible_l(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_stat_damage_elecEP9lua_State"]
    pub(super) fn check_stat_damage_elec(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai17check_stat_sp_dirEP9lua_State"]
    pub(super) fn check_stat_sp_dir(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai25check_stat_unguarded_hindEP9lua_State"]
    pub(super) fn check_stat_unguarded_hind(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai20check_stat_unguardedEP9lua_State"]
    pub(super) fn check_stat_unguarded(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai15check_stat_dashEP9lua_State"]
    pub(super) fn check_stat_dash(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai15check_stat_downEP9lua_State"]
    pub(super) fn check_stat_down(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai15check_stat_piyoEP9lua_State"]
    pub(super) fn check_stat_piyo(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai18check_stat_dragoonEP9lua_State"]
    pub(super) fn check_stat_dragoon(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai18check_stat_genesisEP9lua_State"]
    pub(super) fn check_stat_genesis(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai16check_stat_catchEP9lua_State"]
    pub(super) fn check_stat_catch(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai17check_stat_damageEP9lua_State"]
    pub(super) fn check_stat_damage(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai16check_stat_guardEP9lua_State"]
    pub(super) fn check_stat_guard(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_stat_attack_holdEP9lua_State"]
    pub(super) fn check_stat_attack_hold(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai21check_stat_floor_passEP9lua_State"]
    pub(super) fn check_stat_floor_pass(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_stat_floor_damageEP9lua_State"]
    pub(super) fn check_stat_floor_damage(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_stat_ground_freeEP9lua_State"]
    pub(super) fn check_stat_ground_free(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_stat_ground_free2EP9lua_State"]
    pub(super) fn check_stat_ground_free2(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai19check_stat_air_freeEP9lua_State"]
    pub(super) fn check_stat_air_free(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai18check_stat_touch_uEP9lua_State"]
    pub(super) fn check_stat_touch_u(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai18check_stat_touch_lEP9lua_State"]
    pub(super) fn check_stat_touch_l(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai18check_stat_touch_rEP9lua_State"]
    pub(super) fn check_stat_touch_r(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai29check_stat_cannot_catch_cliffEP9lua_State"]
    pub(super) fn check_stat_cannot_catch_cliff(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai15check_stat_diveEP9lua_State"]
    pub(super) fn check_stat_dive(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai27check_stat_unable_cliff_xluEP9lua_State"]
    pub(super) fn check_stat_unable_cliff_xlu(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai28check_stat_unable_escape_airEP9lua_State"]
    pub(super) fn check_stat_unable_escape_air(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_stat_unable_attackEP9lua_State"]
    pub(super) fn check_stat_unable_attack(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai25check_stat_unable_specialEP9lua_State"]
    pub(super) fn check_stat_unable_special(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_stat_unable_jumpEP9lua_State"]
    pub(super) fn check_stat_unable_jump(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_stat_unable_shieldEP9lua_State"]
    pub(super) fn check_stat_unable_shield(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai15check_stat_haveEP9lua_State"]
    pub(super) fn check_stat_have(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai19check_stat_put_bombEP9lua_State"]
    pub(super) fn check_stat_put_bomb(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai28check_stat_can_use_superleafEP9lua_State"]
    pub(super) fn check_stat_can_use_superleaf(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai29check_stat_can_use_rocketbeltEP9lua_State"]
    pub(super) fn check_stat_can_use_rocketbelt(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai21check_stat_have_throwEP9lua_State"]
    pub(super) fn check_stat_have_throw(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai21check_stat_have_shootEP9lua_State"]
    pub(super) fn check_stat_have_shoot(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai21check_stat_have_swingEP9lua_State"]
    pub(super) fn check_stat_have_swing(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai25check_stat_dogs_blind_ownEP9lua_State"]
    pub(super) fn check_stat_dogs_blind_own(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai27check_stat_target_invisibleEP9lua_State"]
    pub(super) fn check_stat_target_invisible(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai16check_skill_statEP9lua_StateNS_13FighterAIStat11SkillStatusE"]
    pub(super) fn check_skill_stat(arg1: *mut lua_State, arg2: app::FighterAIStat::SkillStatus) -> bool;

    #[link_name = "_ZN3app2ai24check_spirits_event_statEP9lua_StateNS_13FighterAIStat18SpiritsEventStatusE"]
    pub(super) fn check_spirits_event_stat(arg1: *mut lua_State, arg2: app::FighterAIStat::SpiritsEventStatus) -> bool;

    #[link_name = "_ZN3app2ai14check_chr_statEP9lua_StateNS_13FighterAIStat15CharacterStatusENS_11FighterKindE"]
    pub(super) fn check_chr_stat(arg1: *mut lua_State, arg2: app::FighterAIStat::CharacterStatus, arg3: app::FighterKind) -> bool;

    #[link_name = "_ZN3app2ai15air_lasso_countEP9lua_State"]
    pub(super) fn air_lasso_count(arg1: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai15check_cliffableEP9lua_State"]
    pub(super) fn check_cliffable(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_cliffable_floor_lrEP9lua_Statef"]
    pub(super) fn check_cliffable_floor_lr(arg1: *mut lua_State, arg2: f32) -> bool;

    #[link_name = "_ZN3app2ai14check_passableEP9lua_State"]
    pub(super) fn check_passable(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai11shield_rateEP9lua_State"]
    pub(super) fn shield_rate(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai19damage_reaction_mulEP9lua_State"]
    pub(super) fn damage_reaction_mul(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai10stop_frameEP9lua_State"]
    pub(super) fn stop_frame(arg1: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai6heightEP9lua_State"]
    pub(super) fn height(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai5pos_xEP9lua_State"]
    pub(super) fn pos_x(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai5pos_yEP9lua_State"]
    pub(super) fn pos_y(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai7speed_xEP9lua_State"]
    pub(super) fn speed_x(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai7speed_yEP9lua_State"]
    pub(super) fn speed_y(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai5scaleEP9lua_State"]
    pub(super) fn scale(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai11status_kindEP9lua_State"]
    pub(super) fn status_kind(arg1: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai16prev_status_kindEP9lua_State"]
    pub(super) fn prev_status_kind(arg1: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai11motion_kindEP9lua_State"]
    pub(super) fn motion_kind(arg1: *mut lua_State) -> phx::Hash40;

    #[link_name = "_ZN3app2ai12motion_frameEP9lua_State"]
    pub(super) fn motion_frame(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai11motion_rateEP9lua_State"]
    pub(super) fn motion_rate(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai19motion_cancel_frameEP9lua_State"]
    pub(super) fn motion_cancel_frame(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai19jump_rest_availableEP9lua_State"]
    pub(super) fn jump_rest_available(arg1: *mut lua_State) -> u16;

    #[link_name = "_ZN3app2ai17is_sp_u_availableEP9lua_State"]
    pub(super) fn is_sp_u_available(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24is_sp_u_weaken_availableEP9lua_State"]
    pub(super) fn is_sp_u_weaken_available(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai6damageEP9lua_State"]
    pub(super) fn damage(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai2hpEP9lua_State"]
    pub(super) fn hp(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai2lrEP9lua_State"]
    pub(super) fn lr(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai11customize_nEP9lua_State"]
    pub(super) fn customize_n(arg1: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai11customize_sEP9lua_State"]
    pub(super) fn customize_s(arg1: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai12customize_hiEP9lua_State"]
    pub(super) fn customize_hi(arg1: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai12customize_lwEP9lua_State"]
    pub(super) fn customize_lw(arg1: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai17check_use_commandEP9lua_State"]
    pub(super) fn check_use_command(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_use_command_typeEP9lua_State"]
    pub(super) fn check_use_command_type(arg1: *mut lua_State) -> u8;

    #[link_name = "_ZN3app2ai22check_command_236_stepEP9lua_State"]
    pub(super) fn check_command_236_step(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_command_41236_stepEP9lua_State"]
    pub(super) fn check_command_41236_step(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_command_214_stepEP9lua_State"]
    pub(super) fn check_command_214_step(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_command_623_stepEP9lua_State"]
    pub(super) fn check_command_623_step(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai25check_command_236236_stepEP9lua_State"]
    pub(super) fn check_command_236236_step(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_command_21416_stepEP9lua_State"]
    pub(super) fn check_command_21416_step(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai25check_command_214214_stepEP9lua_State"]
    pub(super) fn check_command_214214_step(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_command_23634_stepEP9lua_State"]
    pub(super) fn check_command_23634_step(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai18fighter_uniq_valueEP9lua_State"]
    pub(super) fn fighter_uniq_value(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai19fighter_uniq_value2EP9lua_Statei"]
    pub(super) fn fighter_uniq_value2(arg1: *mut lua_State, arg2: i32) -> f32;

    #[link_name = "_ZN3app2ai18line_segment_checkEP9lua_StateRKN3phx8Vector2fE"]
    pub(super) fn line_segment_check(arg1: *mut lua_State, arg2: *const phx::Vector2f) -> bool;

    #[link_name = "_ZN3app2ai29line_segment_check_from_top_nEP9lua_StateRKN3phx8Vector2fES6_"]
    pub(super) fn line_segment_check_from_top_n(arg1: *mut lua_State, arg2: *const phx::Vector2f, arg3: *const phx::Vector2f) -> bool;

    #[link_name = "_ZN3app2ai28line_segment_check_only_roofEP9lua_StateRKN3phx8Vector2fE"]
    pub(super) fn line_segment_check_only_roof(arg1: *mut lua_State, arg2: *const phx::Vector2f) -> bool;

    #[link_name = "_ZN3app2ai29line_segment_check_only_floorEP9lua_StateRKN3phx8Vector2fE"]
    pub(super) fn line_segment_check_only_floor(arg1: *mut lua_State, arg2: *const phx::Vector2f) -> bool;

    #[link_name = "_ZN3app2ai28line_segment_check_only_wallEP9lua_StateRKN3phx8Vector2fE"]
    pub(super) fn line_segment_check_only_wall(arg1: *mut lua_State, arg2: *const phx::Vector2f) -> bool;

    #[link_name = "_ZN3app2ai12weapon_pos_xEP9lua_State"]
    pub(super) fn weapon_pos_x(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai12weapon_pos_yEP9lua_State"]
    pub(super) fn weapon_pos_y(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai14weapon_speed_xEP9lua_State"]
    pub(super) fn weapon_speed_x(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai14weapon_speed_yEP9lua_State"]
    pub(super) fn weapon_speed_y(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai19target_fighter_kindEP9lua_State"]
    pub(super) fn target_fighter_kind(arg1: *mut lua_State) -> app::FighterKind;

    #[link_name = "_ZN3app2ai24target_copy_fighter_kindEP9lua_State"]
    pub(super) fn target_copy_fighter_kind(arg1: *mut lua_State) -> app::FighterKind;

    #[link_name = "_ZN3app2ai33target_current_attack_start_frameEP9lua_State"]
    pub(super) fn target_current_attack_start_frame(arg1: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai37target_current_attack_combo_end_frameEP9lua_State"]
    pub(super) fn target_current_attack_combo_end_frame(arg1: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai34target_current_attack_cancel_frameEP9lua_State"]
    pub(super) fn target_current_attack_cancel_frame(arg1: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai21check_target_stat_airEP9lua_State"]
    pub(super) fn check_target_stat_air(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai26check_target_stat_build_upEP9lua_State"]
    pub(super) fn check_target_stat_build_up(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai27check_target_stat_attentionEP9lua_State"]
    pub(super) fn check_target_stat_attention(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_target_stat_finalEP9lua_State"]
    pub(super) fn check_target_stat_final(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_target_stat_deadEP9lua_State"]
    pub(super) fn check_target_stat_dead(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai28check_target_stat_invincibleEP9lua_State"]
    pub(super) fn check_target_stat_invincible(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai30check_target_stat_invincible_lEP9lua_State"]
    pub(super) fn check_target_stat_invincible_l(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai30check_target_stat_attack_catchEP9lua_State"]
    pub(super) fn check_target_stat_attack_catch(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai25check_target_stat_reflectEP9lua_State"]
    pub(super) fn check_target_stat_reflect(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai32check_target_stat_unguarded_hindEP9lua_State"]
    pub(super) fn check_target_stat_unguarded_hind(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai27check_target_stat_unguardedEP9lua_State"]
    pub(super) fn check_target_stat_unguarded(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_target_stat_comboEP9lua_State"]
    pub(super) fn check_target_stat_combo(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai28check_target_stat_no_counterEP9lua_State"]
    pub(super) fn check_target_stat_no_counter(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_target_stat_downEP9lua_State"]
    pub(super) fn check_target_stat_down(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai25check_target_stat_fall_spEP9lua_State"]
    pub(super) fn check_target_stat_fall_sp(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_target_stat_piyoEP9lua_State"]
    pub(super) fn check_target_stat_piyo(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_target_stat_piyo_lEP9lua_State"]
    pub(super) fn check_target_stat_piyo_l(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai25check_target_stat_dragoonEP9lua_State"]
    pub(super) fn check_target_stat_dragoon(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_target_stat_cliffEP9lua_State"]
    pub(super) fn check_target_stat_cliff(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai27check_target_stat_cliff_actEP9lua_State"]
    pub(super) fn check_target_stat_cliff_act(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_target_stat_catchEP9lua_State"]
    pub(super) fn check_target_stat_catch(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_target_stat_damageEP9lua_State"]
    pub(super) fn check_target_stat_damage(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_target_stat_guardEP9lua_State"]
    pub(super) fn check_target_stat_guard(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_target_stat_escapeEP9lua_State"]
    pub(super) fn check_target_stat_escape(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai25check_target_stat_rebirthEP9lua_State"]
    pub(super) fn check_target_stat_rebirth(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_target_stat_attackEP9lua_State"]
    pub(super) fn check_target_stat_attack(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai29check_target_stat_attack_holdEP9lua_State"]
    pub(super) fn check_target_stat_attack_hold(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_target_stat_squatEP9lua_State"]
    pub(super) fn check_target_stat_squat(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai31check_target_stat_unable_attackEP9lua_State"]
    pub(super) fn check_target_stat_unable_attack(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai32check_target_stat_unable_specialEP9lua_State"]
    pub(super) fn check_target_stat_unable_special(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai35check_target_stat_specialflag_hoistEP9lua_State"]
    pub(super) fn check_target_stat_specialflag_hoist(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai21check_target_chr_statEP9lua_StateNS_13FighterAIStat15CharacterStatusENS_11FighterKindE"]
    pub(super) fn check_target_chr_stat(arg1: *mut lua_State, arg2: app::FighterAIStat::CharacterStatus, arg3: app::FighterKind) -> bool;

    #[link_name = "_ZN3app2ai12target_widthEP9lua_State"]
    pub(super) fn target_width(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai13target_heightEP9lua_State"]
    pub(super) fn target_height(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai12target_pos_xEP9lua_State"]
    pub(super) fn target_pos_x(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai12target_pos_yEP9lua_State"]
    pub(super) fn target_pos_y(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai14target_speed_xEP9lua_State"]
    pub(super) fn target_speed_x(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai14target_speed_yEP9lua_State"]
    pub(super) fn target_speed_y(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai12target_scaleEP9lua_State"]
    pub(super) fn target_scale(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai26target_jump_rest_availableEP9lua_State"]
    pub(super) fn target_jump_rest_available(arg1: *mut lua_State) -> u16;

    #[link_name = "_ZN3app2ai13target_damageEP9lua_State"]
    pub(super) fn target_damage(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai9target_lrEP9lua_State"]
    pub(super) fn target_lr(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai18target_status_kindEP9lua_State"]
    pub(super) fn target_status_kind(arg1: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai18target_motion_kindEP9lua_State"]
    pub(super) fn target_motion_kind(arg1: *mut lua_State) -> phx::Hash40;

    #[link_name = "_ZN3app2ai19target_motion_frameEP9lua_State"]
    pub(super) fn target_motion_frame(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai25target_hit_collision_rectEP9lua_State"]
    pub(super) fn target_hit_collision_rect(arg1: *mut lua_State) -> cpp::simd::Vector4;

    #[link_name = "_ZN3app2ai12lr_to_targetEP9lua_State"]
    pub(super) fn lr_to_target(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai20is_looking_at_targetEP9lua_State"]
    pub(super) fn is_looking_at_target(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai18distance_to_targetEP9lua_State"]
    pub(super) fn distance_to_target(arg1: *mut lua_State) -> cpp::simd::Vector2;

    #[link_name = "_ZN3app2ai20distance_x_to_targetEP9lua_State"]
    pub(super) fn distance_x_to_target(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai20distance_y_to_targetEP9lua_State"]
    pub(super) fn distance_y_to_target(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai23is_target_on_same_floorEP9lua_State"]
    pub(super) fn is_target_on_same_floor(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_any_danger_targetEP9lua_State"]
    pub(super) fn check_any_danger_target(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_parent_over_groundEP9lua_State"]
    pub(super) fn check_parent_over_ground(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_parent_same_floorEP9lua_State"]
    pub(super) fn check_parent_same_floor(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai12parent_pos_yEP9lua_State"]
    pub(super) fn parent_pos_y(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai14parent_speed_yEP9lua_State"]
    pub(super) fn parent_speed_y(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai22floor_edge_distance_lrEP9lua_Statef"]
    pub(super) fn floor_edge_distance_lr(arg1: *mut lua_State, arg2: f32) -> f32;

    #[link_name = "_ZN3app2ai21floor_edge_distance_fEP9lua_State"]
    pub(super) fn floor_edge_distance_f(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai21floor_edge_distance_bEP9lua_State"]
    pub(super) fn floor_edge_distance_b(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai28floor_edge_distance_floor_lrEP9lua_State"]
    pub(super) fn floor_edge_distance_floor_lr(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai34floor_edge_distance_floor_lr_movedEP9lua_Statef"]
    pub(super) fn floor_edge_distance_floor_lr_moved(arg1: *mut lua_State, arg2: f32) -> f32;

    #[link_name = "_ZN3app2ai11floor_widthEP9lua_State"]
    pub(super) fn floor_width(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai12floor_centerEP9lua_State"]
    pub(super) fn floor_center(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai8floor_lrEP9lua_State"]
    pub(super) fn floor_lr(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai29target_floor_edge_distance_lrEP9lua_Statef"]
    pub(super) fn target_floor_edge_distance_lr(arg1: *mut lua_State, arg2: f32) -> f32;

    #[link_name = "_ZN3app2ai17check_over_groundEP9lua_State"]
    pub(super) fn check_over_ground(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai37check_over_ground_distance_current_lrEP9lua_Statef"]
    pub(super) fn check_over_ground_distance_current_lr(arg1: *mut lua_State, arg2: f32) -> bool;

    #[link_name = "_ZN3app2ai24check_target_over_groundEP9lua_State"]
    pub(super) fn check_target_over_ground(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai15check_over_goalEP9lua_State"]
    pub(super) fn check_over_goal(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai9floor_posEP9lua_Stateb"]
    pub(super) fn floor_pos(arg1: *mut lua_State, arg2: bool) -> cpp::simd::Vector2;

    #[link_name = "_ZN3app2ai15floor_pos_movedEP9lua_Statefb"]
    pub(super) fn floor_pos_moved(arg1: *mut lua_State, arg2: f32, arg3: bool) -> cpp::simd::Vector2;

    #[link_name = "_ZN3app2ai18floor_pos_floor_lrEP9lua_Statefb"]
    pub(super) fn floor_pos_floor_lr(arg1: *mut lua_State, arg2: f32, arg3: bool) -> cpp::simd::Vector2;

    #[link_name = "_ZN3app2ai11floor_movesEP9lua_State"]
    pub(super) fn floor_moves(arg1: *mut lua_State) -> cpp::simd::Vector2;

    #[link_name = "_ZN3app2ai10return_posEP9lua_Stateb"]
    pub(super) fn return_pos(arg1: *mut lua_State, arg2: bool) -> cpp::simd::Vector2;

    #[link_name = "_ZN3app2ai15safe_return_posEP9lua_Stateb"]
    pub(super) fn safe_return_pos(arg1: *mut lua_State, arg2: bool) -> cpp::simd::Vector2;

    #[link_name = "_ZN3app2ai8goal_posEP9lua_State"]
    pub(super) fn goal_pos(arg1: *mut lua_State) -> cpp::simd::Vector2;

    #[link_name = "_ZN3app2ai16check_away_floorEP9lua_State"]
    pub(super) fn check_away_floor(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24is_offensive_on_strategyEP9lua_State"]
    pub(super) fn is_offensive_on_strategy(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24is_defensive_on_strategyEP9lua_State"]
    pub(super) fn is_defensive_on_strategy(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai11press_frameEP9lua_State"]
    pub(super) fn press_frame(arg1: *mut lua_State) -> u8;

    #[link_name = "_ZN3app2ai9push_waitEP9lua_State"]
    pub(super) fn push_wait(arg1: *mut lua_State) -> u8;

    #[link_name = "_ZN3app2ai13change_actionEP9lua_Statet"]
    pub(super) fn change_action(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId);

    #[link_name = "_ZN3app2ai13set_auto_stopEP9lua_Statei"]
    pub(super) fn set_auto_stop(arg1: *mut lua_State, arg2: i32);

    #[link_name = "_ZN3app2ai12update_countEP9lua_State"]
    pub(super) fn update_count(arg1: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai19is_update_count_oddEP9lua_State"]
    pub(super) fn is_update_count_odd(arg1: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai18reset_return_countEP9lua_State"]
    pub(super) fn reset_return_count(arg1: *mut lua_State);

    #[link_name = "_ZN3app2ai19set_no_return_frameEP9lua_Statei"]
    pub(super) fn set_no_return_frame(arg1: *mut lua_State, arg2: i32);

    #[link_name = "_ZN3app2ai22get_cmd_id_from_req_idEP9lua_StateNS_12FighterAIAct5ReqIdE"]
    pub(super) fn get_cmd_id_from_req_id(arg1: *mut lua_State, arg2: app::FighterAIAct::ReqId) -> app::FighterAIAct::CmdId;

    #[link_name = "_ZN3app2ai35get_cmd_id_from_req_id_with_predictEP9lua_StateNS_12FighterAIAct5ReqIdEfff"]
    pub(super) fn get_cmd_id_from_req_id_with_predict(arg1: *mut lua_State, arg2: app::FighterAIAct::ReqId, arg3: f32, arg4: f32, arg5: f32) -> app::FighterAIAct::CmdId;

    #[link_name = "_ZN3app2ai31get_cmd_probability_from_req_idEP9lua_StateNS_12FighterAIAct5ReqIdENS3_5CmdIdE"]
    pub(super) fn get_cmd_probability_from_req_id(arg1: *mut lua_State, arg2: app::FighterAIAct::ReqId, arg3: app::FighterAIAct::CmdId) -> f32;

    #[link_name = "_ZN3app2ai14enable_commandEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn enable_command(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId);

    #[link_name = "_ZN3app2ai15disable_commandEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn disable_command(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId);

    #[link_name = "_ZN3app2ai26disable_command_ground_allEP9lua_State"]
    pub(super) fn disable_command_ground_all(arg1: *mut lua_State);

    #[link_name = "_ZN3app2ai23disable_command_air_allEP9lua_State"]
    pub(super) fn disable_command_air_all(arg1: *mut lua_State);

    #[link_name = "_ZN3app2ai33disable_command_attack_button_allEP9lua_State"]
    pub(super) fn disable_command_attack_button_all(arg1: *mut lua_State);

    #[link_name = "_ZN3app2ai34disable_command_special_button_allEP9lua_State"]
    pub(super) fn disable_command_special_button_all(arg1: *mut lua_State);

    #[link_name = "_ZN3app2ai32reset_cmd_id_probability_add_2ndEP9lua_State"]
    pub(super) fn reset_cmd_id_probability_add_2nd(arg1: *mut lua_State);

    #[link_name = "_ZN3app2ai30set_cmd_id_probability_add_2ndEP9lua_StateNS_12FighterAIAct5CmdIdEf"]
    pub(super) fn set_cmd_id_probability_add_2nd(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId, arg3: f32);

    #[link_name = "_ZN3app2ai32reset_cmd_id_probability_mul_2ndEP9lua_State"]
    pub(super) fn reset_cmd_id_probability_mul_2nd(arg1: *mut lua_State);

    #[link_name = "_ZN3app2ai30set_cmd_id_probability_mul_2ndEP9lua_StateNS_12FighterAIAct5CmdIdEf"]
    pub(super) fn set_cmd_id_probability_mul_2nd(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId, arg3: f32);

    #[link_name = "_ZN3app2ai42set_cmd_id_probability_mul_for_specializerEP9lua_StateNS_12FighterAIAct5CmdIdEf"]
    pub(super) fn set_cmd_id_probability_mul_for_specializer(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId, arg3: f32);

    #[link_name = "_ZN3app2ai26get_cmd_id_probability_mulEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn get_cmd_id_probability_mul(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> f32;

    #[link_name = "_ZN3app2ai21predict_landing_frameEP9lua_State"]
    pub(super) fn predict_landing_frame(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai28predict_target_landing_frameEP9lua_State"]
    pub(super) fn predict_target_landing_frame(arg1: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai28predict_hit_in_target_attackEP9lua_StateNS_12FighterAIAct5CmdIdEff"]
    pub(super) fn predict_hit_in_target_attack(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId, arg3: f32, arg4: f32) -> bool;

    #[link_name = "_ZN3app2ai40predict_hit_in_target_attack_from_motionEP9lua_StateN3phx6Hash40Eff"]
    pub(super) fn predict_hit_in_target_attack_from_motion(arg1: *mut lua_State, arg2: phx::Hash40, arg3: f32, arg4: f32) -> bool;

    #[link_name = "_ZN3app2ai28predict_target_hit_in_attackEP9lua_StateNS_12FighterAIAct5CmdIdEffff"]
    pub(super) fn predict_target_hit_in_attack(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId, arg3: f32, arg4: f32, arg5: f32, arg6: f32) -> bool;

    #[link_name = "_ZN3app2ai35check_line_segment_vs_target_attackEP9lua_StateN3phx6Hash40ERKNS3_8Vector2fES7_RNS3_8Vector4fE"]
    pub(super) fn check_line_segment_vs_target_attack(arg1: *mut lua_State, arg2: phx::Hash40, arg3: *const phx::Vector2f, arg4: *const phx::Vector2f, arg5: *mut phx::Vector4f) -> bool;

    #[link_name = "_ZN3app2ai18attack_start_frameEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_start_frame(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> i32;

    #[link_name = "_ZN3app2ai25target_attack_start_frameEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn target_attack_start_frame(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> i32;

    #[link_name = "_ZN3app2ai16attack_end_frameEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_end_frame(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> i32;

    #[link_name = "_ZN3app2ai19attack_cancel_frameEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_cancel_frame(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> i32;

    #[link_name = "_ZN3app2ai14attack_data_x0EP9lua_StateN3phx6Hash40E"]
    pub(super) fn attack_data_x0(arg1: *mut lua_State, arg2: phx::Hash40) -> f32;

    #[link_name = "_ZN3app2ai14attack_data_x1EP9lua_StateN3phx6Hash40E"]
    pub(super) fn attack_data_x1(arg1: *mut lua_State, arg2: phx::Hash40) -> f32;

    #[link_name = "_ZN3app2ai14attack_data_y0EP9lua_StateN3phx6Hash40E"]
    pub(super) fn attack_data_y0(arg1: *mut lua_State, arg2: phx::Hash40) -> f32;

    #[link_name = "_ZN3app2ai14attack_data_y1EP9lua_StateN3phx6Hash40E"]
    pub(super) fn attack_data_y1(arg1: *mut lua_State, arg2: phx::Hash40) -> f32;

    #[link_name = "_ZN3app2ai22attack_info_needs_turnEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_info_needs_turn(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> bool;

    #[link_name = "_ZN3app2ai20attack_info_reactionEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_info_reaction(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> f32;

    #[link_name = "_ZN3app2ai21attack_info_no_shieldEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_info_no_shield(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> bool;

    #[link_name = "_ZN3app2ai18attack_info_meteorEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_info_meteor(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> bool;

    #[link_name = "_ZN3app2ai23attack_info_reflectableEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_info_reflectable(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> bool;

    #[link_name = "_ZN3app2ai19attack_is_as_weaponEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_is_as_weapon(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> bool;

    #[link_name = "_ZN3app2ai20attack_info_distanceEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_info_distance(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> f32;

    #[link_name = "_ZN3app2ai16motion_to_cmd_idEP9lua_StateN3phx6Hash40E"]
    pub(super) fn motion_to_cmd_id(arg1: *mut lua_State, arg2: phx::Hash40) -> app::FighterAIAct::CmdId;

    #[link_name = "_ZN3app2ai29target_attack_info_needs_turnEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn target_attack_info_needs_turn(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> bool;

    #[link_name = "_ZN3app2ai28target_attack_info_no_shieldEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn target_attack_info_no_shield(arg1: *mut lua_State, arg2: app::FighterAIAct::CmdId) -> bool;

    #[link_name = "_ZN3app2ai23target_motion_to_cmd_idEP9lua_StateN3phx6Hash40E"]
    pub(super) fn target_motion_to_cmd_id(arg1: *mut lua_State, arg2: phx::Hash40) -> app::FighterAIAct::CmdId;

    #[link_name = "_ZN3app2ai32current_attack_combo_next_motionEP9lua_State"]
    pub(super) fn current_attack_combo_next_motion(arg1: *mut lua_State) -> phx::Hash40;

    #[link_name = "_ZN3app2ai39target_current_attack_combo_next_motionEP9lua_State"]
    pub(super) fn target_current_attack_combo_next_motion(arg1: *mut lua_State) -> phx::Hash40;

    #[link_name = "_ZN3app2ai37target_attack_start_frame_from_motionEP9lua_StateN3phx6Hash40E"]
    pub(super) fn target_attack_start_frame_from_motion(arg1: *mut lua_State, arg2: phx::Hash40) -> i32;
}
use crate::*;

#[test]
fn link_event() {
    app::LinkEvent::assert();
}

#[test]
fn link_event_capture() {
    app::LinkEventCapture::assert();
}

#[test]
fn link_event_capture_item() {
    app::LinkEventCaptureItem::assert();
}

#[test]
fn link_event_capture_driver() {
    app::LinkEventCaptureDriver::assert();
}

#[test]
fn link_event_capture_mimikkyu() {
    app::LinkEventCaptureMimikkyu::assert();
}

#[test]
fn link_event_capture_pulled() {
    app::LinkEventCapturePulled::assert();
}

#[test]
fn link_event_final() {
    app::LinkEventFinal::assert();
}

#[test]
fn link_event_mask() {
    app::LinkEventMask::assert();
}

#[test]
fn link_event_pos() {
    app::LinkEventPos::assert();
}

#[test]
fn link_event_star_shot() {
    app::LinkEventStarShot::assert();
}

#[test]
fn link_event_throw() {
    app::LinkEventThrow::assert();
}

#[test]
fn link_event_touch_item() {
    app::LinkEventTouchItem::assert();
}

#[test]
fn link_event_yoshi_tamago_damage_effect() {
    app::LinkEventYoshiTamagoDamageEffect::assert();
}

#[test]
fn fighter_cloud_link_event_final() {
    app::FighterCloudLinkEventFinal::assert();
}

#[test]
fn fighter_inkling_link_event_paint() {
    app::FighterInklingLinkEventPaint::assert();
}

#[test]
fn fighter_pikmin_link_event_weapon_pikmin_change_motion() {
    app::FighterPikminLinkEventWeaponPikminChangeMotion::assert();
}

#[test]
fn fighter_pikmin_link_event_weapon_pikmin_change_status() {
    app::FighterPikminLinkEventWeaponPikminChangeStatus::assert();
}

#[test]
fn fighter_pikmin_link_event_weapon_pikmin_constraint() {
    app::FighterPikminLinkEventWeaponPikminConstraint::assert();
}

#[test]
fn fighter_pikmin_link_event_weapon_pikmin_on_flag() {
    app::FighterPikminLinkEventWeaponOnFlag::assert();
}

#[test]
fn fighter_pikmin_link_event_weapon_pikmin_set_float() {
    app::FighterPikminLinkEventWeaponSetFloat::assert();
}

#[test]
fn fighter_pikmin_link_event_weapon_pikmin_set_int() {
    app::FighterPikminLinkEventWeaponSetInt::assert();
}

#[test]
fn fighter_pikmin_link_event_weapon_pikmin_set_power_mul_status() {
    app::FighterPikminLinkEventWeaponSetPowerMulStatus::assert();
}

#[test]
fn fighter_pikmin_link_event_weapon_pikmin_sync_lr() {
    app::FighterPikminLinkEventWeaponSyncLR::assert();
}

#[test]
fn fighter_pikmin_link_event_weapon_pikmin_sync_pos() {
    app::FighterPikminLinkEventWeaponSyncPos::assert();
}

#[test]
fn l2c_agent() {
    lib::L2CAgent::assert();
}

#[test]
fn l2c_table() {
    lib::L2CTable::assert();
}

#[test]
fn l2c_value() {
    lib::L2CValue::assert();
}

#[test]
fn l2c_value_hack() {
    lib::L2CValueHack::assert();
}

#[test]
fn rect() {
    lib::Rect::assert();
}

#[test]
fn l2c_agent_base() {
    lua2cpp::L2CAgentBase::assert();
}

#[test]
fn l2c_agent_generated_base() {
    lua2cpp::L2CAgentGeneratedBase::assert();
}

#[test]
fn l2c_fighter_base() {
    lua2cpp::L2CFighterBase::assert();
}

#[test]
fn l2c_fighter_common() {
    lua2cpp::L2CFighterCommon::assert();
}

#[test]
fn l2c_weapon_common() {
    lua2cpp::L2CWeaponCommon::assert();
}

#[test]
fn l2c_fighter_ai_base() {
    lua2cpp::L2CFighterAIBase::assert();
}

#[test]
fn l2c_fighter_ai_action_base() {
    lua2cpp::L2CFighterAIActionBase::assert();
}

#[test]
fn l2c_fighter_ai_analyst_base() {
    lua2cpp::L2CFighterAIAnalystBase::assert();
}

#[test]
fn l2c_fighter_ai_mode_base() {
    lua2cpp::L2CFighterAIModeBase::assert();
}

#[test]
fn l2c_fighter_animcmd_effect_common() {
    lua2cpp::L2CFighterAnimcmdEffectCommon::assert();
}

#[test]
fn l2c_fighter_animcmd_expression_common() {
    lua2cpp::L2CFighterAnimcmdExpressionCommon::assert();
}

#[test]
fn l2c_fighter_animcmd_game_common() {
    lua2cpp::L2CFighterAnimcmdGameCommon::assert();
}

#[test]
fn l2c_fighter_animcmd_sound_common() {
    lua2cpp::L2CFighterAnimcmdSoundCommon::assert();
}

#[test]
fn general() {
    validate();
}
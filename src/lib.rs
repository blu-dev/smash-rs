pub mod app;
pub mod cpp;
mod lib_impl;
pub mod lua2cpp;
pub mod phx;

pub use lib_impl::lib;

#[cfg(test)]
mod test;

#[macro_export]
macro_rules! size_of {
    ($ty:tt) => {
        std::mem::size_of::<$ty>()
    }
}

#[cfg(feature = "type_assert")]
#[macro_use]
extern crate memoffset;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct lua_State(u64);

#[cfg(feature = "type_assert")]
pub fn validate() {
    app::LinkEvent::assert();
    app::LinkEventCapture::assert();
    app::LinkEventCaptureItem::assert();
    app::LinkEventCaptureDriver::assert();
    app::LinkEventCaptureMimikkyu::assert();
    app::LinkEventCapturePulled::assert();
    app::LinkEventFinal::assert();
    app::LinkEventMask::assert();
    app::LinkEventPos::assert();
    app::LinkEventStarShot::assert();
    app::LinkEventThrow::assert();
    app::LinkEventTouchItem::assert();
    app::LinkEventYoshiTamagoDamageEffect::assert();
    app::FighterCloudLinkEventFinal::assert();
    app::FighterInklingLinkEventPaint::assert();
    app::FighterPikminLinkEventWeaponPikminChangeMotion::assert();
    app::FighterPikminLinkEventWeaponPikminChangeStatus::assert();
    app::FighterPikminLinkEventWeaponPikminConstraint::assert();
    app::FighterPikminLinkEventWeaponOnFlag::assert();
    app::FighterPikminLinkEventWeaponSetFloat::assert();
    app::FighterPikminLinkEventWeaponSetInt::assert();
    app::FighterPikminLinkEventWeaponSetPowerMulStatus::assert();
    app::FighterPikminLinkEventWeaponSyncLR::assert();
    app::FighterPikminLinkEventWeaponSyncPos::assert();
    app::FighterPokemonLinkEventChange::assert();
    app::FighterRidleyLinkEventMotion::assert();
    app::FighterRyuLinkEventFinalDeadDamage::assert();
    app::FighterRyuLinkEventFinalMoveTarget::assert();
    app::FighterElementLinkEventChange::assert();
    app::WeaponPickelTrolleyLinkEventConfirmMaterial::assert();
    app::WeaponPickelTrolleyLinkEventConsumeMaterial::assert();
    app::WeaponPickelTrolleyLinkEventDestroyed::assert();
    app::WeaponPickelTrolleyLinkEventGetParam::assert();
    app::WeaponPickelTrolleyLinkEventRemoveIfDistance::assert();
    app::WeaponPickelTrolleyLinkEventRemoveRailByGeneration::assert();
    app::WeaponPickelTrolleyLinkEventTurnTorchOn::assert();
    app::WeaponRobotHominglaserLinkEventBurst::assert();
    app::WeaponRobotHominglaserLinkEventSearch::assert();

    lib::L2CAgent::assert();
    lib::L2CTable::assert();
    lib::L2CValue::assert();
    lib::L2CValueHack::assert();
    lib::Rect::assert();

    lua2cpp::L2CAgentBase::assert();
    lua2cpp::L2CAgentGeneratedBase::assert();
    lua2cpp::L2CFighterBase::assert();
    lua2cpp::L2CFighterCommon::assert();
    lua2cpp::L2CWeaponCommon::assert();

    lua2cpp::L2CFighterAIBase::assert();
    lua2cpp::L2CFighterAIActionBase::assert();
    lua2cpp::L2CFighterAIAnalystBase::assert();
    lua2cpp::L2CFighterAIModeBase::assert();

    lua2cpp::L2CFighterAnimcmdEffectCommon::assert();
    lua2cpp::L2CFighterAnimcmdExpressionCommon::assert();
    lua2cpp::L2CFighterAnimcmdGameCommon::assert();
    lua2cpp::L2CFighterAnimcmdSoundCommon::assert();
}
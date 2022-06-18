use crate::*;

#[repr(C)]
pub struct AttackAbsoluteData {
    pub power: f32,
    pub vector: i32,
    pub r_eff: i32,
    pub r_fix: i32,
    pub r_add: i32,
    pub slip: f32,
    pub stop_frame: f32,
    pub stop_delay: f32,
    pub attr: phx::Hash40,
    pub sound_level: CollisionSoundLevel,
    pub sound_attr: CollisionSoundAttr,
    pub lr_check: AttackLRCheck,
    pub no_stop: bool,
    pub no_effect: bool,
    unused: u8,
    pub region: AttackRegion,
    pub catch: bool
}

#[cfg(feature = "type_assert")]
impl AttackAbsoluteData {
    pub fn assert() {
        assert_eq!(size_of!(AttackAbsoluteData), 0x30);
        assert_eq!(offset_of!(AttackAbsoluteData, power),       0x0);
        assert_eq!(offset_of!(AttackAbsoluteData, vector),      0x4);
        assert_eq!(offset_of!(AttackAbsoluteData, r_eff),       0x8);
        assert_eq!(offset_of!(AttackAbsoluteData, r_fix),       0xC);
        assert_eq!(offset_of!(AttackAbsoluteData, r_add),       0x10);
        assert_eq!(offset_of!(AttackAbsoluteData, slip),        0x14);
        assert_eq!(offset_of!(AttackAbsoluteData, stop_frame),  0x18);
        assert_eq!(offset_of!(AttackAbsoluteData, stop_delay),  0x1C);
        assert_eq!(offset_of!(AttackAbsoluteData, attr),        0x20);
        assert_eq!(offset_of!(AttackAbsoluteData, sound_level), 0x28);
        assert_eq!(offset_of!(AttackAbsoluteData, sound_attr),  0x29);
        assert_eq!(offset_of!(AttackAbsoluteData, lr_check),    0x2A);
        assert_eq!(offset_of!(AttackAbsoluteData, no_stop),     0x2B);
        assert_eq!(offset_of!(AttackAbsoluteData, no_effect),   0x2C);
        assert_eq!(offset_of!(AttackAbsoluteData, region),      0x2E);
        assert_eq!(offset_of!(AttackAbsoluteData, catch),       0x2F);
    }
}

#[repr(C)]
pub struct AttackData {
    pub offset: phx::Vector3f,
    padding1: f32,
    pub offset2: phx::Vector3f,
    padding2: f32,
    pub power: f32,
    pub size: f32,
    pub vector: i32,
    pub r_eff: i32,
    pub r_fix: i32,
    pub r_add: i32,
    pub slip: f32,
    pub stop_frame: f32,
    pub stop_delay: f32,
    pub node: phx::Hash40,
    pub target_category: CollisionCategoryMask,
    pub target_situation: CollisionSituationMask,
    pub target_lr: bool,
    pub target_part: CollisionPartMask,
    pub attr: phx::Hash40,
    pub sound_level: CollisionSoundLevel,
    pub sound_attr: CollisionSoundAttr,
    pub set_off: AttackSetOffKind,
    pub no_scale: bool,
    pub shield: bool,
    pub reflector: bool,
    pub absorber: bool,
    pub direct: bool,
    pub no_invincible: bool,
    pub no_xlu: bool,
    pub lr_check: AttackLRCheck,
    pub catch: bool,
    pub no_team: bool,
    pub no_stop: bool,
    pub no_effect: bool,
    unused1: u8,
    pub speed: bool,
    pub region: AttackRegion,
    pub ignore_down: bool,
    pub check_type: CollisionShapeType,
    pub sub_shield: u16,
    pub camera_quake: CameraQuakeKind,
    pub serial_hit_frame: u32,
    pub force_reaction: bool,
    pub no_attacker_log: bool,
    pub no_weight_reaction: u8,
    pub no_reaction_search: u8,
    pub keep_rumble: bool,
    pub composition_speed: bool,
    pub target_pos_node: phx::Hash40,
    pub target_pos_offset: phx::Vector2f,
    padding3: u64,
    pub target_pos_frame: i32,
    pub r_fix_damage_speed_up: bool,
    unused2: u8,
    pub captured_same_time_attack: bool,
    unknown: [u8; 0x59] // 0xA7
}

#[cfg(feature = "type_assert")]
impl AttackData {
    pub fn assert() {
        assert_eq!(size_of!(AttackData), 0x100);
        assert_eq!(offset_of!(AttackData, offset),                    0x0);
        assert_eq!(offset_of!(AttackData, offset2),                   0x10);
        assert_eq!(offset_of!(AttackData, power),                     0x20);
        assert_eq!(offset_of!(AttackData, size),                      0x24);
        assert_eq!(offset_of!(AttackData, vector),                    0x28);
        assert_eq!(offset_of!(AttackData, r_eff),                     0x2C);
        assert_eq!(offset_of!(AttackData, r_fix),                     0x30);
        assert_eq!(offset_of!(AttackData, r_add),                     0x34);
        assert_eq!(offset_of!(AttackData, slip),                      0x38);
        assert_eq!(offset_of!(AttackData, stop_frame),                0x3C);
        assert_eq!(offset_of!(AttackData, stop_delay),                0x40);
        assert_eq!(offset_of!(AttackData, node),                      0x48);
        assert_eq!(offset_of!(AttackData, target_category),           0x50);
        assert_eq!(offset_of!(AttackData, target_situation),          0x52);
        assert_eq!(offset_of!(AttackData, target_lr),                 0x53);
        assert_eq!(offset_of!(AttackData, target_part),               0x54);
        assert_eq!(offset_of!(AttackData, attr),                      0x58);
        assert_eq!(offset_of!(AttackData, sound_level),               0x60);
        assert_eq!(offset_of!(AttackData, sound_attr),                0x61);
        assert_eq!(offset_of!(AttackData, set_off),                   0x62);
        assert_eq!(offset_of!(AttackData, no_scale),                  0x63);
        assert_eq!(offset_of!(AttackData, shield),                    0x64);
        assert_eq!(offset_of!(AttackData, reflector),                 0x65);
        assert_eq!(offset_of!(AttackData, absorber),                  0x66);
        assert_eq!(offset_of!(AttackData, direct),                    0x67);
        assert_eq!(offset_of!(AttackData, no_invincible),             0x68);
        assert_eq!(offset_of!(AttackData, no_xlu),                    0x69);
        assert_eq!(offset_of!(AttackData, lr_check),                  0x6A);
        assert_eq!(offset_of!(AttackData, catch),                     0x6B);
        assert_eq!(offset_of!(AttackData, no_team),                   0x6C);
        assert_eq!(offset_of!(AttackData, no_stop),                   0x6D);
        assert_eq!(offset_of!(AttackData, no_effect),                 0x6E);
        assert_eq!(offset_of!(AttackData, speed),                     0x70);
        assert_eq!(offset_of!(AttackData, region),                    0x71);
        assert_eq!(offset_of!(AttackData, ignore_down),               0x72);
        assert_eq!(offset_of!(AttackData, check_type),                0x73);
        assert_eq!(offset_of!(AttackData, sub_shield),                0x74);
        assert_eq!(offset_of!(AttackData, camera_quake),              0x76);
        assert_eq!(offset_of!(AttackData, serial_hit_frame),          0x78);
        assert_eq!(offset_of!(AttackData, force_reaction),            0x7C);
        assert_eq!(offset_of!(AttackData, no_attacker_log),           0x7D);
        assert_eq!(offset_of!(AttackData, no_weight_reaction),        0x7E);
        assert_eq!(offset_of!(AttackData, no_reaction_search),        0x7F);
        assert_eq!(offset_of!(AttackData, keep_rumble),               0x80);
        assert_eq!(offset_of!(AttackData, composition_speed),         0x81);
        assert_eq!(offset_of!(AttackData, target_pos_node),           0x88);
        assert_eq!(offset_of!(AttackData, target_pos_offset),         0x90);
        assert_eq!(offset_of!(AttackData, target_pos_frame),          0xA0);
        assert_eq!(offset_of!(AttackData, r_fix_damage_speed_up),     0xA4);
        assert_eq!(offset_of!(AttackData, captured_same_time_attack), 0xA6);
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default)]
    #[allow(non_upper_case_globals)]
    pub struct CollisionCategoryMask: u16 {
        const Fighter = 0x1;
        const Enemy = 0x2;
        const Item = 0x4;
        const Gimmick = 0x8;
        const ItemE = 0x10;
        const Floor = 0x20;
    }
}

bitflags! {
    #[repr(C)]
    #[allow(non_upper_case_globals)]
    pub struct CollisionSituationMask: u8 {
        const Ground = 0x1;
        const Air = 0x2;
        const Odd = 0x4;
    }
}

bitflags! {
    #[repr(C)]
    #[allow(non_upper_case_globals)]
    pub struct CollisionPartMask: u16 {
        const Etc = 0x2;
        const Body = 0x9;
        const Legs = 0xC;
        const Head = 0x10;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum CollisionSoundLevel {
    Small = 0x0,
    Medium = 0x1,
    Large = 0x2,
    ExtraLarge = 0x3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum CollisionSoundAttr {
    None = 0x0,
    Punch = 0x1,
    Kick = 0x2,
    CutUp = 0x3,
    Coin = 0x4,
    Bat = 0x5,
    Harisen = 0x6,
    Elec = 0x7,
    Fire = 0x8,
    Water = 0x9,
    Grass = 0xA,
    Bomb = 0xB,
    PeachWeapon = 0xC,
    Snake = 0xD,
    Ike = 0xE,
    Dedede = 0xF,
    Magic = 0x10,
    KameHit = 0x11,
    PeachBinta = 0x12,
    PeachFryingPan = 0x13,
    PeachGolf = 0x14,
    PeachTennis = 0x15,
    PeachParasol = 0x16,
    DaisyBinta = 0x17,
    DaisyFryingPan = 0x18,
    DaisyGolf = 0x19,
    DaisyTennis = 0x1A,
    DaisyParasol = 0x1B,
    Lucario = 0x1C,
    MarthSword = 0x1D,
    MarthFinal = 0x1E,
    MarioCoin = 0x1F,
    MarioFinal = 0x20,
    LuigiCoin = 0x21,
    NessBat = 0x22,
    Freeze = 0x23,
    MarioFireball = 0x24,
    MarioCoinLast = 0x25,
    MarioMant = 0x26,
    FoxBlaster = 0x27,
    LuigiAttackDash = 0x28,
    LuigiSmash = 0x29,
    MarioWaterPump = 0x2A,
    PacmanBell = 0x2B,
    GuruguruHit = 0x2C,
    LizardonFire = 0x2D,
    TrainHit = 0x2E,
    MarioDCoinLast = 0x2F,
    MarioDMant = 0x30,
    MarioDCapsule = 0x31,
    PacmanWater = 0x32,
    MiiGunnerBlaster = 0x33,
    RefletFinalSword = 0x34,
    RefletFinalFire = 0x35,
    RefletFinalElec = 0x36,
    DuckhuntFinal = 0x37,
    ShulkFinalDanban = 0x38,
    ShulkFinalRiki = 0x39,
    FalcoBlaster = 0x3A,
    RyuPunch = 0x3B,
    RyuKick = 0x3C,
    LucasBat = 0x3D,
    RyuFinal01 = 0x3E,
    RyuFinal02 = 0x3F,
    RyuFinal03 = 0x40,
    CloudHit = 0x41,
    CloudSmash01 = 0x42,
    CloudSmash02 = 0x43,
    CloudSmash03 = 0x44,
    CloudFinal01 = 0x45,
    CloudFinal02 = 0x46,
    CloudFinal03 = 0x47,
    BayonettaHit01 = 0x48,
    BayonettaHit02 = 0x49,
    YoshiBiteHit = 0x4A,
    YoshiEggHit = 0x4B,
    RoyHit = 0x4C,
    ChromHit = 0x4D,
    FoxTail = 0x4E,
    Heavy = 0x4F,
    Slap = 0x50,
    ItemHammer = 0x51,
    InklingHit = 0x52,
    MarioLocalCoin = 0x53,
    MarioLocalCoinLast = 0x54,
    FamicomHit = 0x55,
    ZenigameShellHit = 0x56,
    SamusScrew = 0x57,
    SamusDScrew = 0x58,
    SamusScrewFinish = 0x59,
    SamusDScrewFinish = 0x5A,
    KenPunch = 0x5B,
    KenKick = 0x5C,
    KenFinal01 = 0x5D,
    KenFinal02 = 0x5E,
    KenFinal03 = 0x5F,
    ShizueHammer = 0x60,
    SimonWhip = 0x61,
    SimonCross = 0x62,
    RichterWhip = 0x63,
    RichterCross = 0x64,
    SheikFinalPunch = 0x65,
    SheikFinalKick = 0x66,
    MetaknightFinalHit = 0x67,
    RobotFinalHit = 0x68,
    KenShoryu = 0x69,
    DiddyScratch = 0x6A,
    MiiEnemyGBlaster = 0x6B,
    ToonlinkHit = 0x6C,
    JackShot = 0x6D,
    BraveCriticalHit = 0x6E,
    BuddyWing = 0x6F,
    DollyPunch = 0x70,
    DollyKick = 0x71,
    DollyCritical = 0x72,
    DollySuperSpecial01 = 0x73,
    MasterAxe = 0x74,
    MasterArrowMax = 0x75,
    MasterAttack100End = 0x76,
    TantanPunch01 = 0x77,
    TantanPunch02 = 0x78,
    TantanPunch03 = 0x79,
    TantanFinal = 0x7A,
    CloudFinalAppendHit01 = 0x7B,
    CloudFinalAppendHit02 = 0x7C,
    FlameFinal = 0x7D,
    DemonPunch01 = 0x7E,
    DemonPunch02 = 0x7F,
    DemonKick = 0x80,
    DemonCatchAttack = 0x81,
    DemonFinal = 0x82,
    DemonThrowCommand = 0x83,
    DemonAttackSquat4 = 0x84,
    DemonAttackLw3 = 0x85,
    DemonAppeal = 0x86,
    TrailSlash = 0x87,
    TrailStab = 0x88,
    TrailCleave = 0x89,
    TrailCleaveSingle = 0x8A,
    TrailKick = 0x8B,
    TrailFinal = 0x8C,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum AttackSetOffKind {
    Off = 0x0,
    On = 0x1,
    Thru = 0x2,
    NoStop = 0x3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum AttackLRCheck {
    Pos = 0x0,
    Speed = 0x1,
    LR = 0x2,
    Forward = 0x3,
    Backward = 0x4,
    Part = 0x5,
    BackSlash = 0x6,
    Left = 0x7,
    Right = 0x8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum AttackRegion {
    None = 0x0,
    Head = 0x1,
    Body = 0x2,
    Hip = 0x3,
    Punch = 0x4,
    Elbow = 0x5,
    Kick = 0x6,
    Knee = 0x7,
    Throw = 0x8,
    Object = 0x9,
    Sword = 0xA,
    Hammer = 0xB,
    Bomb = 0xC,
    Spin = 0xD,
    Bite = 0xE,
    Magic = 0xF,
    PSI = 0x10,
    Palutena = 0x11,
    Aura = 0x12,
    Bat = 0x13,
    Parasol = 0x14,
    Pikmin = 0x15,
    Water = 0x16,
    Whip = 0x17,
    Tail = 0x18,
    Energy = 0x19,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum CollisionShapeType {
    Sphere = 0x0,
    AABB = 0x1,
    Capsule = 0x2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum CameraQuakeKind {
    None = 0x0,
    KeepSmall = 0x1,
    SmallHalf = 0x2,
    Small = 0x3,
    KeepMiddle = 0x4,
    Middle = 0x5,
    KeepLarge = 0x6,
    Large = 0x7,
    KeepMoreLarge = 0x8,
    MoreLarge = 0x9,
    PowBlock = 0xA,
    Knockout = 0xB,
    DollyStage = 0xC,
    XXL = 0xD,
    Invalid = 0xE,
}

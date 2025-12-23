use std::{collections::HashMap, fmt};

use lazy_static::lazy_static;
pub use rand;
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};
pub use serde;
pub use serde_json;
pub use uuid;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const NUM_HEROS: usize = 46;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Hero {
    Alice,
    Amelia,
    Ashur,
    Aya,
    Belita,
    Beni,
    BigWood,
    Butter,
    Canna,
    Chloe,
    Daya,
    Diana,
    Elena,
    Epica,
    Erpin,
    Espi,
    Festa,
    Fricle,
    Gabia,
    Hilde,
    Ifrit,
    // Jade,
    Jubee,
    Kidian,
    Kommy,
    Leets,
    Levi,
    MaestroMK2,
    Marie,
    Mayo,
    // Meluna,
    Naia,
    Ner,
    Posher,
    Rim,
    Rohne,
    Rude,
    Rufo,
    Selline,
    Shady,
    Silphir,
    Sist,
    Speaki,
    Sylla,
    Tig,
    Ui,
    // Velvet,
    Vivi,
    Xion,
}

impl Hero {
    pub fn new(index: usize) -> Option<Self> {
        match index {
            0 => Some(Hero::Alice),
            1 => Some(Hero::Amelia),
            2 => Some(Hero::Ashur),
            3 => Some(Hero::Aya),
            4 => Some(Hero::Belita),
            5 => Some(Hero::Beni),
            6 => Some(Hero::BigWood),
            7 => Some(Hero::Butter),
            8 => Some(Hero::Canna),
            9 => Some(Hero::Chloe),
            10 => Some(Hero::Daya),
            11 => Some(Hero::Diana),
            12 => Some(Hero::Elena),
            13 => Some(Hero::Epica),
            14 => Some(Hero::Erpin),
            15 => Some(Hero::Espi),
            16 => Some(Hero::Festa),
            17 => Some(Hero::Fricle),
            18 => Some(Hero::Gabia),
            19 => Some(Hero::Hilde),
            20 => Some(Hero::Ifrit),
            21 => Some(Hero::Jubee),
            22 => Some(Hero::Kidian),
            23 => Some(Hero::Kommy),
            24 => Some(Hero::Leets),
            25 => Some(Hero::Levi),
            26 => Some(Hero::MaestroMK2),
            27 => Some(Hero::Marie),
            28 => Some(Hero::Mayo),
            29 => Some(Hero::Naia),
            30 => Some(Hero::Ner),
            31 => Some(Hero::Posher),
            32 => Some(Hero::Rim),
            33 => Some(Hero::Rohne),
            34 => Some(Hero::Rude),
            35 => Some(Hero::Rufo),
            36 => Some(Hero::Selline),
            37 => Some(Hero::Shady),
            38 => Some(Hero::Silphir),
            39 => Some(Hero::Sist),
            40 => Some(Hero::Speaki),
            41 => Some(Hero::Sylla),
            42 => Some(Hero::Tig),
            43 => Some(Hero::Ui),
            44 => Some(Hero::Vivi),
            45 => Some(Hero::Xion),
            _ => None,
        }
    }
}

impl fmt::Display for Hero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hero::Alice => write!(f, "앨리스"),
            Hero::Amelia => write!(f, "아멜리아"),
            Hero::Ashur => write!(f, "에슈르"),
            Hero::Aya => write!(f, "야아"),
            Hero::Belita => write!(f, "벨리타"),
            Hero::Beni => write!(f, "베니"),
            Hero::BigWood => write!(f, "빅우드"),
            Hero::Butter => write!(f, "버터"),
            Hero::Canna => write!(f, "칸나"),
            Hero::Chloe => write!(f, "클로에"),
            Hero::Daya => write!(f, "다야"),
            Hero::Diana => write!(f, "디아나"),
            Hero::Elena => write!(f, "엘레나"),
            Hero::Epica => write!(f, "에피카"),
            Hero::Erpin => write!(f, "에르핀"),
            Hero::Espi => write!(f, "에스피"),
            Hero::Festa => write!(f, "페스타"),
            Hero::Fricle => write!(f, "프리클"),
            Hero::Gabia => write!(f, "가비아"),
            Hero::Hilde => write!(f, "힐데"),
            Hero::Ifrit => write!(f, "이프리트"),
            // Hero::Jade => write!(f, "제이드"),
            Hero::Jubee => write!(f, "쥬비"),
            Hero::Kidian => write!(f, "키디언"),
            Hero::Kommy => write!(f, "코미"),
            Hero::Leets => write!(f, "리츠"),
            Hero::Levi => write!(f, "레비"),
            Hero::MaestroMK2 => write!(f, "마에스트로 2호"),
            Hero::Marie => write!(f, "마리"),
            Hero::Mayo => write!(f, "마요"),
            // Hero::Meluna => write!(f, "멜루나"),
            Hero::Naia => write!(f, "나이아"),
            Hero::Ner => write!(f, "네르"),
            Hero::Posher => write!(f, "포셔"),
            Hero::Rim => write!(f, "림"),
            Hero::Rohne => write!(f, "로네"),
            Hero::Rude => write!(f, "루드"),
            Hero::Rufo => write!(f, "루포"),
            Hero::Selline => write!(f, "셀리네"),
            Hero::Shady => write!(f, "셰이디"),
            Hero::Silphir => write!(f, "실피르"),
            Hero::Sist => write!(f, "시스트"),
            Hero::Speaki => write!(f, "스피키"),
            Hero::Sylla => write!(f, "실라"),
            Hero::Tig => write!(f, "티그"),
            Hero::Ui => write!(f, "우이"),
            // Hero::Velvet => write!(f, "벨벳"),
            Hero::Vivi => write!(f, "비비"),
            Hero::Xion => write!(f, "시온"),
        }
    }
}

impl Distribution<Hero> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hero {
        Hero::new(rng.random_range(0..NUM_HEROS)).unwrap()
    }
}

pub const MAX_POINT: u16 = 9_999;
pub const MAX_PLAY_TIME: i32 = 150_000; // 150 seconds
pub const MAX_CTRL_TIME: u16 = 10_000; // 10 seconds
pub const MAX_HEALTH_COUNT: usize = 5;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Packet {
    // Server -> Client
    Connection(PlayData),
    // Client -> Server
    EnterGame,
    // Client -> Server
    TryCancelGame,
    // Server -> Client
    CancelSuccess,
    // Server -> Client
    MatchingStatus {
        millis: u16,
    },
    // Server -> Client
    MatchingSuccess {
        left: PlayData,
        right: PlayData,
    },
    // Client -> Server
    GameLoadSuccess,
    // Server -> Client
    GameLoadTimeout,
    // Server -> Client
    PrepareInGame,
    // Client -> Server
    UpdateThrowParams {
        angle: u8,
        power: u8,
    },
    // Client -> Server
    ThrowProjectile,
    // Server -> Client
    InGameLeftTurn {
        total_remaining_millis: i32,
        remaining_millis: u16,
        left_health_cnt: u8,
        right_health_cnt: u8,
        control: Option<(u8, u8)>,
    },
    // Server -> Client
    InGameRightTurn {
        total_remaining_millis: i32,
        remaining_millis: u16,
        left_health_cnt: u8,
        right_health_cnt: u8,
        control: Option<(u8, u8)>,
    },
    // Server -> Client
    InGameTurnSetup {
        wind_angle: u8,
        wind_power: u8,
    },
    // Server -> Client
    InGameProjectileThrown {
        total_remaining_millis: i32,
        remaining_millis: u16,
        left_health_cnt: u8,
        right_health_cnt: u8,
        projectile_pos: (f32, f32),
        projectile_vel: (f32, f32),
    },
    // Server -> Client
    GameResult {
        win: u16,
        lose: u16,
        victory: bool,
    },
    // Server -> Client
    GameResultDraw,
    // Client -> Server
    RankingQuery,
    // Server -> Client
    RankingResult {
        my_rank: Option<u32>,
        top_list: Vec<RankItem>,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayData {
    pub uuid: Option<Uuid>,
    pub name: String,
    pub hero: Hero,
    pub win: u16,
    pub lose: u16,
}

pub const WORLD_MIN_X: f32 = -1440.0;
pub const WORLD_MAX_X: f32 = 1440.0;
pub const WORLD_MIN_Y: f32 = -540.0;
pub const WORLD_MAX_Y: f32 = 540.0;

pub const LEFT_CAM_POS_X: f32 = -480.0;
pub const RIGHT_CAM_POS_X: f32 = 480.0;

pub const LEFT_PLAYER_POS_X: f32 = -960.0;
pub const LEFT_PLAYER_POS_Y: f32 = 340.0;

pub const RIGHT_PLAYER_POS_X: f32 = -LEFT_PLAYER_POS_X;
pub const RIGHT_PLAYER_POS_Y: f32 = LEFT_PLAYER_POS_Y;

pub const LEFT_THROW_POS_X: f32 = LEFT_PLAYER_POS_X;
pub const LEFT_THROW_POS_Y: f32 = LEFT_PLAYER_POS_Y + 96.0;

pub const LEFT_START_ANGLE: f32 = 30f32.to_radians();
pub const LEFT_END_ANGLE: f32 = 75f32.to_radians();

pub const RIGHT_THROW_POS_X: f32 = RIGHT_PLAYER_POS_X;
pub const RIGHT_THROW_POS_Y: f32 = RIGHT_PLAYER_POS_Y + 96.0;

pub const RIGHT_START_ANGLE: f32 = 105f32.to_radians();
pub const RIGHT_END_ANGLE: f32 = 150f32.to_radians();

pub const THROW_POWER: f32 = 1500.0;
pub const THROW_END_TIME: u16 = 3_000; // 3 seconds

pub const WIND_POWER: f32 = THROW_POWER * 0.125;

pub const PROJECTILE_SIZE: f32 = 64.0;
pub const GRAVITY: f32 = -9.80665 * 84.0;

lazy_static! {
    pub static ref COLLIDER_DATA: HashMap<Hero, Circle> = {
        let map = HashMap::from_iter([
            (Hero::Alice, Circle::new(40.0, (0.0, 160.0))),
            (Hero::Amelia, Circle::new(40.0, (0.0, 162.0))),
            (Hero::Ashur, Circle::new(40.0, (2.0, 156.0))),
            (Hero::Aya, Circle::new(40.0, (-4.0, 172.0))),
            (Hero::Belita, Circle::new(40.0, (-6.0, 164.0))),
            (Hero::Beni, Circle::new(40.0, (8.0, 156.0))),
            (Hero::BigWood, Circle::new(45.0, (16.0, 216.0))),
            (Hero::Butter, Circle::new(36.0, (8.0, 148.0))),
            (Hero::Canna, Circle::new(40.0, (-2.0, 156.0))),
            (Hero::Chloe, Circle::new(36.0, (-8.0, 140.0))),
            (Hero::Daya, Circle::new(34.0, (-12.0, 148.0))),
            (Hero::Diana, Circle::new(40.0, (10.0, 146.0))),
            (Hero::Elena, Circle::new(40.0, (8.0, 160.0))),
            (Hero::Epica, Circle::new(40.0, (-4.0, 146.0))),
            (Hero::Erpin, Circle::new(40.0, (0.0, 142.0))),
            (Hero::Espi, Circle::new(40.0, (0.0, 150.0))),
            (Hero::Festa, Circle::new(40.0, (4.0, 154.0))),
            (Hero::Fricle, Circle::new(36.0, (0.0, 146.0))),
            (Hero::Gabia, Circle::new(40.0, (0.0, 164.0))),
            (Hero::Hilde, Circle::new(40.0, (0.0, 160.0))),
            (Hero::Ifrit, Circle::new(40.0, (4.0, 146.0))),
            // (Hero::Jade, Circle::new(40.0, (0.0, 122.0))),
            (Hero::Jubee, Circle::new(40.0, (8.0, 136.0))),
            (Hero::Kidian, Circle::new(36.0, (0.0, 122.0))),
            (Hero::Kommy, Circle::new(40.0, (0.0, 156.0))),
            (Hero::Leets, Circle::new(40.0, (16.0, 130.0))),
            (Hero::Levi, Circle::new(40.0, (0.0, 160.0))),
            (Hero::MaestroMK2, Circle::new(40.0, (16.0, 168.0))),
            (Hero::Marie, Circle::new(40.0, (-8.0, 140.0))),
            (Hero::Mayo, Circle::new(40.0, (0.0, 158.0))),
            // (Hero::Meluna, Circle::new(40.0, (0.0, 142.0))),
            (Hero::Naia, Circle::new(40.0, (10.0, 142.0))),
            (Hero::Ner, Circle::new(40.0, (6.0, 152.0))),
            (Hero::Posher, Circle::new(40.0, (0.0, 156.0))),
            (Hero::Rim, Circle::new(36.0, (0.0, 152.0))),
            (Hero::Rohne, Circle::new(40.0, (4.0, 152.0))),
            (Hero::Rude, Circle::new(40.0, (22.0, 116.0))),
            (Hero::Rufo, Circle::new(40.0, (4.0, 156.0))),
            (Hero::Selline, Circle::new(38.0, (16.0, 146.0))),
            (Hero::Shady, Circle::new(40.0, (-4.0, 152.0))),
            (Hero::Silphir, Circle::new(40.0, (0.0, 132.0))),
            (Hero::Sist, Circle::new(44.0, (12.0, 128.0))),
            (Hero::Speaki, Circle::new(36.0, (0.0, 148.0))),
            (Hero::Sylla, Circle::new(40.0, (-4.0, 174.0))),
            (Hero::Tig, Circle::new(40.0, (-4.0, 156.0))),
            (Hero::Ui, Circle::new(36.0, (0.0, 168.0))),
            // (Hero::Velvet, Circle::new(40.0, (0.0, 146.0))),
            (Hero::Vivi, Circle::new(36.0, (-4.0, 158.0))),
            (Hero::Xion, Circle::new(36.0, (0.0, 142.0))),
        ]);
        
        assert_eq!(map.len(), NUM_HEROS);
        map
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub radius: f32,
    pub center: (f32, f32),
}

impl Circle {
    pub fn new(radius: f32, center: (f32, f32)) -> Self {
        Self { radius, center }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankItem {
    pub rank: u32,
    pub uuid: String,
    pub name: String,
    pub wins: u32,
    pub losses: u32,
}

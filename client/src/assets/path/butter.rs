use super::*;

const HERO_NAME: &str = "Butter";

pub struct HeroVoice;

impl HeroVoiceSet for HeroVoice {
    fn all(&self) -> &[&'static str] {
        &[
            VOC_PATH_ANGER_1,
            VOC_PATH_ANGER_2,
            VOC_PATH_ANGER_3,
            VOC_PATH_ANGER_4,
            VOC_PATH_ANGER_5,
            VOC_PATH_ATTACK_1,
            VOC_PATH_ATTACK_2,
            VOC_PATH_DEFEAT_1,
            VOC_PATH_DEFEAT_2,
            VOC_PATH_HIT_1,
            VOC_PATH_HIT_2,
            VOC_PATH_HIT_3,
            VOC_PATH_HIT_4,
            VOC_PATH_HIT_5,
            VOC_PATH_PULL_CHEEK,
            VOC_PATH_RUBBING_1,
            VOC_PATH_RUBBING_2,
            VOC_PATH_RUBBING_3,
            VOC_PATH_VICTORY_1,
            VOC_PATH_VICTORY_2,
        ]
    }

    fn defeat(&self) -> [&'static str; 2] {
        VOC_PATH_DEFEATS
    }

    fn victory(&self) -> [&'static str; 2] {
        VOC_PATH_VICTORIES
    }

    fn smash_end(&self) -> [&'static str; 5] {
        VOC_PATH_SMASH_ENDS
    }

    fn pull_cheek(&self) -> &'static str {
        VOC_PATH_PULL_CHEEK
    }

    fn rubbing(&self) -> [&'static str; 3] {
        VOC_PATH_RUBBINGS
    }

    fn attack(&self) -> [&'static str; 2] {
        VOC_PATH_ATTACKS
    }

    fn hit(&self) -> [&'static str; 5] {
        VOC_PATH_HITS
    }
}

#[rustfmt::skip] const VOC_PATH_ANGER_1: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_Anger1.sound", QUERY, VERSION);
#[rustfmt::skip] const VOC_PATH_ANGER_2: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_Anger2.sound", QUERY, VERSION);
#[rustfmt::skip] const VOC_PATH_ANGER_3: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_Anger3.sound", QUERY, VERSION);
#[rustfmt::skip] const VOC_PATH_ANGER_4: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_Anger4.sound", QUERY, VERSION);
#[rustfmt::skip] const VOC_PATH_ANGER_5: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_Anger5.sound", QUERY, VERSION);
const VOC_PATH_SMASH_ENDS: [&str; 5] = [
    VOC_PATH_ANGER_1,
    VOC_PATH_ANGER_2,
    VOC_PATH_ANGER_3,
    VOC_PATH_ANGER_4,
    VOC_PATH_ANGER_5,
];

#[rustfmt::skip] const VOC_PATH_ATTACK_1: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_BasicAttack1.sound", QUERY, VERSION);
#[rustfmt::skip] const VOC_PATH_ATTACK_2: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_BasicAttack2.sound", QUERY, VERSION);
const VOC_PATH_ATTACKS: [&str; 2] = [VOC_PATH_ATTACK_1, VOC_PATH_ATTACK_2];

#[rustfmt::skip] const VOC_PATH_DEFEAT_1: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_Defeat1.sound", QUERY, VERSION);
#[rustfmt::skip] const VOC_PATH_DEFEAT_2: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_Defeat2.sound", QUERY, VERSION);
const VOC_PATH_DEFEATS: [&str; 2] = [VOC_PATH_DEFEAT_1, VOC_PATH_DEFEAT_2];

#[rustfmt::skip] const VOC_PATH_HIT_1: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_Hit1.sound", QUERY, VERSION);
#[rustfmt::skip] const VOC_PATH_HIT_2: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_Hit2.sound", QUERY, VERSION);
#[rustfmt::skip] const VOC_PATH_HIT_3: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_Hit3.sound", QUERY, VERSION);
#[rustfmt::skip] const VOC_PATH_HIT_4: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_Hit4.sound", QUERY, VERSION);
#[rustfmt::skip] const VOC_PATH_HIT_5: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_Hit5.sound", QUERY, VERSION);
const VOC_PATH_HITS: [&str; 5] = [
    VOC_PATH_HIT_1,
    VOC_PATH_HIT_2,
    VOC_PATH_HIT_3,
    VOC_PATH_HIT_4,
    VOC_PATH_HIT_5,
];

#[rustfmt::skip] const VOC_PATH_PULL_CHEEK: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_Touch1.sound", QUERY, VERSION);

#[rustfmt::skip] const VOC_PATH_RUBBING_1: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_Touch2.sound", QUERY, VERSION);
#[rustfmt::skip] const VOC_PATH_RUBBING_2: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_Touch2_1.sound", QUERY, VERSION);
#[rustfmt::skip] const VOC_PATH_RUBBING_3: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_Touch2_2.sound", QUERY, VERSION);
const VOC_PATH_RUBBINGS: [&str; 3] = [VOC_PATH_RUBBING_1, VOC_PATH_RUBBING_2, VOC_PATH_RUBBING_3];

#[rustfmt::skip] const VOC_PATH_VICTORY_1: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_Victory1.sound", QUERY, VERSION);
#[rustfmt::skip] const VOC_PATH_VICTORY_2: &str = concatcp!("sounds/", HERO_NAME, "/Voice_", HERO_NAME, "_Victory2.sound", QUERY, VERSION);
const VOC_PATH_VICTORIES: [&str; 2] = [VOC_PATH_VICTORY_1, VOC_PATH_VICTORY_2];

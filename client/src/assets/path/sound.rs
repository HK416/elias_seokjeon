use super::*;

macro_rules! sound_path {
    ($module: tt, $hero: expr) => {
        pub mod $module {
            use super::*;

            pub struct HeroVoice;

            impl HeroVoiceSet for HeroVoice {
                fn all(&self) -> &[&'static str] {
                    &[
                        VOC_PATH_DEFEAT_1,
                        VOC_PATH_DEFEAT_2,
                        VOC_PATH_DUTCH_RUB_END_1,
                        VOC_PATH_DUTCH_RUB_END_2,
                        VOC_PATH_HIT_1,
                        VOC_PATH_HIT_2,
                        VOC_PATH_HIT_3,
                        VOC_PATH_HIT_4,
                        VOC_PATH_HIT_5,
                        VOC_PATH_SHOUT_1,
                        VOC_PATH_SHOUT_2,
                        VOC_PATH_SHOUT_3,
                        VOC_PATH_SHOUT_4,
                        VOC_PATH_SHOUT_5,
                        VOC_PATH_TOUCH_1,
                        VOC_PATH_TOUCH_2,
                        VOC_PATH_VICTORY_1,
                        VOC_PATH_VICTORY_2,
                    ]
                }

                fn defeat(&self) -> &[&'static str] {
                    &[
                        VOC_PATH_DEFEAT_1,
                        VOC_PATH_DEFEAT_2,
                    ]
                }

                fn ducth_rub_end(&self) -> &[&'static str] {
                    &[
                        VOC_PATH_DUTCH_RUB_END_1,
                        VOC_PATH_DUTCH_RUB_END_2,
                    ]
                }

                fn hit(&self) -> &[&'static str] {
                    &[
                        VOC_PATH_HIT_1,
                        VOC_PATH_HIT_2,
                        VOC_PATH_HIT_3,
                        VOC_PATH_HIT_4,
                        VOC_PATH_HIT_5,
                    ]
                }

                fn shout(&self) -> &[&'static str] {
                    &[
                        VOC_PATH_SHOUT_1,
                        VOC_PATH_SHOUT_2,
                        VOC_PATH_SHOUT_3,
                        VOC_PATH_SHOUT_4,
                        VOC_PATH_SHOUT_5,
                    ]
                }

                fn touch_1(&self) -> &[&'static str] {
                    &[
                        VOC_PATH_TOUCH_1,
                    ]
                }

                fn touch_2(&self) -> &[&'static str] {
                    &[
                        VOC_PATH_TOUCH_2,
                    ]
                }

                fn victory(&self) -> &[&'static str] {
                    &[
                        VOC_PATH_VICTORY_1,
                        VOC_PATH_VICTORY_2,
                    ]
                }

            }

            #[rustfmt::skip] const VOC_PATH_DEFEAT_1: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Defeat1.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_DEFEAT_2: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Defeat2.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_DUTCH_RUB_END_1: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_DutchRubEnd1.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_DUTCH_RUB_END_2: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_DutchRubEnd2.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_HIT_1: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Hit1.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_HIT_2: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Hit2.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_HIT_3: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Hit3.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_HIT_4: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Hit4.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_HIT_5: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Hit5.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_SHOUT_1: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Shout1.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_SHOUT_2: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Shout2.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_SHOUT_3: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Shout3.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_SHOUT_4: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Shout4.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_SHOUT_5: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Shout5.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_TOUCH_1: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Touch1.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_TOUCH_2: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Touch2.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_VICTORY_1: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Victory1.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_VICTORY_2: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Victory2.sound", QUERY, VERSION);
        }
    };
}

sound_path!(aya, "Aya");
sound_path!(bigwood, "BigWood");
sound_path!(butter, "Butter");
sound_path!(erpin, "Erpin");
sound_path!(kidian, "Kidian");
sound_path!(kommy, "Kommy");
sound_path!(mayo, "Mayo");
sound_path!(rohne, "Rohne");
sound_path!(speaki, "Speaki");
sound_path!(xion, "xXionx");

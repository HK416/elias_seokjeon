use super::*;

macro_rules! hero_assets {
    ($module: tt, $hero: expr) => {
        pub mod $module {
            use super::*;

            pub struct HeroVoice;

            impl HeroVoiceSet for HeroVoice {
                fn all(&self) -> &[&'static str] {
                    &[
                        VOC_PATH_CALL_PLAYER_1,
                        VOC_PATH_CALL_PLAYER_2,
                        VOC_PATH_CALL_PLAYER_3,
                        VOC_PATH_CALL_PLAYER_4,
                        VOC_PATH_CALL_PLAYER_5,
                        VOC_PATH_DEFEAT_1,
                        VOC_PATH_DEFEAT_2,
                        VOC_PATH_DUTCH_RUB_END_1,
                        VOC_PATH_DUTCH_RUB_END_2,
                        VOC_PATH_GREETING,
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

                fn call_player(&self) -> &[&'static str] {
                    &[
                        VOC_PATH_CALL_PLAYER_1,
                        VOC_PATH_CALL_PLAYER_2,
                        VOC_PATH_CALL_PLAYER_3,
                        VOC_PATH_CALL_PLAYER_4,
                        VOC_PATH_CALL_PLAYER_5,
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

                fn greeting(&self) -> &[&'static str] {
                    &[
                        VOC_PATH_GREETING
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

            #[rustfmt::skip] pub const MODEL_PATH: &str = concatcp!("models/", $hero, "/", $hero, ".model", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_CALL_PLAYER_1: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_CallPlayer1.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_CALL_PLAYER_2: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_CallPlayer2.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_CALL_PLAYER_3: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_CallPlayer3.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_CALL_PLAYER_4: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_CallPlayer4.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_CALL_PLAYER_5: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_CallPlayer5.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_DEFEAT_1: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Defeat1.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_DEFEAT_2: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Defeat2.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_DUTCH_RUB_END_1: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_DutchRubEnd1.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_DUTCH_RUB_END_2: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_DutchRubEnd2.sound", QUERY, VERSION);
            #[rustfmt::skip] const VOC_PATH_GREETING: &str = concatcp!("sounds/", $hero, "/Voice_", $hero, "_Greeting.sound", QUERY, VERSION);
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

hero_assets!(alice, "Alice");
hero_assets!(amelia, "Amelia");
hero_assets!(ashur, "Ashur");
hero_assets!(aya, "Aya");
hero_assets!(belita, "Belita");
hero_assets!(beni, "Beni");
hero_assets!(bigwood, "BigWood");
hero_assets!(butter, "Butter");
hero_assets!(canna, "Canna");
hero_assets!(chloe, "Chloe");
hero_assets!(daya, "Daya");
hero_assets!(diana, "Diana");
hero_assets!(elena, "Elena");
hero_assets!(epica, "Epica");
hero_assets!(erpin, "Erpin");
hero_assets!(espi, "Espi");
hero_assets!(festa, "Festa");
hero_assets!(fricle, "Fricle");
hero_assets!(gabia, "Gabia");
hero_assets!(hilde, "Hilde");
hero_assets!(ifrit, "Ifrit");
// hero_assets!(jade, "Jade");
hero_assets!(jubee, "Jubee");
hero_assets!(kidian, "Kidian");
hero_assets!(kommy, "Kommy");
hero_assets!(leets, "Leets");
hero_assets!(levi, "Levi");
hero_assets!(maestro_mk2, "MaestroMK2");
hero_assets!(marie, "Marie");
hero_assets!(mayo, "Mayo");
// hero_assets!(meluna, "Meluna");
hero_assets!(naia, "Naia");
hero_assets!(ner, "Ner");
hero_assets!(posher, "Posher");
hero_assets!(rim, "Rim");
hero_assets!(rohne, "Rohne");
hero_assets!(rude, "Rude");
hero_assets!(rufo, "Rufo");
hero_assets!(selline, "Selline");
hero_assets!(shady, "Shady");
hero_assets!(silphir, "Silphir");
hero_assets!(sist, "Sist");
hero_assets!(speaki, "Speaki");
hero_assets!(sylla, "Sylla");
hero_assets!(tig, "Tig");
hero_assets!(ui, "Ui");
// hero_assets!(velvet, "Velvet");
hero_assets!(vivi, "Vivi");
hero_assets!(xion, "xXionx");

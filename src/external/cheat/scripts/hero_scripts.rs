use egui::{Align2, Color32, FontId};

use crate::{external::{interfaces::enums::Hero, External}, input::keyboard::KeyState, settings::structs::Settings};
use super::HeroScript;

#[derive(Default)]
pub struct Shiv {}

impl HeroScript for Shiv {
    fn update(&mut self, _: &External, _: KeyState, _: &mut Settings) {
    }

    fn draw(&self, g: &egui::Painter, game: &External) {
        let local_player = game.get_local_player();
        let ult_ability = local_player.abilities.list.get(3);
        if ult_ability.is_none() {
            return;
        }
        let ult_ability = ult_ability.unwrap();
        if ult_ability.coodown {
            return;
        }
        let upgrade = ult_ability.points; // ульт (топор)
        let mut font = FontId::default();
        font.size = 32f32;
        for player in game.players.iter() {
            if player.rect.max.x == 0f32 {
                continue;
            }
            if player.is_alive() && player.pawn.team != local_player.pawn.team {
                let health_perc = 100f32 / player.pawn.max_health as f32 * player.pawn.health as f32;
                let can_kill: bool = if upgrade < 7 {
                    health_perc < 19.8f32 // 20%
                }
                else {
                    health_perc < 27.8f32 // 28%
                };
                if player.pawn.health <= 190 || can_kill {
                    g.text(player.rect.center(), Align2::CENTER_CENTER, "💀", font.clone(), Color32::RED);
                }
            }
        }
    }

    fn hero_id(&self) -> Hero {
        Hero::Shiv
    }
    
    fn name(&self) -> &str {
        "ult"
    }

    fn init_key_code(&self) -> Option<i32> {
        None
    }
}
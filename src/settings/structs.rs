use egui::{Align2, Color32};
use serde::{Deserialize, Serialize};

use crate::{external::interfaces::enums::TargetBone, input::keyboard::Key};

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum BoxType
{
    Default,
    Rounded,
    Edges

}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct TextSettings {
    pub enable: bool,
    pub shadow: bool,
    pub align: Align2,
    pub font_size: f32,
    pub font_color: Color32,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct GlobalSettings {
    pub key_overlay: Key,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct RadarSettings {
    pub enable: bool,
    pub(crate) rect: egui::Rect,
    pub color_background: Color32,
    pub color_border: Color32,
    pub player_radius: f32,
    pub scale: f32,
    pub color_enemy: Color32,
    pub color_team: Color32,
}

#[derive(Debug)]
#[derive(Default)]
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub global: GlobalSettings,
    pub esp_players: EspPlayers,
    pub healthbars: HealthbarSettings,
    pub radar: RadarSettings,
    pub aim: AimSettings,
    pub spectators: bool
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct AimSettings
{
    pub players: AimProperties,
    pub enable_player_fov: bool,
    pub creeps: AimProperties,
    pub enable_creep_fov: bool,
    pub angle_per_pixel: f32,
    pub creep_color: Color32,
    pub soul_color: Color32,
    pub aim_bone: TargetBone,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct AimProperties
{
    pub enable: bool,
    pub fov: f32,
    pub smooth: f32,
    pub velocity_prediction: bool,
    pub velocity_div_dav: f32,
    pub rcs: bool,
    pub range: f32,
    pub key: Key,
    pub targeting: bool,
    pub color: Color32
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct EspPlayers {
    pub render: bool,

    pub outline_rect: bool,
    pub fill_rect: bool,
    pub glow: bool,
    pub shadow: bool,

    pub outline_color: Color32,
    pub fill_color: Color32,
    pub glow_color: Color32,
    pub shadow_color: Color32,

    pub stroke_width: f32,
    pub shadow_size: f32,
    pub shadow_blur: f32,
    pub box_type: BoxType,

    pub text_hero: TextSettings,
    pub text_health: TextSettings,
    pub text_distance: TextSettings,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct HealthbarSettings
{
    pub enable: bool,
    pub background_color: Color32,
    pub outline_color: Color32,
    pub hp_color: Color32
}
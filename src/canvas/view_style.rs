use super::Color;
pub enum BackfaceVisibility {
    Visible,
    Hidden,
}
pub struct ViewStyle {
    pub backface_visibility: BackfaceVisibility,
    pub background_color: Color,
    pub opacity: f32,
}

impl Default for ViewStyle {
    fn default() -> Self {
        Self {
            backface_visibility: BackfaceVisibility::Visible,
            background_color: Color::WHITE,
            opacity: 1f32,
        }
    }
}

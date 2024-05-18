use axum_embed::ServeEmbed;
use rust_embed::Embed;

#[derive(Embed, Clone)]
#[folder = "assets/"]
pub struct Assets;

impl Assets {
    pub fn service() -> ServeEmbed<Assets> {
        ServeEmbed::<Assets>::new()
    }
}

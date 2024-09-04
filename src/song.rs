use core::fmt;

use crate::song_meta::SongMeta;
use crate::song_settings::SongSettings;

#[derive(Default)]
pub struct Song 
{
    song_meta : SongMeta,
    song_settings : SongSettings,
}

impl fmt::Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.song_meta, self.song_settings)
    }
}

impl Song {
    pub fn new() -> Self {
        Song {
            song_meta : SongMeta::default(),
            song_settings : SongSettings::default(),
        }
    }
}

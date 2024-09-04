mod song;
mod song_meta;
mod song_settings;
mod song_position;

pub mod prelude {
    pub use crate::song::Song;
    pub use crate::song_meta::SongMeta;
    pub use crate::song_settings::SongSettings;
    pub use crate::song_position::SongPosition;
}
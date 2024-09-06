mod song;
mod song_meta;
mod song_settings;
mod song_position;
mod song_chord;

pub mod prelude {
    pub use crate::song::Song;
    pub use crate::song_meta::SongMeta;
    pub use crate::song_settings::SongSettings;
    pub use crate::song_position::SongPosition;
    pub use crate::song_position::Positionable;
    pub use crate::song_chord::SongChord;
    pub use crate::song_chord::Chord;
    pub use crate::song_chord::NoteName;
    pub use crate::song_chord::NoteMod;
    pub use crate::song_chord::ChordFuntion;
    pub use crate::song_chord::ChordType;
}
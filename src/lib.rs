mod song;
mod song_meta;
mod song_settings;

pub mod prelude {
    pub use crate::song::Song;
    pub use crate::song_meta::SongMeta;
    pub use crate::song_settings::SongSettings;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}

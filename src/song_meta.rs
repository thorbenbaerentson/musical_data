use std::fmt;

#[derive(Default, Clone)]
pub struct SongMeta 
{
    title : Option<String>,
    album  : Option<String>,
    comment : Option<String>,
    songwriter : Option<String>,
    composer : Option<String>,
    arranger : Option<String>,
    copyright : Option<String>,
    artist : Option<String>,
    artist_page : Option<String>,
    genre : Option<String>,
    year : Option<i32>,
}

impl fmt::Display for SongMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Title: '{}' Album: '{}' Year: '{}' Comments: '{}' Songwriter: '{}' Composer: '{}' Arranger: '{}' Copyright: '{}' Artist: '{}' Page: '{}' Genre: '{}'", 
            self.title.clone().unwrap_or(String::from("-")), 
            self.album.clone().unwrap_or(String::from("-")),
            self.year.clone().unwrap_or(1970),
            self.comment.clone().unwrap_or(String::from("-")),
            self.songwriter.clone().unwrap_or(String::from("-")),
            self.composer.clone().unwrap_or(String::from("-")),
            self.arranger.clone().unwrap_or(String::from("-")),
            self.copyright.clone().unwrap_or(String::from("-")),
            self.artist.clone().unwrap_or(String::from("-")),
            self.artist_page.clone().unwrap_or(String::from("-")),
            self.genre.clone().unwrap_or(String::from("-")),
        
        )
    }
}

impl SongMeta 
{
    // Getter methods
    pub fn get_title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn get_album(&self) -> Option<&String> {
        self.album.as_ref()
    }

    pub fn get_comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }

    pub fn get_songwriter(&self) -> Option<&String> {
        self.songwriter.as_ref()
    }

    pub fn get_composer(&self) -> Option<&String> {
        self.composer.as_ref()
    }

    pub fn get_arranger(&self) -> Option<&String> {
        self.arranger.as_ref()
    }

    pub fn get_copyright(&self) -> Option<&String> {
        self.copyright.as_ref()
    }

    pub fn get_artist(&self) -> Option<&String> {
        self.artist.as_ref()
    }

    pub fn get_artist_page(&self) -> Option<&String> {
        self.artist_page.as_ref()
    }

    pub fn get_genre(&self) -> Option<&String> {
        self.genre.as_ref()
    }

    pub fn get_year(&self) -> Option<i32> {
        self.year
    }

    // Setter methods
    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn set_album(&mut self, album: Option<String>) {
        self.album = album;
    }

    pub fn set_comment(&mut self, comment: Option<String>) {
        self.comment = comment;
    }

    pub fn set_songwriter(&mut self, songwriter: Option<String>) {
        self.songwriter = songwriter;
    }

    pub fn set_composer(&mut self, composer: Option<String>) {
        self.composer = composer;
    }

    pub fn set_arranger(&mut self, arranger: Option<String>) {
        self.arranger = arranger;
    }

    pub fn set_copyright(&mut self, copyright: Option<String>) {
        self.copyright = copyright;
    }

    pub fn set_artist(&mut self, artist: Option<String>) {
        self.artist = artist;
    }

    pub fn set_artist_page(&mut self, artist_page: Option<String>) {
        self.artist_page = artist_page;
    }

    pub fn set_genre(&mut self, genre: Option<String>) {
        self.genre = genre;
    }

    pub fn set_year(&mut self, year: Option<i32>) {
        self.year = year;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_title() {
        let mut song_meta = SongMeta::default();
        let title = Some(String::from("Title"));
        song_meta.set_title(title.clone());
        assert_eq!(song_meta.get_title(), title.as_ref());
    }

    #[test]
    fn test_set_and_get_album() {
        let mut song_meta = SongMeta::default();
        let album = Some(String::from("Album"));
        song_meta.set_album(album.clone());
        assert_eq!(song_meta.get_album(), album.as_ref());
    }

    #[test]
    fn test_set_and_get_artist() {
        let mut song_meta = SongMeta::default();
        let artist = Some(String::from("Artist"));
        song_meta.set_artist(artist.clone());
        assert_eq!(song_meta.get_artist(), artist.as_ref());
    }

    #[test]
    fn test_set_and_get_artist_page() {
        let mut song_meta = SongMeta::default();
        let artist = Some(String::from("Artist Page"));
        song_meta.set_artist_page(artist.clone());
        assert_eq!(song_meta.get_artist_page(), artist.as_ref());
    }

    #[test]
    fn test_set_and_get_genre() {
        let mut song_meta = SongMeta::default();
        let genre = Some(String::from("Genre"));
        song_meta.set_genre(genre.clone());
        assert_eq!(song_meta.get_genre(), genre.as_ref());
    }

    #[test]
    fn test_set_and_get_comment() {
        let mut song_meta = SongMeta::default();
        let comment = Some(String::from("This is a comment"));
        song_meta.set_comment(comment.clone());
        assert_eq!(song_meta.get_comment(), comment.as_ref());
    }

    #[test]
    fn test_set_and_get_year() {
        let mut song_meta = SongMeta::default();
        let year = Some(2024);
        song_meta.set_year(year);
        assert_eq!(song_meta.get_year(), year);
    }

    #[test]
    fn test_set_and_get_composer() {
        let mut song_meta = SongMeta::default();
        let comment = Some(String::from("Composer"));
        song_meta.set_composer(comment.clone());
        assert_eq!(song_meta.get_composer(), comment.as_ref());
    }

    #[test]
    fn test_set_and_get_arranger() {
        let mut song_meta = SongMeta::default();
        let comment = Some(String::from("Arranger"));
        song_meta.set_arranger(comment.clone());
        assert_eq!(song_meta.get_arranger(), comment.as_ref());
    }

    #[test]
    fn test_set_and_get_songwriter() {
        let mut song_meta = SongMeta::default();
        let comment = Some(String::from("Songwriter"));
        song_meta.set_songwriter(comment.clone());
        assert_eq!(song_meta.get_songwriter(), comment.as_ref());
    }

    #[test]
    fn test_set_and_get_copyright() {
        let mut song_meta = SongMeta::default();
        let comment = Some(String::from("Copyright"));
        song_meta.set_copyright(comment.clone());
        assert_eq!(song_meta.get_copyright(), comment.as_ref());
    }
}
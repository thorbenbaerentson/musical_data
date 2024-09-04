use core::fmt;

#[derive(Clone)]
pub struct SongSettings
{
    ppq : u64,          // Pulses per Quarter
    sample_rate : i64,
    frame_type : i64,
    tempo : f64,
    time_signature_numerator : u64,
    time_signature_denominator : u64,
    key_signature : String,
    track_count : i32,
    length : f64,
    bit_depth : i64,
    time_format : i64,
}

impl Default for SongSettings {
    fn default() -> Self {
        Self { 
            ppq: 960, 
            sample_rate: 44100, 
            frame_type: Default::default(), 
            tempo: 120.0, 
            time_signature_numerator: 4, 
            time_signature_denominator: 4, 
            key_signature: String::from("C Major"), 
            track_count: Default::default(), 
            length: Default::default(), 
            bit_depth: 64, 
            time_format: Default::default() 
        }
    }
}

impl fmt::Display for SongSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Sample Rate: {} Frame Type: {} Tempo: {} Time Signature: {}/{} Key: {} Tracks: {} Length: {} Bit Depth: {} Time Format: {}", 
            self.sample_rate, 
            self.frame_type, 
            self.tempo, 
            self.time_signature_numerator, 
            self.time_signature_denominator, 
            self.key_signature, 
            self.track_count, 
            self.length,
            self.bit_depth,
            self.time_format
        )
    }
}

impl SongSettings {
    // Getters
    pub fn get_pulses_per_quarter(&self) -> u64 {
        self.ppq
    }

    /// Get the number of pulses per beat. We need this in order to calculate 
    /// the nummber of ticks per beat if the denominator of the time signature 
    /// in not 4. But rather 8, 16 or 2 and 1 etc.
    pub fn get_pulses_per_beat(&self) -> u64 {
        if self.get_time_signature_denominator() != 1 &&
           self.get_time_signature_denominator() % 2 != 0 {
            panic!("Invalid time signature settings!");
        }

        let divisor = 4.0 / self.time_signature_denominator as f32;

        self.ppq / (divisor.floor() as u64)
    }

    pub fn get_sample_rate(&self) -> i64 {
        self.sample_rate
    }

    pub fn get_frame_type(&self) -> i64 {
        self.frame_type
    }

    pub fn get_tempo(&self) -> f64 {
        self.tempo
    }

    pub fn get_time_signature_numerator(&self) -> u64 {
        self.time_signature_numerator
    }

    pub fn get_time_signature_denominator(&self) -> u64 {
        self.time_signature_denominator
    }

    pub fn get_key_signature(&self) -> &String {
        &self.key_signature
    }

    pub fn get_track_count(&self) -> i32 {
        self.track_count
    }

    pub fn get_length(&self) -> f64 {
        self.length
    }

    pub fn get_bit_depth(&self) -> i64 {
        self.bit_depth
    }

    pub fn get_time_format(&self) -> i64 {
        self.time_format
    }

    // Setters
    pub fn set_pulses_per_quarter(&mut self, value : u64) {
        self.ppq = value
    }

    pub fn set_sample_rate(&mut self, value: i64) {
        self.sample_rate = value;
    }

    pub fn set_frame_type(&mut self, value: i64) {
        self.frame_type = value;
    }

    pub fn set_tempo(&mut self, value: f64) {
        self.tempo = value;
    }

    pub fn set_time_signature_numerator(&mut self, value: u64) {
        self.time_signature_numerator = value;
    }

    pub fn set_time_signature_denominator(&mut self, value: u64) {
        self.time_signature_denominator = value;
    }

    pub fn set_key_signature(&mut self, value: String) {
        self.key_signature = value;
    }

    pub fn set_track_count(&mut self, value: i32) {
        self.track_count = value;
    }

    pub fn set_length(&mut self, value: f64) {
        self.length = value;
    }

    pub fn set_bit_depth(&mut self, value: i64) {
        self.bit_depth = value;
    }

    pub fn set_time_format(&mut self, value: i64) {
        self.time_format = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ppq_getter_setter() {
        let mut song_settings = SongSettings::default();
        song_settings.set_pulses_per_quarter(960 * 2);
        assert_eq!(song_settings.get_pulses_per_quarter(), 960 * 2);
    }

    #[test]
    fn test_sample_rate_getter_and_setter() {
        let mut song_settings = SongSettings::default();
        song_settings.set_sample_rate(44100);
        assert_eq!(song_settings.get_sample_rate(), 44100);
    }

    #[test]
    fn test_frame_type_getter_and_setter() {
        let mut song_settings = SongSettings::default();
        song_settings.set_frame_type(1);
        assert_eq!(song_settings.get_frame_type(), 1);
    }

    #[test]
    fn test_tempo_getter_and_setter() {
        let mut song_settings = SongSettings::default();
        song_settings.set_tempo(120.0);
        assert_eq!(song_settings.get_tempo(), 120.0);
    }

    #[test]
    fn test_time_signature_numerator_getter_and_setter() {
        let mut song_settings = SongSettings::default();
        song_settings.set_time_signature_numerator(4);
        assert_eq!(song_settings.get_time_signature_numerator(), 4);
    }

    #[test]
    fn test_time_signature_denominator_getter_and_setter() {
        let mut song_settings = SongSettings::default();
        song_settings.set_time_signature_denominator(4);
        assert_eq!(song_settings.get_time_signature_denominator(), 4);
    }

    #[test]
    fn test_key_signature_getter_and_setter() {
        let mut song_settings = SongSettings::default();
        let key_signature = String::from("G Major");
        song_settings.set_key_signature(key_signature.clone());
        assert_eq!(song_settings.get_key_signature(), &key_signature);
    }

    #[test]
    fn test_track_count_getter_and_setter() {
        let mut song_settings = SongSettings::default();
        song_settings.set_track_count(8);
        assert_eq!(song_settings.get_track_count(), 8);
    }

    #[test]
    fn test_length_getter_and_setter() {
        let mut song_settings = SongSettings::default();
        song_settings.set_length(180.5);
        assert_eq!(song_settings.get_length(), 180.5);
    }

    #[test]
    fn test_bit_depth_getter_and_setter() {
        let mut song_settings = SongSettings::default();
        song_settings.set_bit_depth(16);
        assert_eq!(song_settings.get_bit_depth(), 16);
    }

    #[test]
    fn test_time_format_getter_and_setter() {
        let mut song_settings = SongSettings::default();
        song_settings.set_time_format(1);
        assert_eq!(song_settings.get_time_format(), 1);
    }

    #[test]
    fn test_default() {
        let subject = SongSettings::default();
        assert_eq!(*subject.get_key_signature(), String::from("C Major"));
        assert_eq!(subject.get_bit_depth(), 64);
        assert_eq!(subject.get_sample_rate(), 44100);
        assert_eq!(subject.get_time_signature_denominator(), 4);
        assert_eq!(subject.get_time_signature_numerator(), 4);
        assert_eq!(subject.get_tempo(), 120.0);
        assert_eq!(subject.get_pulses_per_quarter(), 960);
    }
}
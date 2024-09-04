use crate::prelude::SongSettings;

/// Stores a song position in midi ticks and provides methods to translate ticks into
/// hours, minutes and seconds and milliseconds or bar, beat, note and click etc.
pub struct SongPosition {
    ticks_on : u64,
    ticks_off : Option<u64>,
}

impl SongPosition {
    pub fn new(ticks : u64) -> Self {
        SongPosition{
            ticks_on : ticks,
            ticks_off : None,
        }
    }

    pub fn from(ticks_on : u64, ticks_off : u64) -> Self {
        SongPosition {
            ticks_on,
            ticks_off : Some(ticks_off),
        }
    }

    pub fn get_as_bars_and_beats(&self, settings : &SongSettings) -> (u64, u64, u64, u64) {
        let beats =  self.ticks_on / settings.get_pulses_per_beat();
        let bar  : u64 = beats / settings.get_time_signature_numerator();
        let beat : u64 = beats % settings.get_time_signature_numerator();
        let note : u64 = 0;
        let tick : u64 = 0;

        println!("Quarters: {}", beats);


        (bar, beat, note, tick)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::SongSettings;
    use test_case::test_case;
    use super::*;

    #[test_case(960, 4, 4, 0, 1, 0, 0; "")]
    fn test_ppq_to_bars_and_beats(ticks : u64, numerator : u64, denomiator : u64,  bar : u64, beat : u64, note : u64, tick  : u64) {
        let mut song_settings = SongSettings::default();
        song_settings.set_time_signature_numerator(numerator);
        song_settings.set_time_signature_denominator(denomiator);
        let pos = SongPosition::new(ticks);
        
        let (bar2, beat2, note2, tick2) = pos.get_as_bars_and_beats(&song_settings);

        assert_eq!(bar, bar2);
        assert_eq!(beat, beat2);
        assert_eq!(note, note2);
        assert_eq!(tick, tick2);
    }
}
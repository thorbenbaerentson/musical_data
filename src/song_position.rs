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

    pub fn get_length(&self) -> u64 {
        if self.ticks_off.is_none() {
            return 0;
        }

        return self.ticks_off.unwrap() - self.ticks_on;
    }

    pub fn get_as_bars_and_beats(&self, settings : &SongSettings) -> (u64, u64, f64) {
        let beats =  self.ticks_on / settings.get_pulses_per_beat();
        let bar  : u64 = beats / settings.get_time_signature_numerator();
        let beat : u64 = beats % settings.get_time_signature_numerator();

        let bar_to_ticks = bar * settings.get_time_signature_numerator() * settings.get_pulses_per_beat();
        let mut note : f64 = self.ticks_on as f64 - (bar_to_ticks + beat * settings.get_pulses_per_beat()) as f64;
        note = note / settings.get_pulses_per_beat() as f64;

        // println!("Bar: {} Quarter: {} Note: {} (Bar as ticks: {}) {}/{} (PPB: {})", 
        //     bar, 
        //     beat, 
        //     note, 
        //     bar_to_ticks, 
        //     settings.get_time_signature_numerator(), 
        //     settings.get_time_signature_denominator(),
        //     settings.get_pulses_per_beat()
        // );

        (bar, beat, note)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::SongSettings;
    use test_case::test_case;
    use super::*;

    #[test_case(960, 4, 4, 0, 1, 0.0)]
    #[test_case(480, 4, 4, 0, 0, 0.5)]
    #[test_case(720, 4, 4, 0, 0, 0.75)]
    #[test_case(960, 6, 8, 0, 2, 0.0)]
    #[test_case(960, 1, 2, 0, 0, 0.5)]
    #[test_case(2880, 3, 4, 1, 0, 0.0)]
    #[test_case(3360, 3, 4, 1, 0, 0.5)]
    #[test_case(3360, 4, 4, 0, 3, 0.5)]
    #[test_case(3600, 4, 4, 0, 3, 0.75)]
    #[test_case(3600, 5, 4, 0, 3, 0.75)]
    #[test_case(4800, 2, 4, 2, 1, 0.0)]
    #[test_case(5040, 2, 4, 2, 1, 0.25)]
    fn test_ppq_to_bars_and_beats(ticks : u64, numerator : u64, denomiator : u64,  bar : u64, beat : u64, note : f64) {
        let mut song_settings = SongSettings::default();
        song_settings.set_time_signature_numerator(numerator);
        song_settings.set_time_signature_denominator(denomiator);
        let pos = SongPosition::new(ticks);
        
        let (bar2, beat2, note2) = pos.get_as_bars_and_beats(&song_settings);

        assert_eq!(bar, bar2);
        assert_eq!(beat, beat2);
        assert_eq!(note, note2);
    }
}
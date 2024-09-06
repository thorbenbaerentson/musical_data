use std::fmt;
use crate::prelude::{Positionable, SongPosition};

#[derive(Default, Debug, Clone, Copy)]
pub enum NoteMod {
    Flat,
    #[default]
    Normal,
    Sharp
}

#[derive(Debug, Clone, Copy)]
pub enum NoteName {
    C(NoteMod),
    D(NoteMod),
    E(NoteMod),
    F(NoteMod),
    G(NoteMod),
    A(NoteMod),
    B(NoteMod),
}

impl Default for NoteName {
    fn default() -> Self {
        NoteName::C(NoteMod::Normal)
    }
}

impl fmt::Display for NoteName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl NoteName {
    pub fn to_string(&self) -> &str {
        match self {
            NoteName::C(m) => {
                match m {
                    NoteMod::Flat => "Cb",
                    NoteMod::Normal => "C",
                    NoteMod::Sharp => "C#",
                }
            },
            NoteName::D(m) => {
                match m {
                    NoteMod::Flat => "Db",
                    NoteMod::Normal => "D",
                    NoteMod::Sharp => "D#",
                }
            },
            NoteName::E(m) => {
                match m {
                    NoteMod::Flat => "Eb",
                    NoteMod::Normal => "E",
                    NoteMod::Sharp => "E#",
                }
            },
            NoteName::F(m) => {
                match m {
                    NoteMod::Flat => "Fb",
                    NoteMod::Normal => "F",
                    NoteMod::Sharp => "F#",
                }
            },
            NoteName::G(m) => {
                match m {
                    NoteMod::Flat => "Gb",
                    NoteMod::Normal => "G",
                    NoteMod::Sharp => "G#",
                }
            },
            NoteName::A(m) => {
                match m {
                    NoteMod::Flat => "Ab",
                    NoteMod::Normal => "A",
                    NoteMod::Sharp => "A#",
                }
            },
            NoteName::B(m) => {
                match m {
                    NoteMod::Flat => "Bb",
                    NoteMod::Normal => "B",
                    NoteMod::Sharp => "B#",
                }
            },
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum ChordType {
    #[default]
    Major,
    Minor,
    Diminished,
    Sus(u64)
}

#[derive(Debug, Clone, Copy)]
pub enum ChordFuntion {
    Root(NoteMod),
    Second(NoteMod),
    Fourth(NoteMod),
    Fifth(NoteMod),
    Sixth(NoteMod),
    Seventh(NoteMod),
    Nineth(NoteMod),
    Eleventh(NoteMod),
    Thirteenth(NoteMod)
}

impl Default for ChordFuntion {
    fn default() -> Self {
        ChordFuntion::Root(NoteMod::Normal)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Chord {
    root : NoteName,
    base : Option<NoteName>,
    interval_1 : Option<ChordFuntion>,
    interval_2 : Option<ChordFuntion>,
    interval_3 : Option<ChordFuntion>,
    interval_4 : Option<ChordFuntion>,
    interval_5 : Option<ChordFuntion>,
    interval_6 : Option<ChordFuntion>,
}

pub struct SongChord {
    pos : SongPosition,
    chord : Chord,
}

impl Positionable for SongChord {
    fn get_position(&self) -> &SongPosition {
        &self.pos
    }
}

impl SongChord {
    pub fn new(tick : u64, chord : Chord) -> Self {
        SongChord {
            pos : SongPosition::new(tick),
            chord
        }
    }
}
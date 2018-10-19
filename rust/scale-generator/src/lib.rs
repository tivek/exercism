// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug, Clone)]
pub enum Error {
    BadNote,
    BadInterval,
}

#[derive(Debug, Clone)]
pub struct Scale<'a> {
    intervals: &'a str,
    notes: Vec<&'static str>,
}

const CHROMATIC_SHARPS: [&'static str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

const CHROMATIC_FLATS: [&'static str; 12] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

impl<'a> Scale<'a> {
    pub fn new(tonic: &str, intervals: &'a str) -> Result<Scale<'a>, Error> {
        let all_notes = match tonic {
            "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "g#" | "d#" | "C"
            | "a" => CHROMATIC_SHARPS,
            "F" | "Bb" | "Eb" | "Ab" | "Db" | "Gb" | "d" | "g" | "c" | "f" | "bb" | "eb" => {
                CHROMATIC_FLATS
            }
            _ => return Err(Error::BadNote),
        };
        let mut all_notes = all_notes.iter().cycle();

        let &first = all_notes
            .by_ref()
            .take(12)
            .find(|&&n: &&&str| n.to_ascii_uppercase() == tonic.to_ascii_uppercase())
            .unwrap();

        let mut notes: Vec<&str> = Vec::new();
        notes.push(first);
        for interval in intervals.chars() {
            match interval {
                'm' => {}
                'M' => {
                    all_notes.next();
                }
                'A' => {
                    all_notes.next();
                    all_notes.next();
                }
                _ => return Err(Error::BadInterval),
            };
            let &note = all_notes.next().unwrap();
            if note == first {
                break;
            }
            notes.push(note);
        }
        Ok(Scale {
            intervals: intervals,
            notes: notes,
        })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Scale::new(tonic, "mmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        return self.notes.iter().map(|s| s.to_string()).collect();
    }
}

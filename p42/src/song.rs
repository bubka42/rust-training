extern crate std;

use std::io::Write;

static DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

static GIFTS: [&str; 12] = [
    " partridge in a pear tree",
    "Two turtle doves,",
    "Three French hens,",
    "Four calling birds,",
    "Five golden rings,",
    "Six geese a-laying,",
    "Seven swans a-swimming,",
    "Eight maids a-milking,",
    "Nine ladies dancing,",
    "Ten lords a-leaping,",
    "Eleven pipers piping,",
    "Twelve drummers drumming,",
];

static LEFT: &str = "On the ";
static RIGHT: &str = " day of Christmas,";
static SECOND: &str = "my true love gave to me";
static V1: &str = "A";
static V2: &str = "And a";

pub struct SongIter {
    pub day: usize,
    pub index: usize,
}

impl Default for SongIter {
    fn default() -> Self {
        SongIter { day: 1, index: 0 }
    }
}

impl SongIter {
    /// get the current line
    /// ```
    /// use p42::song::SongIter;
    ///
    /// let new_iter = SongIter { day: 1, index: 0 };
    /// let s = new_iter.get_line();
    /// assert_eq!(s.as_str(), "On the first day of Christmas,");
    /// ```
    pub fn get_line(&self) -> String {
        match self.index {
            0 => {
                let mut first = String::from(LEFT);
                first.push_str(DAYS[self.day - 1]);
                first.push_str(RIGHT);
                first
            }
            1 => String::from(SECOND),
            i => {
                if i == self.day + 2 {
                    String::from("")
                } else if i == self.day + 1 {
                    let mut v = match self.day {
                        1 => String::from(V1),
                        _ => String::from(V2),
                    };
                    v.push_str(GIFTS[0]);
                    match self.day {
                        12 => v.push('!'),
                        _ => v.push('.'),
                    };
                    v
                } else {
                    String::from(GIFTS[self.day - i + 1])
                }
            }
        }
    }

    /// go to next state of iterator
    pub fn increment(&mut self) {
        self.index += 1;
        if self.index > self.day + 2 {
            self.day += 1;
            self.index = 0;
        }
    }
}

impl Iterator for SongIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.day > 12 || self.index > 13 {
            None
        } else {
            let mut line = self.get_line();
            if self.day < 12 || self.index < 13 {
                line.push('\n');
            }
            self.increment();
            Some(line)
        }
    }
}

/// add line number to each line in lyrics
pub fn iter_with_line_num() -> impl Iterator<Item = String> {
    let song_iter: SongIter = SongIter::default();
    song_iter
        .enumerate()
        .map(|(num, line)| (num + 1).to_string() + ": " + &line)
}

/// duplicate each line in lyrics
pub fn duplicate_lines(
    iter: impl Iterator<Item = String>,
    num: usize,
) -> impl Iterator<Item = String> {
    iter.flat_map(move |line| {
        let mut numlines = Vec::new();
        for _ in 0..num {
            numlines.push(line.clone());
        }
        numlines
    })
}

/// extract lyrics from song iterator
/// ```
/// use p42::song::SongIter;
/// use p42::song::song_to_string;
/// use p42::song::iter_with_line_num;
///
/// assert_eq!(song_to_string(SongIter::default()), include_str!("lyrics_for_test.txt").replace("\r", ""));
/// ```
pub fn song_to_string(mut iter: impl Iterator<Item = String>) -> String {
    let mut lyrics = iter.next().unwrap();
    for line in iter {
        lyrics.push_str(line.as_str());
    }
    lyrics
}

/// extract lyrics from song iterator into file using song_to_iter
pub fn song_to_file_1(iter: impl Iterator<Item = String>, path: &String) -> std::io::Result<()> {
    std::fs::write(path, song_to_string(iter))?;
    Ok(())
}

/// extract lyrics from song iterator into file without using song_to_iter
pub fn song_to_file_2(iter: impl Iterator<Item = String>, path: &String) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    for line in iter {
        file.write_all(line.as_bytes())?;
    }
    Ok(())
}

// send lyrics from iterator to TCP connection
pub fn song_to_tcp(mut iter: impl Iterator<Item = String>, addr: &str) -> std::io::Result<()> {
    let mut stream = std::net::TcpStream::connect(addr)?;
    stream.write_all(iter.next().unwrap().as_bytes())?;
    for line in iter {
        stream.write_all(line.as_bytes())?;
    }
    Ok(())
}

// listen to TCP connection for song lyrics
pub fn song_from_tcp(port: u16) -> std::io::Result<()> {
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let listener = std::net::TcpListener::bind(addr)?;
    let mut stdout = std::io::stdout().lock();
    std::io::copy(&mut listener.accept()?.0, &mut stdout)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::SongIter;

    #[test]
    fn test_get_line() {
        let new_iter = super::SongIter { day: 1, index: 2 };
        let s = new_iter.get_line();
        assert_eq!(s.as_str(), "A partridge in a pear tree.");
        let new_iter = super::SongIter { day: 3, index: 4 };
        let s = new_iter.get_line();
        assert_eq!(s.as_str(), "And a partridge in a pear tree.");
        let new_iter = super::SongIter { day: 6, index: 8 };
        let s = new_iter.get_line();
        assert_eq!(s.as_str(), "");
        let new_iter = super::SongIter { day: 8, index: 1 };
        let s = new_iter.get_line();
        assert_eq!(s.as_str(), "my true love gave to me");
        let new_iter = super::SongIter { day: 10, index: 4 };
        let s = new_iter.get_line();
        assert_eq!(s.as_str(), "Eight maids a-milking,");
    }

    #[test]
    fn test_iter() {
        let mut new_iter = super::SongIter { day: 1, index: 0 };
        assert_eq!(
            new_iter.next().unwrap().as_str(),
            "On the first day of Christmas,\n"
        );
        assert_eq!(
            new_iter.next().unwrap().as_str(),
            "my true love gave to me\n"
        );
        assert_eq!(
            new_iter.next().unwrap().as_str(),
            "A partridge in a pear tree.\n"
        );
        assert_eq!(new_iter.next().unwrap().as_str(), "\n");
    }

    #[test]
    fn test_iter_with_line_num() {
        let mut new_iter_num = super::iter_with_line_num();
        assert_eq!(
            new_iter_num.next().unwrap().as_str(),
            "1: On the first day of Christmas,\n"
        );
        assert_eq!(
            new_iter_num.next().unwrap().as_str(),
            "2: my true love gave to me\n"
        );
        assert_eq!(
            new_iter_num.next().unwrap().as_str(),
            "3: A partridge in a pear tree.\n"
        );
        assert_eq!(new_iter_num.next().unwrap().as_str(), "4: \n");
    }

    #[test]
    fn test_duplicate_lines() {
        let mut duplicate_iter = super::duplicate_lines(SongIter::default(), 2);
        assert_eq!(
            duplicate_iter.next().unwrap().as_str(),
            "On the first day of Christmas,\n"
        );
        assert_eq!(
            duplicate_iter.next().unwrap().as_str(),
            "On the first day of Christmas,\n"
        );
        assert_eq!(
            duplicate_iter.next().unwrap().as_str(),
            "my true love gave to me\n"
        );
        assert_eq!(
            duplicate_iter.next().unwrap().as_str(),
            "my true love gave to me\n"
        );
        assert_eq!(
            duplicate_iter.next().unwrap().as_str(),
            "A partridge in a pear tree.\n"
        );
        assert_eq!(
            duplicate_iter.next().unwrap().as_str(),
            "A partridge in a pear tree.\n"
        );
        assert_eq!(duplicate_iter.next().unwrap().as_str(), "\n");
        assert_eq!(duplicate_iter.next().unwrap().as_str(), "\n");
    }
}

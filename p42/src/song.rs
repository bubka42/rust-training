extern crate std;

static DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

static GIFTS: [&str; 12] = [
    " partridge in a pear tree.",
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
static SECOND: &str = "My true love gave to me";
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
    /// let new_iter = SongIter { day: 1, index: 2 };
    /// let s = new_iter.get_line();
    /// assert_eq!(s.as_str(), "A partridge in a pear tree.");
    /// let new_iter = SongIter { day: 3, index: 4 };
    /// let s = new_iter.get_line();
    /// assert_eq!(s.as_str(), "And a partridge in a pear tree.");
    /// let new_iter = SongIter { day: 6, index: 8 };
    /// let s = new_iter.get_line();
    /// assert_eq!(s.as_str(), "");
    /// let new_iter = SongIter { day: 8, index: 1 };
    /// let s = new_iter.get_line();
    /// assert_eq!(s.as_str(), "My true love gave to me");
    /// let new_iter = SongIter { day: 10, index: 4 };
    /// let s = new_iter.get_line();
    /// assert_eq!(s.as_str(), "Eight maids a-milking,");
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
            let line = self.get_line();
            self.increment();
            Some(line)
        }
    }
}

#[test]
fn test_iter() {
    let mut new_iter = SongIter { day: 1, index: 0 };
    assert_eq!(
        new_iter.next().unwrap().as_str(),
        "On the first day of Christmas,"
    );
    assert_eq!(new_iter.next().unwrap().as_str(), "My true love gave to me");
    assert_eq!(
        new_iter.next().unwrap().as_str(),
        "A partridge in a pear tree."
    );
    assert_eq!(new_iter.next().unwrap().as_str(), "");
}

/// add line number to each line in lyrics
pub fn iter_with_line_num() -> impl Iterator<Item = String> {
    let song_iter: SongIter = SongIter::default();
    song_iter
        .enumerate()
        .map(|(num, line)| (num + 1).to_string() + ": " + &line)
}

#[test]
fn test_iter_with_line_num() {
    let mut new_iter_num = iter_with_line_num();
    assert_eq!(
        new_iter_num.next().unwrap().as_str(),
        "1: On the first day of Christmas,"
    );
    assert_eq!(
        new_iter_num.next().unwrap().as_str(),
        "2: My true love gave to me"
    );
    assert_eq!(
        new_iter_num.next().unwrap().as_str(),
        "3: A partridge in a pear tree."
    );
    assert_eq!(new_iter_num.next().unwrap().as_str(), "4: ");
}

/// extract lyrics from song iterator
/// ```
/// use p42::song::SongIter;
/// use p42::song::song_to_string;
/// use p42::song::iter_with_line_num;
///
/// assert_eq!(song_to_string(iter_with_line_num()), "1: On the first day of Christmas,\n2: My true love gave to me\n3: A partridge in a pear tree.\n4: \n5: On the second day of Christmas,\n6: My true love gave to me\n7: Two turtle doves,\n8: And a partridge in a pear tree.\n9: \n10: On the third day of Christmas,\n11: My true love gave to me\n12: Three French hens,\n13: Two turtle doves,\n14: And a partridge in a pear tree.\n15: \n16: On the fourth day of Christmas,\n17: My true love gave to me\n18: Four calling birds,\n19: Three French hens,\n20: Two turtle doves,\n21: And a partridge in a pear tree.\n22: \n23: On the fifth day of Christmas,\n24: My true love gave to me\n25: Five golden rings,\n26: Four calling birds,\n27: Three French hens,\n28: Two turtle doves,\n29: And a partridge in a pear tree.\n30: \n31: On the sixth day of Christmas,\n32: My true love gave to me\n33: Six geese a-laying,\n34: Five golden rings,\n35: Four calling birds,\n36: Three French hens,\n37: Two turtle doves,\n38: And a partridge in a pear tree.\n39: \n40: On the seventh day of Christmas,\n41: My true love gave to me\n42: Seven swans a-swimming,\n43: Six geese a-laying,\n44: Five golden rings,\n45: Four calling birds,\n46: Three French hens,\n47: Two turtle doves,\n48: And a partridge in a pear tree.\n49: \n50: On the eighth day of Christmas,\n51: My true love gave to me\n52: Eight maids a-milking,\n53: Seven swans a-swimming,\n54: Six geese a-laying,\n55: Five golden rings,\n56: Four calling birds,\n57: Three French hens,\n58: Two turtle doves,\n59: And a partridge in a pear tree.\n60: \n61: On the ninth day of Christmas,\n62: My true love gave to me\n63: Nine ladies dancing,\n64: Eight maids a-milking,\n65: Seven swans a-swimming,\n66: Six geese a-laying,\n67: Five golden rings,\n68: Four calling birds,\n69: Three French hens,\n70: Two turtle doves,\n71: And a partridge in a pear tree.\n72: \n73: On the tenth day of Christmas,\n74: My true love gave to me\n75: Ten lords a-leaping,\n76: Nine ladies dancing,\n77: Eight maids a-milking,\n78: Seven swans a-swimming,\n79: Six geese a-laying,\n80: Five golden rings,\n81: Four calling birds,\n82: Three French hens,\n83: Two turtle doves,\n84: And a partridge in a pear tree.\n85: \n86: On the eleventh day of Christmas,\n87: My true love gave to me\n88: Eleven pipers piping,\n89: Ten lords a-leaping,\n90: Nine ladies dancing,\n91: Eight maids a-milking,\n92: Seven swans a-swimming,\n93: Six geese a-laying,\n94: Five golden rings,\n95: Four calling birds,\n96: Three French hens,\n97: Two turtle doves,\n98: And a partridge in a pear tree.\n99: \n100: On the twelfth day of Christmas,\n101: My true love gave to me\n102: Twelve drummers drumming,\n103: Eleven pipers piping,\n104: Ten lords a-leaping,\n105: Nine ladies dancing,\n106: Eight maids a-milking,\n107: Seven swans a-swimming,\n108: Six geese a-laying,\n109: Five golden rings,\n110: Four calling birds,\n111: Three French hens,\n112: Two turtle doves,\n113: And a partridge in a pear tree.");
/// ```
pub fn song_to_string(mut iter: impl Iterator<Item = String>) -> String {
    let mut lyrics = iter.next().unwrap();
    for line in iter {
        lyrics.push('\n');
        lyrics.push_str(line.as_str());
    }
    lyrics
}

/// extract lyrics from song iterator into file
pub fn song_to_file(iter: impl Iterator<Item = String>, path: &String) -> std::io::Result<()> {
    std::fs::write(path, song_to_string(iter))?;
    Ok(())
}

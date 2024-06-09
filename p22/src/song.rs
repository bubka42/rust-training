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

pub fn get_line(day: usize, index: usize) -> String {
    match index {
        0 => {
            let mut first = String::from(LEFT);
            first.push_str(DAYS[day - 1]);
            first.push_str(RIGHT);
            first
        }
        1 => String::from(SECOND),
        i => {
            if i == day + 2 {
                String::new()
            } else if i == day + 1 {
                let mut v = match day {
                    1 => String::from(V1),
                    _ => String::from(V2),
                };
                v.push_str(GIFTS[0]);
                match day {
                    12 => v.push('!'),
                    _ => v.push('.'),
                };
                v
            } else {
                String::from(GIFTS[day - i + 1])
            }
        }
    }
}

pub fn print_lyrics() {
    for day in 1..13 {
        for index in 0..day + 3 {
            println!("{}", get_line(day, index));
        }
    }
}

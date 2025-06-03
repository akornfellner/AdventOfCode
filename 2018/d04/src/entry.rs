use std::cmp::Ordering;
use std::fmt;

#[derive(Debug)]
pub struct Entry {
    pub year: usize,
    pub month: usize,
    pub day: usize,
    pub hour: usize,
    pub minute: usize,
    pub entry_type: EntryType,
}

impl Entry {
    pub fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let date = parts[0].trim_matches('[').split('-').collect::<Vec<&str>>();
        let time = parts[1].trim_matches(']').split(':').collect::<Vec<&str>>();
        let year = date[0].parse::<usize>().unwrap();
        let month = date[1].parse::<usize>().unwrap();
        let day = date[2].parse::<usize>().unwrap();
        let hour = time[0].parse::<usize>().unwrap();
        let minute = time[1].parse::<usize>().unwrap();
        let entry_type = match parts[2] {
            "Guard" => EntryType::Begin(parts[3].trim_matches('#').parse::<usize>().unwrap()),
            "falls" => EntryType::FallAsleep,
            "wakes" => EntryType::WakeUp,
            _ => panic!("Unknown entry type"),
        };

        Entry {
            year,
            month,
            day,
            hour,
            minute,
            entry_type,
        }
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.hour == other.hour
            && self.minute == other.minute
    }
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year
            .cmp(&other.year)
            .then(self.month.cmp(&other.month))
            .then(self.day.cmp(&other.day))
            .then(self.hour.cmp(&other.hour))
            .then(self.minute.cmp(&other.minute))
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entry_type_str = match &self.entry_type {
            EntryType::Begin(id) => format!("Begin({})", id),
            EntryType::FallAsleep => "FallAsleep".to_string(),
            EntryType::WakeUp => "WakeUp".to_string(),
        };
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}-{:02} {}",
            self.year, self.month, self.day, self.hour, self.minute, entry_type_str
        )
    }
}

#[derive(Debug)]
pub enum EntryType {
    Begin(usize),
    FallAsleep,
    WakeUp,
}

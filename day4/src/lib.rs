use std::str::FromStr;


pub struct Range {
    pub start: i64,
    pub end: i64,
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split("-");
        let (start, end) = (s.next().ok_or("No start found")?, s.next().ok_or("No end found")?);
        Ok(Self { 
            start: start.parse().map_err(|_| "Failed to parse start int")?, 
            end: end.parse().map_err(|_| "Failed to parse end int")?, 
        })
    }
}

pub struct Pair {
    pub a: Range,
    pub b: Range,
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(",");
        let (a, b) = (s.next().ok_or("No first range")?, s.next().ok_or("No second range")?);

        Ok(Self { a: a.parse()?, b: b.parse()? })
    }
}

impl Pair {
    pub fn is_fully_overlapped(&self) -> bool {
        (self.a.start <= self.b.start && self.a.end >= self.b.end) ||
        (self.b.start <= self.a.start && self.b.end >= self.a.end)
    }

    pub fn is_any_overlapped(&self) -> bool {
        (self.a.end >= self.b.start) &&
        (self.a.start <= self.b.end)
    }
}

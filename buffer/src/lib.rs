#[macro_use] extern crate serde_derive;

use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Eq, Serialize, Deserialize)]
pub struct Point {
    r: usize,
    c: usize,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.r == other.r && self.c == other.c
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.r > other.r {
            Ordering::Greater
        } else if self.r == other.r {
            self.c.cmp(&other.c)
        } else {
            Ordering::Less
        }
    }
}

impl Point {
    pub fn new(r: usize, c: usize) -> Point {
        Point {
            r: r,
            c: c
        }
    }
}

pub struct Buffer {
    pub lines: Vec<Vec<char>>,
    pub text_len: usize,
    pub point: Point
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Line {
    pub line: String,
    pub number: usize
}

#[derive(PartialEq, Debug)]
pub enum BufErr {
    InvalidPoint,
    InvalidStartPoint,
    InvalidEndPoint,
    InvalidDeletionLength
}

impl fmt::Display for BufErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &BufErr::InvalidPoint => { write!(f, "invalid point") }
            &BufErr::InvalidStartPoint => { write!(f, "invalid start point") }
            &BufErr::InvalidEndPoint => { write!(f, "invalid end point") }
            &BufErr::InvalidDeletionLength => { write!(f, "invalid deletion length") }
        }
    }
}

type BufResult<T> = Result<T, BufErr>;

fn push_multiple<T>(v: &mut Vec<T>, mut offset: usize, s: &[T]) where T: Clone + Default {
    if s.len() == 0 {
        return;
    }
    if v.len() == 0 {
        v.extend_from_slice(s);
        return;
    }
    assert!(offset <= v.len());
    let pad = s.len() - ((v.len() - offset) % s.len());
    v.extend(std::iter::repeat(Default::default()).take(pad));
    v.extend_from_slice(s);
    let total = v.len();
    while total - offset >= s.len() {
        for i in 0..s.len() {
            v.swap(offset + i, total - s.len() + i);
        }
        offset += s.len();
    }
    v.truncate(total - pad);
}

pub trait IntoLine {
    fn into_line(self, number: usize) -> Line;
}

impl IntoLine for String {
    fn into_line(self, number: usize) -> Line {
        Line {
            number: number,
            line: self
        }
    }
}

impl<'a> IntoLine for &'a str {
    fn into_line(self, number: usize) -> Line {
        Line {
            number: number,
            line: self.to_string()
        }
    }
}

impl Line {
    fn new(number: usize, line: String) -> Line {
        Line {
            number: number,
            line: line
        }
    }
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            lines: vec![],
            text_len: 0,
            point: Point { r: 0, c: 0 }
        }
    }

    pub fn with_contents(string: &str) -> Buffer {
        let mut buf = Buffer::new();
        let lines = string.split("\n")
            .map(|string| string.to_string().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        buf.text_len = string.len();
        if string.ends_with("\n") {
            buf.lines = lines[..lines.len() - 1].to_vec();
        } else {
            // No terminating newline in string, but we add it when we
            // insert it into the buffer, so we add 1 to text_len
            buf.lines = lines;
            buf.text_len += 1
        }
        buf
    }

    pub fn insert_at_pt(&mut self, string: &str, pt: &Point) -> BufResult<Vec<Line>> {
        let row = pt.r;
        let col = pt.c;
        if row > self.lines.len() ||
            (self.lines.len() > 0 && row < self.lines.len() && col > self.lines[row].len()) ||
            row == self.lines.len() && col != 0 {
            return Err(BufErr::InvalidPoint);
        }
        let lines = string.split("\n")
            .map(|string| string.to_string().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // When we return the modified lines, we need to be careful not to run
        // past the end of self.lines if the inserted string ends in a newline
        // and the insertion point is at past the end of the buffer (i.e.,
        // creating a new line at the end of the buffer)
        let n_lines = if lines[lines.len() - 1].len() == 0 && row == self.lines.len() {
            lines.len() - 1
        } else {
            lines.len()
        };
        if row == self.lines.len() {
            self.lines.extend_from_slice(&lines[..lines.len() - 1]);
            if lines[lines.len() - 1].len() != 0 {
                self.lines.push(lines[lines.len() - 1].clone());
                // No terminating newline in string, but we add it when we
                // insert it into the buffer, so we add 1 to text_len
                self.text_len += 1;
            }
        } else {
            // Save characters after insertion point on the same line
            let rest_of_line = self.lines[row].drain(col..).collect::<Vec<_>>();
            // Add first line of inserted string at the insertion point
            self.lines[row].extend(lines[0].iter());
            if lines.len() > 1 {
                // Prepend last line of inserted string to the rest of the first
                // line
                let mut last_line = lines[lines.len() - 1].clone();
                last_line.extend(rest_of_line);
                // Insert prepended last line
                self.lines.insert(row + 1, last_line);
                // Insert rest of the lines in the inserted string in reverse
                // order
                push_multiple(&mut self.lines, row + 1, &lines[1..lines.len() - 1]);
            } else {
                self.lines[row].extend(rest_of_line);
            }
        }
        self.text_len += string.len();

        let mut modified_lines = vec![];
        for (i, line) in self.lines[row..row + n_lines].iter().enumerate() {
            modified_lines.push(Line::new(row + i, line.iter().cloned().collect()));
        }
        Ok(modified_lines)
    }

    pub fn region_to_str(&self, start: &Point, end: &Point) -> BufResult<String> {
        if start.r > self.lines.len() ||
            (self.lines.len() > 0 && start.r < self.lines.len() &&
                start.c > self.lines[start.r].len()) ||
            start.r == self.lines.len() && start.c != 0 {
            return Err(BufErr::InvalidStartPoint);
        }
        if end.r > self.lines.len() ||
            (self.lines.len() > 0 && end.r < self.lines.len() &&
                end.c > self.lines[end.r].len()) ||
            end.r == self.lines.len() && end.c != 0 {
            return Err(BufErr::InvalidEndPoint);
        }
        if end < start {
            return Err(BufErr::InvalidDeletionLength)
        }
        if end == start {
            return Ok("".to_string());
        }

        let mut string = String::from("");
        if start.r == end.r {
            return Ok(self.lines[start.r][start.c..end.c]
                .iter()
                .cloned()
                .collect::<String>());
        }
        string += &(self.lines[start.r].iter().cloned().collect::<String>() + "\n");
        for line in &self.lines[start.r + 1..end.r] {
            string += &(line.iter().cloned().collect::<String>() + "\n");
        }
        if end.c != 0 {
            string += &self.lines[end.r][..end.c]
                .iter()
                .cloned()
                .collect::<String>();
        }
        Ok(string)
    }

    pub fn delete_region(&mut self, start: &Point, end: &Point) -> BufResult<String> {
        if start.r > self.lines.len() ||
            (self.lines.len() > 0 && start.r < self.lines.len() &&
                start.c > self.lines[start.r].len()) ||
            start.r == self.lines.len() && start.c != 0 {
            return Err(BufErr::InvalidStartPoint);
        }
        if end.r > self.lines.len() ||
            (self.lines.len() > 0 && end.r < self.lines.len() &&
                end.c > self.lines[end.r].len()) ||
            end.r == self.lines.len() && end.c != 0 {
            return Err(BufErr::InvalidEndPoint);
        }
        if end < start {
            return Err(BufErr::InvalidDeletionLength)
        }
        if end == start {
            return Ok("".to_string());
        }

        let mut string = String::from("");
        if start.r == end.r {
            return Ok(self.lines[start.r].drain(start.c..end.c).collect::<String>());
        }
        string += &(self.lines[start.r].drain(start.c..).collect::<String>() + "\n");
        for line in &self.lines[start.r + 1..end.r] {
            string += &(line.iter().cloned().collect::<String>() + "\n");
        }
        let mut last_part = String::from("");
        if end.c != 0 {
            last_part += &self.lines[end.r]
                .drain(..end.c)
                .collect::<String>();
        }
        string += &last_part;
        self.lines.drain(start.r + 1..end.r);

        if start.r < self.lines.len() - 1 {
            // Combine with rest of last line in deleted region
            let rest_of_last_line = self.lines[start.r + 1].clone();
            self.lines[start.r].extend(rest_of_last_line);
            self.lines.remove(start.r + 1);
        }

        self.text_len -= string.len();
        if self.text_len == 0 {
            self.text_len = 1;
        }
        Ok(string)
    }

    pub fn to_str(&mut self) -> String {
        self.region_to_str(&Point::new(0, 0), &Point::new(self.lines.len(), 0)).unwrap()
    }
}

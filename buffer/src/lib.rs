use std::cmp::Ordering;

#[derive(Debug, Eq)]
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

#[derive(PartialEq, Debug)]
pub enum BufErr {
    InvalidPoint,
    InvalidDeletionLength
}

type BufResult = Result<(), BufErr>;
type BufResultString = Result<String, BufErr>;

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

    pub fn insert_at_pt(&mut self, string: &str, pt: &Point) -> BufResult {
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
        if row == self.lines.len() {
            for line in lines.iter().take(lines.len() - 1) {
                self.lines.push(line.clone());
            }
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
                for line in lines[1..lines.len() - 1].iter().rev() {
                    self.lines.insert(row + 1, line.clone());
                }
            } else {
                self.lines[row].extend(rest_of_line);
            }
        }
        self.text_len += string.len();
        Ok(())
    }

    pub fn region_to_str(&self, start: &Point, end: &Point) -> BufResultString {
        if start.r > self.lines.len() ||
            (self.lines.len() > 0 && start.r < self.lines.len() &&
                start.c > self.lines[start.r].len()) ||
            start.r == self.lines.len() && start.c != 0 {
            return Err(BufErr::InvalidPoint);
        }
        if end.r > self.lines.len() ||
            (self.lines.len() > 0 && end.r < self.lines.len() &&
                end.c > self.lines[end.r].len()) ||
            end.r == self.lines.len() && end.c != 0 {
            return Err(BufErr::InvalidPoint);
        }
        if end < start {
            return Err(BufErr::InvalidPoint)
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

    pub fn delete_region(&mut self, start: &Point, end: &Point) -> BufResultString {
        if start.r > self.lines.len() ||
            (self.lines.len() > 0 && start.r < self.lines.len() &&
                start.c > self.lines[start.r].len()) ||
            start.r == self.lines.len() && start.c != 0 {
            return Err(BufErr::InvalidPoint);
        }
        if end.r > self.lines.len() ||
            (self.lines.len() > 0 && end.r < self.lines.len() &&
                end.c > self.lines[end.r].len()) ||
            end.r == self.lines.len() && end.c != 0 {
            return Err(BufErr::InvalidPoint);
        }
        if end < start {
            return Err(BufErr::InvalidPoint)
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

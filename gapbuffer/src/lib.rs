const INTIAL_BUFFER_LENGTH: usize = 1024;

pub struct GapBuffer {
    pub buf: Vec<char>,

    // text_len is the sum of the lengths of the first and last halves
    pub text_len: usize,

    // In buffer space
    pub gap_start_idx: usize,

    // In buffer space. gap_end_idx is actually one after the actual gap end
    // index. This makes most calculations easier since most of the time we just
    // want to figure out the length of the gap.
    pub gap_end_idx: usize,

    // User insertion point. In text space.
    pub point_idx: usize,

    // Tuples are (start, length). In text space.
    pub lines: Vec<(usize, usize)>
}

#[derive(PartialEq, Debug)]
pub enum GbErr {
    InvalidPoint,
    InvalidDeletionLength
}

type GbResult = Result<(), GbErr>;

// Returns the index of all '\n' characters in string
fn find_all_newline_idxs(string: &str) -> Vec<usize> {
    let mut idxs: Vec<usize> = vec![];
    let mut cur_start_idx: usize = 0;
    loop {
        let cur_str = &string[cur_start_idx..];
        match cur_str.find("\n") {
            Some(idx) => {
                idxs.push(idx);
                cur_start_idx = idx + 1;
            }
            None => {
                return idxs;
            }
        }
    }
}

impl GapBuffer {
    pub fn new() -> GapBuffer {
        GapBuffer {
            text_len: 0,
            buf: vec!['\0'; INTIAL_BUFFER_LENGTH],
            gap_start_idx: 0,
            gap_end_idx: INTIAL_BUFFER_LENGTH,
            point_idx: 0,
            lines: vec![],
        }
    }

    pub fn with_capacity(capacity: usize) -> GapBuffer {
        GapBuffer {
            text_len: 0,
            buf: vec!['\0'; capacity],
            gap_start_idx: 0,
            gap_end_idx: capacity,
            point_idx: 0,
            lines: vec![],
        }
    }

    pub fn convert_pt_txt_to_buf_space(&self, point: usize) -> usize {
        if point < self.gap_start_idx {
            point
        } else {
            point + self.gap_end_idx - self.gap_start_idx
        }
    }

    /// Moves the gap so that it starts at point. point must be a valid index in
    /// text space.
    fn move_gap(&mut self, point: usize) {
        if self.gap_start_idx == self.gap_end_idx {
            // If there is no gap, just update the indices
            self.gap_start_idx = point;
            self.gap_end_idx = point;
            return;
        }
        if point < self.gap_start_idx {
            // Move chars on and after point to last half
            let mut moved_chars_copy = vec!['\0'; self.gap_start_idx - point];
            moved_chars_copy.copy_from_slice(&self.buf[point..self.gap_start_idx]);
            let moved_chars_slice = &mut self.buf[self.gap_end_idx - moved_chars_copy.len()..self.gap_end_idx];
            moved_chars_slice.copy_from_slice(&moved_chars_copy);
            self.gap_start_idx -= moved_chars_copy.len();
            self.gap_end_idx -= moved_chars_copy.len();
        } else if point > self.gap_start_idx {
            // Move chars before and on point to first half
            let mut moved_chars_copy = vec!['\0'; point - self.gap_start_idx];
            let moved_chars_copy_len = moved_chars_copy.len();
            moved_chars_copy.copy_from_slice(&self.buf[self.gap_end_idx..self.gap_end_idx + moved_chars_copy_len]);
            let moved_chars_slice = &mut self.buf[self.gap_start_idx..self.gap_start_idx + moved_chars_copy.len()];
            moved_chars_slice.copy_from_slice(&moved_chars_copy);
            self.gap_start_idx += moved_chars_copy.len();
            self.gap_end_idx += moved_chars_copy.len();
        }
        assert_eq!(point, self.gap_start_idx);
    }

    /// Inserts the string at the point. point must be a valid index in text
    /// space. If not, a GbErr will be returned. If the point is not the gap
    /// start index, we move the gap by copying all characters in the current
    /// half to the other half so that the new gap start index is at point (see
    /// move_gap()).
    ///
    /// Inserting at point p results in the first char of string to be located
    /// at p, and the point will be updated to p + string.len() (as will the gap
    /// start index).
    ///
    /// If we insert a string that is longer than the gap length, the buffer
    /// will be doubled successively in size until the string can fit.
    ///
    /// If we insert a string that is exactly the same length as the gap, the
    /// gap start and end indexes will be updated to buf.len().
    pub fn insert_at_pt(&mut self, string: &str, point: usize) -> GbResult {
        if point > self.text_len {
            return Err(GbErr::InvalidPoint);
        }
        let mut gap_len = self.gap_end_idx - self.gap_start_idx;
        if string.len() > gap_len {
            // Increase size of buffer
            let old_buf_len = self.buf.len();
            if self.gap_end_idx != self.buf.len() {
                // Last half exists
                // Copy last half into last_half_copy
                let mut last_half_copy = vec!['\0'; self.buf.len() - self.gap_end_idx];
                {
                    let last_half_slice = &self.buf[self.gap_end_idx..old_buf_len];
                    last_half_copy.copy_from_slice(last_half_slice);
                }
                while gap_len < string.len() {
                    let cur_buf_len = self.buf.len();
                    self.buf.resize(cur_buf_len * 2, '\0');
                    self.gap_end_idx += cur_buf_len;
                    gap_len += cur_buf_len;
                }
                let new_buf_len = self.buf.len();
                let last_half_slice = &mut self.buf[self.gap_end_idx..new_buf_len];
                last_half_slice.copy_from_slice(&last_half_copy);
            } else {
                // Last half does not exist
                while gap_len < string.len() {
                    let cur_buf_len = self.buf.len();
                    self.buf.resize(cur_buf_len * 2, '\0');
                    self.gap_end_idx += cur_buf_len;
                    gap_len += cur_buf_len;
                }
            }
        }
        self.move_gap(point);
        let string_chars = string.to_string().chars().collect::<Vec<char>>();
        {
            let copy_slice = &mut self.buf[self.gap_start_idx..self.gap_start_idx
                + string_chars.len()];
            copy_slice.copy_from_slice(&string_chars);
        }
        self.gap_start_idx += string_chars.len();
        self.text_len += string_chars.len();

        // Update lines
        // Assume that the inserted string did not contain a newline
        let line_idx;
        match self.lines.binary_search_by_key(&point, |&(a, _)| a) {
            Ok(idx) => {
                line_idx = idx;
                for line_tup in &mut self.lines[idx..] {
                    *line_tup = (line_tup.0 + string.len(), line_tup.1);
                }
            }
            Err(idx) => {
                line_idx = idx;
                for line_tup in &mut self.lines[idx + 1..] {
                    *line_tup = (line_tup.0 + string.len(), line_tup.1);
                }
            }
        }
        if (self.lines.len() == 0) {
            self.lines.push((0, string.len()));
        } else {
            self.lines[line_idx] = (self.lines[line_idx].0, self.lines[line_idx].1 + string.len());
        }
        let rel_newline_idxs = find_all_newline_idxs(string);
        let mut next_line_idx = line_idx + 1;
        for nli in &rel_newline_idxs {

        }

        Ok(())
    }

    /// Deletes length chars starting from point. point must be a valid index in
    /// text space, and point + length must not exceed the text length. If
    /// either of these conditions are violated, a GbErr will be returned.
    /// Deletion is performed by moving the gap to point, then adding length to
    /// the gap end index.
    pub fn delete_at_pt(&mut self, point: usize, length: usize) -> GbResult {
        if point > self.text_len {
            return Err(GbErr::InvalidPoint);
        }
        if point + length > self.text_len {
            return Err(GbErr::InvalidDeletionLength);
        }

        self.move_gap(point);
        self.gap_end_idx += length;
        self.text_len -= length;

        if self.text_len < self.buf.len() / 4 {
            let new_len = self.buf.len() / 4;
            let mut last_half_copy = vec!['\0'; self.buf.len() - self.gap_end_idx];
            last_half_copy.copy_from_slice(&self.buf[self.gap_end_idx..]);
            self.buf.resize(new_len, '\0');
            &mut self.buf[new_len - last_half_copy.len()..].copy_from_slice(&last_half_copy);
            self.gap_end_idx = self.buf.len() - last_half_copy.len();
        }
        Ok(())
    }
}

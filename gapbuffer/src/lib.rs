const INTIAL_BUFFER_LENGTH: usize = 1024;

pub struct GapBuffer {
    pub text_len: usize,
    pub buf: Vec<char>,
    pub gap_start_idx: usize,
    pub gap_end_idx: usize,
    pub point_idx: usize,
}

#[derive(PartialEq, Debug)]
pub enum GbErr {
    InvalidPoint,
    InvalidDeletionLength
}

type GbResult = Result<(), GbErr>;

impl GapBuffer {
    pub fn new() -> GapBuffer {
        GapBuffer {
            text_len: 0,
            buf: vec!['\0'; INTIAL_BUFFER_LENGTH],
            gap_start_idx: 0,
            gap_end_idx: INTIAL_BUFFER_LENGTH,
            point_idx: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> GapBuffer {
        GapBuffer {
            text_len: 0,
            buf: vec!['\0'; capacity],
            gap_start_idx: 0,
            gap_end_idx: capacity,
            point_idx: 0,
        }
    }

    /// Moves the gap so that it starts at point. point must be valid (in the
    /// range [0, buf.len()]).
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

    /// Inserts the string at the point. point must be valid (in the range [0,
    /// buf.len()]). If not, a GbErr will be returned. If the point is not the
    /// gap start index, we move the gap by copying all characters in the
    /// current half to the other half so that the new gap start index is at
    /// point (see move_gap()).
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
        Ok(())
    }

    /// Deletes length chars starting from point. point must be valid (in the
    /// range [0, buf.len()]). If not, a GbErr will be returned. Deletion is
    /// performed by moving the gap to point, then adding length to the gap end
    /// index.
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

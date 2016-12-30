extern crate gapbuffer;
use gapbuffer::{GapBuffer, GbErr};

#[test]
/// Simple insertion into empty gap buffer
fn test_insert_at_pt_simple1() {
    let mut gb = GapBuffer::with_capacity(10);
    assert!(gb.insert_at_pt("abc", 0).is_ok());
    assert_eq!(gb.gap_start_idx, 3);
    assert_eq!(gb.gap_end_idx, 10);
    assert_eq!(gb.text_len, 3);

    let s = (&gb.buf[..gb.gap_start_idx]).iter().cloned().collect::<String>();
    assert_eq!(s, "abc");
}

#[test]
/// Insertion into start of gap buffer with existing text
fn test_insert_at_pt_simple2() {
    let mut gb = GapBuffer::with_capacity(10);
    let chars = "abc".chars().collect::<Vec<char>>();
    &mut gb.buf[..chars.len()].copy_from_slice(&chars[..]);
    gb.gap_start_idx += chars.len();
    gb.text_len = 3;

    assert!(gb.insert_at_pt("de", 0).is_ok());
    assert_eq!(gb.gap_start_idx, 2);
    assert_eq!(gb.gap_end_idx, 7);
    assert_eq!(gb.text_len, 5);

    let mut s = (&gb.buf[..gb.gap_start_idx]).iter().cloned().collect::<String>();
    assert_eq!(s, "de");
    s = (&gb.buf[gb.gap_end_idx..]).iter().cloned().collect::<String>();
    assert_eq!(s, "abc");
}

#[test]
/// Insertion into middle of first half of gap buffer with existing text
fn test_insert_at_pt_simple3() {
    let mut gb = GapBuffer::with_capacity(10);
    let chars = "abc".chars().collect::<Vec<char>>();
    &mut gb.buf[..chars.len()].copy_from_slice(&chars[..]);
    gb.gap_start_idx += chars.len();
    gb.text_len = 3;

    assert!(gb.insert_at_pt("de", 1).is_ok());
    assert_eq!(gb.gap_start_idx, 3);
    assert_eq!(gb.gap_end_idx, 8);
    assert_eq!(gb.text_len, 5);

    let mut s = (&gb.buf[..gb.gap_start_idx]).iter().cloned().collect::<String>();
    assert_eq!(s, "ade");
    s = (&gb.buf[gb.gap_end_idx..]).iter().cloned().collect::<String>();
    assert_eq!(s, "bc");
}

#[test]
/// Insertion into beginning of gap
fn test_insert_at_pt_simple4() {
    let mut gb = GapBuffer::with_capacity(10);
    let chars = "abc".chars().collect::<Vec<char>>();
    &mut gb.buf[..chars.len()].copy_from_slice(&chars[..]);
    gb.gap_start_idx += chars.len();
    gb.text_len = 3;

    assert!(gb.insert_at_pt("de", 3).is_ok());
    assert_eq!(gb.gap_start_idx, 5);
    assert_eq!(gb.gap_end_idx, 10);
    assert_eq!(gb.text_len, 5);

    let mut s = (&gb.buf[..gb.gap_start_idx]).iter().cloned().collect::<String>();
    assert_eq!(s, "abcde");
    s = (&gb.buf[gb.gap_end_idx..]).iter().cloned().collect::<String>();
    assert_eq!(s, "");
}

#[test]
/// Insertion into last half of gap
fn test_insert_at_pt_simple5() {
    let mut gb = GapBuffer::with_capacity(10);
    let mut chars = "abc".chars().collect::<Vec<char>>();
    &mut gb.buf[..chars.len()].copy_from_slice(&chars[..]);
    gb.gap_start_idx += chars.len();

    chars = "de".chars().collect::<Vec<char>>();
    let buf_len = gb.buf.len();
    &mut gb.buf[buf_len - chars.len()..].copy_from_slice(&chars[..]);
    gb.gap_end_idx -= chars.len();
    gb.text_len = 5;

    assert!(gb.insert_at_pt("fg", 4).is_ok());
    // First half is "abcdfg", second half is "e"
    assert_eq!(gb.gap_start_idx, 6);
    assert_eq!(gb.gap_end_idx, 9);
    assert_eq!(gb.text_len, 7);

    let mut s = (&gb.buf[..gb.gap_start_idx]).iter().cloned().collect::<String>();
    assert_eq!(s, "abcdfg");
    s = (&gb.buf[gb.gap_end_idx..]).iter().cloned().collect::<String>();
    assert_eq!(s, "e");
}

#[test]
/// Complete displacement of last half
fn test_insert_at_pt_simple6() {
    let mut gb = GapBuffer::with_capacity(10);
    let mut chars = "abc".chars().collect::<Vec<char>>();
    &mut gb.buf[..chars.len()].copy_from_slice(&chars[..]);
    gb.gap_start_idx += chars.len();

    chars = "de".chars().collect::<Vec<char>>();
    let buf_len = gb.buf.len();
    &mut gb.buf[buf_len - chars.len()..].copy_from_slice(&chars[..]);
    gb.gap_end_idx -= chars.len();
    gb.text_len = 5;

    assert!(gb.insert_at_pt("fg", 5).is_ok());
    // First half is "abcdfg", second half is "e"
    assert_eq!(gb.gap_start_idx, 7);
    assert_eq!(gb.gap_end_idx, 10);
    assert_eq!(gb.text_len, 7);

    let mut s = (&gb.buf[..gb.gap_start_idx]).iter().cloned().collect::<String>();
    assert_eq!(s, "abcdefg");
    s = (&gb.buf[gb.gap_end_idx..]).iter().cloned().collect::<String>();
    assert_eq!(s, "");
}

#[test]
/// Add until full
fn test_insert_at_pt_simple7() {
    let mut gb = GapBuffer::with_capacity(10);
    let chars = "abc".chars().collect::<Vec<char>>();
    &mut gb.buf[..chars.len()].copy_from_slice(&chars[..]);
    gb.gap_start_idx += chars.len();
    gb.text_len = 3;

    assert!(gb.insert_at_pt("0123456", 0).is_ok());
    assert_eq!(gb.buf.len(), 10);
    assert_eq!(gb.gap_start_idx, 7);
    assert_eq!(gb.gap_end_idx, 7);
    assert_eq!(gb.text_len, 10);

    let mut s = (&gb.buf[..gb.gap_start_idx]).iter().cloned().collect::<String>();
    assert_eq!(s, "0123456");
    s = (&gb.buf[gb.gap_end_idx..]).iter().cloned().collect::<String>();
    assert_eq!(s, "abc");
}

#[test]
/// Resize once
fn test_insert_at_pt_resize1() {
    let mut gb = GapBuffer::with_capacity(10);
    let mut chars = "abc".chars().collect::<Vec<char>>();
    &mut gb.buf[..chars.len()].copy_from_slice(&chars[..]);
    gb.gap_start_idx += chars.len();

    chars = "de".chars().collect::<Vec<char>>();
    let buf_len = gb.buf.len();
    &gb.buf[buf_len - chars.len()..].copy_from_slice(&chars[..]);
    gb.gap_end_idx -= chars.len();
    gb.text_len = 5;

    assert!(gb.insert_at_pt("01234567", 0).is_ok());
    assert_eq!(gb.buf.len(), 20);
    assert_eq!(gb.gap_start_idx, 8);
    assert_eq!(gb.gap_end_idx, 15);
    assert_eq!(gb.text_len, 13);

    let mut s = (&gb.buf[..gb.gap_start_idx]).iter().cloned().collect::<String>();
    assert_eq!(s, "01234567");

    s = (&gb.buf[gb.gap_end_idx..]).iter().cloned().collect::<String>();
    assert_eq!(s, "abcde");
}

#[test]
/// Resize twice when full
fn test_insert_at_pt_resize2() {
    let mut gb = GapBuffer::with_capacity(5);
    let chars = "abcde".chars().collect::<Vec<char>>();
    &gb.buf[..chars.len()].copy_from_slice(&chars[..]);
    gb.gap_start_idx = chars.len();
    gb.gap_end_idx = chars.len();
    gb.text_len = 5;

    assert!(gb.insert_at_pt("01234567", 0).is_ok());
    assert_eq!(gb.buf.len(), 20);
    assert_eq!(gb.gap_start_idx, 8);
    assert_eq!(gb.gap_end_idx, 15);
    assert_eq!(gb.text_len, 13);

    let mut s = (&gb.buf[..gb.gap_start_idx]).iter().cloned().collect::<String>();
    assert_eq!(s, "01234567");

    s = (&gb.buf[gb.gap_end_idx..]).iter().cloned().collect::<String>();
    assert_eq!(s, "abcde");
}

#[test]
/// Errors on invalid point
fn test_insert_at_pt_invalid_pt1() {
    let mut gb = GapBuffer::with_capacity(5);
    let result = gb.insert_at_pt("a", 1);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), GbErr::InvalidPoint);
}

#[test]
/// Simple deletion
fn test_delete_at_pt_simple1() {
    let chars = "abcde".chars().collect::<Vec<char>>();
    let mut gb = GapBuffer {
        text_len: chars.len(),
        buf: vec!['\0'; 10],
        gap_start_idx: chars.len(),
        gap_end_idx: 10,
        point_idx: 0
    };

    &mut gb.buf[..chars.len()].copy_from_slice(&chars);

    let result = gb.delete_at_pt(1, 2);
    assert!(result.is_ok());
    assert_eq!(gb.text_len, 3);
    assert_eq!(gb.gap_start_idx, 1);
    assert_eq!(gb.gap_end_idx, 8);

    let mut s = (&gb.buf[..gb.gap_start_idx]).iter().cloned().collect::<String>();
    assert_eq!(s, "a");

    s = (&gb.buf[gb.gap_end_idx..]).iter().cloned().collect::<String>();
    assert_eq!(s, "de");
}

#[test]
/// Simple deletion across gap
fn test_delete_at_pt_simple2() {
    let first_chars = "ab".chars().collect::<Vec<char>>();
    let last_chars = "cde".chars().collect::<Vec<char>>();
    let mut gb = GapBuffer {
        text_len: first_chars.len() + last_chars.len(),
        buf: vec!['\0'; 10],
        gap_start_idx: first_chars.len(),
        gap_end_idx: 10 - last_chars.len(),
        point_idx: 0
    };

    &mut gb.buf[..first_chars.len()].copy_from_slice(&first_chars);
    &mut gb.buf[10 - last_chars.len()..].copy_from_slice(&last_chars);

    let result = gb.delete_at_pt(1, 3);
    assert!(result.is_ok());
    assert_eq!(gb.text_len, 2);
    assert_eq!(gb.gap_start_idx, 1);
    assert_eq!(gb.gap_end_idx, 9);

    let mut s = (&gb.buf[..gb.gap_start_idx]).iter().cloned().collect::<String>();
    assert_eq!(s, "a");

    s = (&gb.buf[gb.gap_end_idx..]).iter().cloned().collect::<String>();
    assert_eq!(s, "e");
}

#[test]
/// Resizing
fn test_delete_at_pt_resize1() {
    let first_chars = "ab".chars().collect::<Vec<char>>();
    let last_chars = "cde".chars().collect::<Vec<char>>();
    let initial_buf_len = 20;
    let mut gb = GapBuffer {
        text_len: first_chars.len() + last_chars.len(),
        buf: vec!['\0'; initial_buf_len],
        gap_start_idx: first_chars.len(),
        gap_end_idx: initial_buf_len - last_chars.len(),
        point_idx: 0
    };

    &mut gb.buf[..first_chars.len()].copy_from_slice(&first_chars);
    &mut gb.buf[initial_buf_len - last_chars.len()..].copy_from_slice(&last_chars);

    let result = gb.delete_at_pt(1, 3);
    // New length is 5
    assert!(result.is_ok());
    assert_eq!(gb.text_len, 2);
    assert_eq!(gb.gap_start_idx, 1);
    assert_eq!(gb.gap_end_idx, 4);

    let mut s = (&gb.buf[..gb.gap_start_idx]).iter().cloned().collect::<String>();
    assert_eq!(s, "a");

    s = (&gb.buf[gb.gap_end_idx..]).iter().cloned().collect::<String>();
    assert_eq!(s, "e");
}

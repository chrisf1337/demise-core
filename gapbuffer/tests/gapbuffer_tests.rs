extern crate gapbuffer;
use gapbuffer::GapBuffer;

#[test]
/// Simple insertion into empty gap buffer
fn test_insert_at_pt_simple1() {
    let mut gb = GapBuffer::with_capacity(10);
    gb.insert_at_pt("abc", 0);
    assert_eq!(gb.gap_start_idx, 3);
    assert_eq!(gb.gap_end_idx, 10);
    assert_eq!(gb.point_idx, 3);
    assert_eq!(gb.text_len, 3);

    let s = (&gb.buf[..gb.gap_start_idx]).iter().cloned().collect::<String>();
    assert_eq!(s, "abc");
}

#[test]
/// Insertion into start of gap buffer with existing text
fn test_insert_at_pt_simple2() {
    let mut gb = GapBuffer::with_capacity(10);
    let chars = "abc".chars().collect::<Vec<char>>();
    &gb.buf[..chars.len()].copy_from_slice(&chars[..]);
    gb.gap_start_idx += chars.len();
    gb.text_len = 3;

    gb.insert_at_pt("de", 0);
    assert_eq!(gb.gap_start_idx, 2);
    assert_eq!(gb.gap_end_idx, 7);
    assert_eq!(gb.point_idx, 2);
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
    &gb.buf[..chars.len()].copy_from_slice(&chars[..]);
    gb.gap_start_idx += chars.len();
    gb.text_len = 3;

    gb.insert_at_pt("de", 1);
    assert_eq!(gb.gap_start_idx, 3);
    assert_eq!(gb.gap_end_idx, 8);
    assert_eq!(gb.point_idx, 3);
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
    &gb.buf[..chars.len()].copy_from_slice(&chars[..]);
    gb.gap_start_idx += chars.len();
    gb.text_len = 3;

    gb.insert_at_pt("de", 3);
    assert_eq!(gb.gap_start_idx, 5);
    assert_eq!(gb.gap_end_idx, 10);
    assert_eq!(gb.point_idx, 5);
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
    &gb.buf[..chars.len()].copy_from_slice(&chars[..]);
    gb.gap_start_idx += chars.len();

    chars = "de".chars().collect::<Vec<char>>();
    let buf_len = gb.buf.len();
    &gb.buf[buf_len - chars.len()..].copy_from_slice(&chars[..]);
    gb.gap_end_idx -= chars.len();
    gb.text_len = 5;

    gb.insert_at_pt("fg", 4);
    // First half is "abcdfg", second half is "e"
    assert_eq!(gb.gap_start_idx, 6);
    assert_eq!(gb.gap_end_idx, 9);
    assert_eq!(gb.point_idx, 6);
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
    &gb.buf[..chars.len()].copy_from_slice(&chars[..]);
    gb.gap_start_idx += chars.len();

    chars = "de".chars().collect::<Vec<char>>();
    let buf_len = gb.buf.len();
    &gb.buf[buf_len - chars.len()..].copy_from_slice(&chars[..]);
    gb.gap_end_idx -= chars.len();
    gb.text_len = 5;

    gb.insert_at_pt("fg", 5);
    // First half is "abcdfg", second half is "e"
    assert_eq!(gb.gap_start_idx, 7);
    assert_eq!(gb.gap_end_idx, 10);
    assert_eq!(gb.point_idx, 7);
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
    &gb.buf[..chars.len()].copy_from_slice(&chars[..]);
    gb.gap_start_idx += chars.len();
    gb.text_len = 3;

    gb.insert_at_pt("0123456", 0);
    assert_eq!(gb.buf.len(), 10);
    assert_eq!(gb.gap_start_idx, 10);
    assert_eq!(gb.gap_end_idx, 10);
    assert_eq!(gb.point_idx, 7);
    assert_eq!(gb.text_len, 10);

    let s = (&gb.buf[..gb.gap_start_idx]).iter().cloned().collect::<String>();
    assert_eq!(s, "0123456abc");
}

#[test]
/// Resize once
fn test_insert_at_pt_resize1() {
    let mut gb = GapBuffer::with_capacity(10);
    let mut chars = "abc".chars().collect::<Vec<char>>();
    &gb.buf[..chars.len()].copy_from_slice(&chars[..]);
    gb.gap_start_idx += chars.len();

    chars = "de".chars().collect::<Vec<char>>();
    let buf_len = gb.buf.len();
    &gb.buf[buf_len - chars.len()..].copy_from_slice(&chars[..]);
    gb.gap_end_idx -= chars.len();
    gb.text_len = 5;

    gb.insert_at_pt("01234567", 0);
    assert_eq!(gb.buf.len(), 20);
    assert_eq!(gb.gap_start_idx, 8);
    assert_eq!(gb.gap_end_idx, 15);
    assert_eq!(gb.point_idx, 8);
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

    gb.insert_at_pt("01234567", 0);
    assert_eq!(gb.buf.len(), 20);
    assert_eq!(gb.gap_start_idx, 8);
    assert_eq!(gb.gap_end_idx, 15);
    assert_eq!(gb.point_idx, 8);
    assert_eq!(gb.text_len, 13);

    let mut s = (&gb.buf[..gb.gap_start_idx]).iter().cloned().collect::<String>();
    assert_eq!(s, "01234567");

    s = (&gb.buf[gb.gap_end_idx..]).iter().cloned().collect::<String>();
    assert_eq!(s, "abcde");
}

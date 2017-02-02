extern crate buffer;
use buffer::{Buffer, Point, BufErr};

#[test]
fn test_insert_empty_buffer1() {
    let mut buf = Buffer::new();
    assert!(buf.insert_at_pt("abc\n", &Point::new(0, 0)).is_ok());
    assert_eq!(buf.text_len, 4);
    assert_eq!(buf.lines.len(), 1);
    assert_eq!(buf.region_to_str(&Point::new(0, 0), &Point::new(0, 1)).unwrap(), "a");
    assert_eq!(buf.region_to_str(&Point::new(0, 0), &Point::new(0, 3)).unwrap(), "abc");
    assert_eq!(buf.region_to_str(&Point::new(0, 0), &Point::new(1, 0)).unwrap(), "abc\n");
    assert_eq!(buf.region_to_str(&Point::new(1, 0), &Point::new(1, 1)).unwrap_err(),
        BufErr::InvalidEndPoint);
    assert_eq!(buf.region_to_str(&Point::new(0, 0), &Point::new(2, 0)).unwrap_err(),
        BufErr::InvalidEndPoint);
}

#[test]
fn test_insert_empty_buffer2() {
    let mut buf = Buffer::new();
    assert!(buf.insert_at_pt("abc\nde", &Point::new(0, 0)).is_ok());
    assert_eq!(buf.text_len, 7);
    assert_eq!(buf.lines.len(), 2);
    assert_eq!(buf.region_to_str(&Point::new(0, 0), &Point::new(1, 0)).unwrap(), "abc\n");
    assert_eq!(buf.region_to_str(&Point::new(0, 0), &Point::new(1, 1)).unwrap(), "abc\nd");
    assert_eq!(buf.region_to_str(&Point::new(0, 0), &Point::new(1, 2)).unwrap(), "abc\nde");
    assert_eq!(buf.region_to_str(&Point::new(0, 0), &Point::new(2, 0)).unwrap(), "abc\nde\n");
    assert_eq!(buf.region_to_str(&Point::new(0, 0), &Point::new(1, 4)).unwrap_err(),
        BufErr::InvalidEndPoint);
}

#[test]
fn test_insert_existing_buffer1() {
    let mut buf = Buffer::new();
    assert!(buf.insert_at_pt("abc\nde", &Point::new(0, 0)).is_ok());
    assert_eq!(buf.text_len, 7);
    assert!(buf.insert_at_pt("!", &Point::new(0, 0)).is_ok());
    assert_eq!(buf.text_len, 8);
    assert_eq!(buf.lines.len(), 2);
    // !abc
    // de
    assert_eq!(buf.region_to_str(&Point::new(0, 0), &Point::new(0, 2)).unwrap(), "!a");
    assert_eq!(buf.region_to_str(&Point::new(0, 0), &Point::new(1, 0)).unwrap(), "!abc\n");

    assert!(buf.insert_at_pt("1\n1", &Point::new(0, 4)).is_ok());
    assert_eq!(buf.text_len, 11);
    assert_eq!(buf.lines.len(), 3);
    // !abc1
    // 1
    // de
    assert_eq!(buf.region_to_str(&Point::new(0, 0), &Point::new(0, 5)).unwrap(), "!abc1");
    assert_eq!(buf.region_to_str(&Point::new(1, 0), &Point::new(2, 0)).unwrap(), "1\n");
    assert_eq!(buf.region_to_str(&Point::new(2, 0), &Point::new(3, 0)).unwrap(), "de\n");

    assert!(buf.insert_at_pt("12", &Point::new(1, 1)).is_ok());
    assert_eq!(buf.text_len, 13);
    // !abc1
    // 112
    // de
    assert_eq!(buf.region_to_str(&Point::new(1, 0), &Point::new(2, 0)).unwrap(), "112\n");

    assert!(buf.insert_at_pt("ab\ncde\n", &Point::new(1, 1)).is_ok());
    assert_eq!(buf.text_len, 20);
    // !abc1
    // 1ab
    // cde
    // 12
    // de
    assert_eq!(buf.region_to_str(&Point::new(1, 0), &Point::new(5 ,0)).unwrap(), "1ab\ncde\n12\nde\n");

    assert!(buf.insert_at_pt("\n", &Point::new(2, 2)).is_ok());
    assert_eq!(buf.text_len, 21);
    // !abc1
    // 1ab
    // cd
    // e
    // 12
    // de
    assert_eq!(buf.region_to_str(&Point::new(2, 0), &Point::new(5, 0)).unwrap(), "cd\ne\n12\n");
}

#[test]
fn test_insert_end_of_buffer1() {
    let mut buf = Buffer::with_contents("abc\ndef\nghi");
    assert_eq!(buf.text_len, 12);

    assert!(buf.insert_at_pt("a\nb\n", &Point::new(3, 0)).is_ok());
    assert_eq!(buf.region_to_str(&Point::new(0, 0), &Point::new(5, 0)).unwrap(),
        "abc\ndef\nghi\na\nb\n");
    assert_eq!(buf.text_len, 16);
}

#[test]
fn test_region_to_str1() {
    let buf = Buffer::new();
    assert_eq!(buf.region_to_str(&Point::new(0, 0), &Point::new(0, 0)).unwrap(), "");
    assert_eq!(buf.region_to_str(&Point::new(0, 0), &Point::new(0, 1)).unwrap_err(),
        BufErr::InvalidEndPoint);
}

#[test]
fn test_delete_region1() {
    let mut buf = Buffer::with_contents("abc\ndef\nghi\n");
    // abc
    // def
    // ghi
    assert_eq!(buf.text_len, 12);
    assert_eq!(buf.lines.len(), 3);
    assert_eq!(buf.delete_region(&Point::new(1, 1), &Point::new(2, 0)).unwrap(), "ef\n");
    assert_eq!(buf.text_len, 9);
    // abc
    // dghi
    assert_eq!(buf.lines.len(), 2);
    assert_eq!(buf.region_to_str(&Point::new(0, 0), &Point::new(2, 0)).unwrap(), "abc\ndghi\n");
}

#[test]
fn test_delete_region2() {
    let mut buf = Buffer::with_contents("abc\ndef\nghi\n");
    // abc
    // def
    // ghi
    assert_eq!(buf.delete_region(&Point::new(0, 0), &Point::new(3, 0)).unwrap(),
        "abc\ndef\nghi\n");
    assert_eq!(buf.lines.len(), 1);
    assert_eq!(buf.text_len, 1);
    assert_eq!(buf.region_to_str(&Point::new(0, 0), &Point::new(1, 0)).unwrap(), "\n");
    assert_eq!(buf.region_to_str(&Point::new(0, 0), &Point::new(0, 1)).unwrap_err(),
        BufErr::InvalidEndPoint);
}

#[test]
fn test_delete_region3() {
    let mut buf = Buffer::with_contents("abc\ndef\nghi\njkl\n");
    // abc
    // def
    // ghi
    // jkl
    assert_eq!(buf.lines.len(), 4);
    assert_eq!(buf.text_len, 16);
    assert_eq!(buf.delete_region(&Point::new(0, 1), &Point::new(2, 2)).unwrap(),
        "bc\ndef\ngh");
    // ai
    // jkl
    assert_eq!(buf.lines.len(), 2);
    assert_eq!(buf.to_str(), "ai\njkl\n");
    assert_eq!(buf.text_len, 7);
}

#[test]
fn test_delete_region4() {
    let mut buf = Buffer::with_contents("abc\ndef\nghi\njkl\n");
    // abc
    // def
    // ghi
    // jkl
    assert_eq!(buf.lines.len(), 4);
    assert_eq!(buf.text_len, 16);
    assert_eq!(buf.delete_region(&Point::new(0, 1), &Point::new(3, 3)).unwrap(),
        "bc\ndef\nghi\njkl");
    // a
    assert_eq!(buf.lines.len(), 1);
    assert_eq!(buf.to_str(), "a\n");
    assert_eq!(buf.text_len, 2);
}

#[test]
fn test_insert_edge_case1() {
    let mut buf = Buffer::new();
    assert_eq!(buf.insert_at_pt("a", &Point::new(0, 1)).unwrap_err(), BufErr::InvalidPoint);
    assert_eq!(buf.insert_at_pt("a", &Point::new(1, 0)).unwrap_err(), BufErr::InvalidPoint);
}

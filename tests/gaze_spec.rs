// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use gaze::{Gaze, GazeErr};
use gaze::steps::StringMatch;

#[test]
fn handle_empty_string() {
    let mut gaze = Gaze::new("");
    assert_eq!(gaze.is_complete(), true);
    assert_eq!(gaze.next(), None);
    assert_eq!(gaze.peek(), None);
}

#[test]
fn handle_single_char_string() {
    let mut gaze = Gaze::new("x");
    assert_eq!(gaze.is_complete(), false);
    assert_eq!(gaze.peek(), Some("x".into()));
    assert_eq!(gaze.is_complete(), false);
    assert_eq!(gaze.next(), Some("x".into()));
    assert_eq!(gaze.is_complete(), true);
    assert_eq!(gaze.next(), None);
    assert_eq!(gaze.peek(), None);
}

#[test]
fn handle_string_matcher() {
    let mut gaze = Gaze::new("this is some text");
    let res = gaze.run(StringMatch("this ".into()));
    assert_eq!(res, Ok("this ".into()));
    assert_eq!(gaze.peek(), Some("i".into()));
    assert_eq!(gaze.is_complete(), false);
    let res = gaze.run(StringMatch("this ".into()));
    assert_eq!(res, Err(GazeErr::NoMatch));
    assert_eq!(gaze.is_complete(), false);
    let res = gaze.run(StringMatch("is some text".into()));
    assert_eq!(res, Ok("is some text".into()));
    assert_eq!(gaze.is_complete(), true);
    let res = gaze.run(StringMatch("is some text".into()));
    assert_eq!(res, Err(GazeErr::NoMatch));
    assert_eq!(gaze.is_complete(), true);
}

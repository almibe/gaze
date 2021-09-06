// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use gaze::{Gaze, GazeErr, Step};
use gaze::steps::{TakeString, IgnoreAll};
use std::collections::HashSet;

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
    let res = gaze.run(&TakeString("this ".into()));
    assert_eq!(res, Ok("this ".into()));
    assert_eq!(gaze.peek(), Some("i".into()));
    assert_eq!(gaze.is_complete(), false);
    let res = gaze.run(&TakeString("this ".into()));
    assert_eq!(res, Err(GazeErr::NoMatch));
    assert_eq!(gaze.is_complete(), false);
    let res = gaze.run(&TakeString("is some text".into()));
    assert_eq!(res, Ok("is some text".into()));
    assert_eq!(gaze.is_complete(), true);
    let res = gaze.run(&TakeString("is some text".into()));
    assert_eq!(res, Err(GazeErr::NoMatch));
    assert_eq!(gaze.is_complete(), true);
}

#[test]
fn handle_ignore_all() {
    let mut gaze = Gaze::new("   \t  this \tis some text \t ");
    let mut hs = HashSet::new();
    hs.insert(' ');
    hs.insert('\t');
    let ignore_all = IgnoreAll(hs);
    let res = gaze.run(&ignore_all);
    assert_eq!(res, Ok(()));
    assert_eq!(gaze.peek(), Some("t".into()));
    assert_eq!(gaze.is_complete(), false);
    let res = gaze.run(&ignore_all);
    assert_eq!(res, Ok(()));
    assert_eq!(gaze.is_complete(), false);
    let res = gaze.run(&TakeString("this \tis some text".into()));
    assert_eq!(res, Ok("this \tis some text".into()));
    assert_eq!(gaze.is_complete(), false);
    let res = gaze.run(&ignore_all);
    assert_eq!(res, Ok(()));
    assert_eq!(gaze.is_complete(), true);
    let res = gaze.run(&ignore_all);
    assert_eq!(res, Ok(()));
    assert_eq!(gaze.is_complete(), true);
}

#[test]
fn nested_steps() {
    struct Internal();

    impl Step<String, GazeErr> for Internal {
        fn attempt(&self, gaze: &mut Gaze) -> Result<String, GazeErr> {
            gaze.run(&TakeString("a".into()))?;
            gaze.run(&TakeString("b".into()))?;
            gaze.run(&TakeString("c".into()))?;
            Ok("abc".into())
        }
    }

    let step = Internal();

    let mut gaze_pass = Gaze::new("abc");
    assert_eq!(gaze_pass.run(&step), Ok("abc".into()));
    assert_eq!(gaze_pass.is_complete(), true);

    let mut gaze_fail = Gaze::new("abd");
    assert_eq!(gaze_fail.run(&step), Err(GazeErr::NoMatch));
    assert_eq!(gaze_fail.is_complete(), false);
    assert_eq!(gaze_fail.current_offset(), 0);
}

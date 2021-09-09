// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use unicode_segmentation::UnicodeSegmentation;

use crate::{Gaze, GazeErr, Step};
use std::collections::HashSet;

pub struct TakeString<'a> {
    value: &'a str,
    graphemes: Vec<&'a str>,
}

impl TakeString<'_> {
    pub fn new(value: &str) -> TakeString {
        let graphemes = value.graphemes(true).collect::<Vec<&str>>();
        TakeString { value, graphemes }
    }
}

impl Step<String, GazeErr> for TakeString<'_> {
    fn attempt(&self, gaze: &mut Gaze) -> Result<String, GazeErr> {
        let mut current_pos = 0usize;
        loop {
            if current_pos >= self.graphemes.len() {
                return Ok(self.value.to_string());
            } else {
                let next_value = gaze.peek();
                match next_value {
                    None => {
                        return Err(GazeErr::NoMatch);
                    }
                    Some(c) => {
                        if self.graphemes[current_pos] == c {
                            gaze.next();
                            current_pos += 1;
                        } else {
                            return Err(GazeErr::NoMatch);
                        }
                    }
                }
            }
        }
    }
}

/// A Step that ignores all chars passed in.
pub struct IgnoreAll<'a>(pub HashSet<&'a str>);

impl Step<(), GazeErr> for IgnoreAll<'_> {
    fn attempt(&self, gaze: &mut Gaze) -> Result<(), GazeErr> {
        //TODO this will need to be rewritten once handle Unicode better
        loop {
            let next_value = gaze.peek();
            match next_value {
                None => {
                    return Ok(());
                }
                Some(c) => {
                    if self.0.contains(c) {
                        gaze.next();
                    } else {
                        return Ok(());
                    }
                }
            }
        }
    }
}

/// A Step that takes values from the String until the predicate fails.
pub struct TakeWhile<'a>(pub &'a dyn Fn(&str) -> bool);

impl Step<String, GazeErr> for TakeWhile<'_> {
    fn attempt(&self, gaze: &mut Gaze) -> Result<String, GazeErr> {
        //TODO this will need to be rewritten once handle Unicode better
        let mut res = String::new();
        loop {
            let next_value = gaze.peek();
            match next_value {
                None => {
                    return Ok(res);
                }
                Some(c) => {
                    if self.0(c) {
                        res += c;
                        gaze.next();
                    } else {
                        return Ok(res);
                    }
                }
            }
        }
    }
}

/// A Step that takes values from the String until the predicate passes.
pub struct TakeUntil<'a>(pub &'a dyn Fn(&str) -> bool);

impl Step<String, GazeErr> for TakeUntil<'_> {
    fn attempt(&self, gaze: &mut Gaze) -> Result<String, GazeErr> {
        //TODO this will need to be rewritten once handle Unicode better
        //TODO also this should share code with TakeWhile
        let mut res = String::new();
        loop {
            let next_value = gaze.peek();
            match next_value {
                None => {
                    return Ok(res);
                }
                Some(c) => {
                    if !self.0(c) {
                        res += c;
                        gaze.next();
                    } else {
                        return Ok(res);
                    }
                }
            }
        }
    }
}

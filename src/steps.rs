// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{GazeErr, Step, Gaze};
use std::collections::HashSet;

pub struct TakeString(pub String);

impl Step<String, GazeErr> for TakeString {
    fn attempt(&self, gaze: &mut Gaze) -> Result<String, GazeErr> {
        //TODO this will need to be rewritten once handle Unicode better
        let mut current_pos = 0usize;
        loop {
            if current_pos >= self.0.len() {
                return Ok(self.0.clone());
            } else {
                let next_value = gaze.peek();
                match next_value {
                    None => {
                        return Err(GazeErr::NoMatch);
                    }
                    Some(c) => {
                        let to_match = self.0.bytes().nth(current_pos).ok_or_else(|| GazeErr::NoMatch)?;
                        let value = c.as_bytes().first().ok_or_else(|| GazeErr::NoMatch)?;
                        if to_match == *value {
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
pub struct IgnoreAll(pub HashSet<char>);

impl Step<(), GazeErr> for IgnoreAll {
    fn attempt(&self, gaze: &mut Gaze) -> Result<(), GazeErr> {
        //TODO this will need to be rewritten once handle Unicode better
        loop {
            let next_value = gaze.peek();
            match next_value {
                None => {
                    return Ok(());
                }
                Some(c) => {
                    let value = (*c.as_bytes().first().ok_or_else(|| GazeErr::NoMatch)?) as char;
                    if self.0.contains(&value) {
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
pub struct TakeWhile<'a>(pub &'a dyn Fn(&char) -> bool);

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
                    let value = (*c.as_bytes().first().ok_or_else(|| GazeErr::NoMatch)?) as char;
                    if self.0(&value) {
                        res += &*c;
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
pub struct TakeUntil<'a>(pub &'a dyn Fn(&char) -> bool);

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
                    let value = (*c.as_bytes().first().ok_or_else(|| GazeErr::NoMatch)?) as char;
                    if !self.0(&value) {
                        res += &*c;
                        gaze.next();
                    } else {
                        return Ok(res);
                    }
                }
            }
        }
    }
}

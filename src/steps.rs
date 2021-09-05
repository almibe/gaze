// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{GazeErr, Step, Gaze};

pub struct StringMatch(pub String);

impl Step<String, GazeErr> for StringMatch {
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

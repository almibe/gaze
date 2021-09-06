// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod steps;

/// The main struct for working with Gaze.
pub struct Gaze<'a> {
    input: &'a str,
    offset: usize,
    line: usize,
    location_in_line: usize,
    save_point: Option<usize>,
}

#[derive(PartialEq,Debug)]
pub enum GazeErr {
    NoMatch
}

pub enum State {}

pub trait Transform<I, O> {
    fn transform(input: I) -> O;
}

pub trait Step<O, E> {
    fn attempt(&self, gaze: &mut Gaze) -> Result<O, E>;
}

impl Gaze<'_> {
    pub fn new(input: &str) -> Gaze {
        Gaze {
            input,
            offset: 0,
            line: 0,
            location_in_line: 0,
            save_point: None,
        }
    }

    /// Returns the next char, but doesn't affect the current Parser location.
    /// Returns None if there is no more text.
    pub fn peek(&self) -> Option<String> {
        if self.is_complete() {
            None
        } else {
            let x = self.input.as_bytes()[self.offset] as char; //TODO rewrite
            Some(x.to_string())
        }
    }

    /// Increases the current Parser location 1 space and returns the next char.
    /// Returns None if there is no more text.
    pub fn next(&mut self) -> Option<String> {
        if self.input.len() <= self.offset {
            None
        } else {
            let x = self.input.as_bytes()[self.offset] as char; //TODO rewrite
            self.offset = self.offset + 1;
            Some(x.to_string())
        }
    }

    pub fn run<O, E>(&mut self, step: &impl Step<O, E>) -> Result<O, E> {
        let start = self.offset;
        let res = step.attempt(self);
        match res {
            Ok(_) => {
                res
            }
            Err(_) => {
                self.offset = start;
                res
            }
        }
    }

    pub fn current_offset(&self) -> usize {
        self.offset
    }

    pub fn remaining_text(&self) -> String {
        self.input[self.offset..self.input.len()].to_string()
    }

    pub fn is_complete(&self) -> bool {
        self.input.len() <= self.offset
    }
}

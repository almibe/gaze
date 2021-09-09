// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use unicode_segmentation::UnicodeSegmentation;

pub mod steps;

/// The main struct for working with Gaze.
pub struct Gaze<'a> {
    //input: &'a str,
    graphemes: Vec<&'a str>,
    grapheme_offset: usize,
    // offset: usize,           //byte offset?
    // line: usize,             //line number
    // location_in_line: usize, //grapheme offset of current line
    // save_point: Option<usize>,
}

#[derive(PartialEq, Debug)]
pub enum GazeErr {
    NoMatch,
}

pub trait Step<O, E> {
    fn attempt(&self, gaze: &mut Gaze) -> Result<O, E>;
}

impl Gaze<'_> {
    pub fn new(input: &str) -> Gaze {
        let graphemes = input.graphemes(true).collect::<Vec<&str>>();
        Gaze {
            //input,
            graphemes,
            grapheme_offset: 0,
            // offset: 0,
            // line: 0,
            // location_in_line: 0,
            // save_point: None,
        }
    }

    /// Returns the next char, but doesn't affect the current Parser location.
    /// Returns None if there is no more text.
    pub fn peek(&self) -> Option<&str> {
        if self.is_complete() {
            None
        } else {
            Some(self.graphemes[self.grapheme_offset])
        }
    }

    /// Increases the current Parser location 1 space and returns the next char.
    /// Returns None if there is no more text.
    pub fn next(&mut self) -> Option<&str> {
        if self.graphemes.len() <= self.grapheme_offset {
            None
        } else {
            let next = self.graphemes[self.grapheme_offset];
            self.grapheme_offset += 1;
            Some(next)
        }
    }

    pub fn run<O, E>(&mut self, step: &impl Step<O, E>) -> Result<O, E> {
        let start = self.grapheme_offset;
        let res = step.attempt(self);
        match res {
            Ok(_) => res,
            Err(_) => {
                self.grapheme_offset = start;
                res
            }
        }
    }

    pub fn current_offset(&self) -> usize {
        self.grapheme_offset
    }

    // pub fn remaining_text(&self) -> &str {
    //     &self.input[self.offset..self.input.len()]
    // }

    pub fn is_complete(&self) -> bool {
        self.graphemes.len() <= self.grapheme_offset
    }
}

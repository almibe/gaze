// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ops::Range;

/// The main struct for working with Gaze.
pub struct Gaze<'a> {
    input: &'a str,
    offset: usize,
    line: usize,
    location_in_line: usize,
    save_point: Option<usize>,
}

pub trait Transform<I,O> {
    fn transform(input: I) -> O;
}

impl Gaze<'_> {
    fn bite(&mut self, distance: usize) {
        self.offset = self.offset + distance;
    }

    fn current_offset(&self) -> usize {
        self.offset
    }

    fn remaining_text(&self) -> String {
        self.input[self.offset..self.input.len()].to_string()
    }

    fn is_complete(&self) -> bool {
        self.input.len() <= self.offset
    }
}

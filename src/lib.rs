// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use unicode_segmentation::UnicodeSegmentation;

pub mod steps;

pub struct Gaze<I> {
    offset: usize,
    input: Vec<I>,
}

impl<I> Gaze<I> {
    pub fn from_str(text: &str) -> Gaze<&str> {
        let input = text.graphemes(true).collect::<Vec<&str>>();
        Gaze { input, offset: 0 }
    }

    pub fn from_vec(input: Vec<I>) -> Gaze<I> {
        Gaze { input, offset: 0 }
    }

    pub fn is_complete(&self) -> bool {
        self.offset >= self.input.len()
    }

    pub fn peek(&self) -> Option<I>
    where
        I: Clone,
    {
        if self.is_complete() {
            None
        } else {
            Some(self.input[self.offset].clone())
        }
    }

    pub fn next(&mut self) -> Option<I>
    where
        I: Clone,
    {
        if self.is_complete() {
            None
        } else {
            let next = Some(self.input[self.offset].clone());
            self.offset += 1;
            next
        }
    }

    pub fn attempt<T, E>(&mut self, step: &Step<I, T, E>) -> Result<T, E>
    where
        I: Clone,
        T: Clone,
    {
        let start_of_this_loop = self.offset;
        let res = step(self);
        match res {
            Ok(_) => res,
            Err(e) => {
                self.offset = start_of_this_loop;
                Err(e)
            }
        }
    }

    pub fn ignore<T, E>(&mut self, step: &Step<I, T, E>)
    where
        I: Clone,
        T: Clone,
    {
        let start_of_this_loop = self.offset;
        let res = step(self);
        if res.is_err() {
            self.offset = start_of_this_loop;
        }
    }
}

pub type Step<I, T, E> = dyn Fn(&mut Gaze<I>) -> Result<T, E>;

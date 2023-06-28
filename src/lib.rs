// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use unicode_segmentation::UnicodeSegmentation;

pub mod nibblers;

pub struct Gaze<I> {
    offset: usize,
    input: Vec<I>,
}

impl<I> Gaze<I> {
    /// A helper constructor that takes a str and converts it to a Vec of graphemes.
    pub fn from_str(text: &str) -> Gaze<&str> {
        let input = text.graphemes(true).collect::<Vec<&str>>();
        Gaze { input, offset: 0 }
    }

    /// Construct a Gaze instances from a Vec.
    pub fn from_vec(input: Vec<I>) -> Gaze<I> {
        Gaze { input, offset: 0 }
    }

    /// Returns true if all data has been consumed.
    pub fn is_complete(&self) -> bool {
        self.offset >= self.input.len()
    }

    /// Look at the next 
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

    pub fn attempt<O>(&mut self, step: &Step<I, O>) -> Option<O>
    where
        I: Clone,
        O: Clone,
    {
        let start_of_this_loop = self.offset;
        let res = step(self);
        match res {
            Some(_) => res,
            None => {
                self.offset = start_of_this_loop;
                None
            }
        }
    }

    pub fn attempt_nibbler<O>(&mut self, nibbler: &mut dyn Nibbler<I, O>) -> Option<O>
    where
        I: Clone,
        O: Clone,
    {
        let start_of_this_loop = self.offset;
        let res = nibbler.run(self);
        match res {
            Some(_) => res,
            None => {
                self.offset = start_of_this_loop;
                None
            }
        }
    }

    pub fn ignore<O>(&mut self, step: &Step<I, O>)
    where
        I: Clone,
        O: Clone,
    {
        let start_of_this_loop = self.offset;
        let res = step(self);
        if res.is_none() {
            self.offset = start_of_this_loop;
        }
    }
}

pub type Step<I, O> = dyn Fn(&mut Gaze<I>) -> Option<O>;

pub trait Nibbler<I, O> {
    fn run(&mut self, gaze: &mut Gaze<I>) -> Option<O>;
}

pub struct TakeNibbler<I: Clone> { pub to_match: I }

impl <I: Clone + PartialEq>Nibbler<I, I> for TakeNibbler<I> {
    fn run(&mut self, gaze: &mut Gaze<I>) -> Option<I> {
        match gaze.next() {
            Some(value) => {
                if value == self.to_match {
                    Some(self.to_match.clone())
                } else {
                    None
                }
            },
            None => None
        }
    }
}

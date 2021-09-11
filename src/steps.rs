// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use unicode_segmentation::UnicodeSegmentation;

use crate::{Gaze, Match, NoMatch, Tokenizer};
use std::collections::HashSet;

pub struct TakeString<'a, T> {
    graphemes: Vec<&'a str>,
    token: T,
}

impl<T> TakeString<'_, T>
where
    T: Copy,
{
    pub fn new(value: &str, token: T) -> TakeString<T> {
        let graphemes = value.graphemes(true).collect::<Vec<&str>>();
        TakeString { graphemes, token }
    }
}

impl<T> Tokenizer<T> for TakeString<'_, T>
where
    T: Copy,
{
    fn attempt(&self, gaze: &mut Gaze) -> Result<Match<T>, NoMatch> {
        let mut current_pos = 0usize;
        loop {
            if current_pos >= self.graphemes.len() {
                return Ok(Match(self.token));
            } else {
                let next_value = gaze.peek();
                match next_value {
                    None => {
                        return Err(NoMatch);
                    }
                    Some(c) => {
                        if self.graphemes[current_pos] == c {
                            gaze.next();
                            current_pos += 1;
                        } else {
                            return Err(NoMatch);
                        }
                    }
                }
            }
        }
    }
}

// /// A Step that ignores all chars passed in.
// pub struct IgnoreAll<'a>(pub HashSet<&'a str>);

// impl Tokenizer<()> for IgnoreAll<'_> {
//     fn attempt(&self, gaze: &mut Gaze) -> Result<(), GazeErr> {
//         //TODO this will need to be rewritten once handle Unicode better
//         loop {
//             let next_value = gaze.peek();
//             match next_value {
//                 None => {
//                     return Ok(());
//                 }
//                 Some(c) => {
//                     if self.0.contains(c) {
//                         gaze.next();
//                     } else {
//                         return Ok(());
//                     }
//                 }
//             }
//         }
//     }
// }

// /// A Step that takes values from the String until the predicate fails.
// pub struct TakeWhile<'a>(pub &'a dyn Fn(&str) -> bool);

// impl Tokenizer<String> for TakeWhile<'_> {
//     fn attempt(&self, gaze: &mut Gaze) -> Result<String, GazeErr> {
//         //TODO this will need to be rewritten once handle Unicode better
//         let mut res = String::new();
//         loop {
//             let next_value = gaze.peek();
//             match next_value {
//                 None => {
//                     return Ok(res);
//                 }
//                 Some(c) => {
//                     if self.0(c) {
//                         res += c;
//                         gaze.next();
//                     } else {
//                         return Ok(res);
//                     }
//                 }
//             }
//         }
//     }
// }

// /// A Step that takes values from the String until the predicate passes.
// pub struct TakeUntil<'a>(pub &'a dyn Fn(&str) -> bool);

// impl Tokenizer<String> for TakeUntil<'_> {
//     fn attempt(&self, gaze: &mut Gaze) -> Result<String, GazeErr> {
//         //TODO this will need to be rewritten once handle Unicode better
//         //TODO also this should share code with TakeWhile
//         let mut res = String::new();
//         loop {
//             let next_value = gaze.peek();
//             match next_value {
//                 None => {
//                     return Ok(res);
//                 }
//                 Some(c) => {
//                     if !self.0(c) {
//                         res += c;
//                         gaze.next();
//                     } else {
//                         return Ok(res);
//                     }
//                 }
//             }
//         }
//     }
// }

// pub struct TakeFirst<'a, T>(pub Box<[&'a dyn Tokenizer<T>]>);

// impl<T> Tokenizer<T> for TakeFirst<'_, T> {
//     fn attempt(&self, gaze: &mut Gaze) -> Result<T, GazeErr> {
//         for step in &*self.0 {
//             let res = gaze.run(*step);
//             match res {
//                 Ok(_) => return res,
//                 Err(_) => continue,
//             }
//         }
//         Err(GazeErr::NoMatch)
//     }
// }

// pub struct TakeAll<'a, T>(pub Box<[&'a dyn Tokenizer<T>]>);

// impl<T> Tokenizer<Box<[T]>> for TakeAll<'_, T> {
//     fn attempt(&self, gaze: &mut Gaze) -> Result<Box<[T]>, GazeErr> {
//         let mut res: Vec<T> = Vec::new();
//         for step in &*self.0 {
//             let r = gaze.run(*step);
//             match r {
//                 Ok(r) => res.push(r),
//                 Err(_) => return Err(GazeErr::NoMatch),
//             }
//         }
//         Ok(res.into_boxed_slice())
//     }
// }

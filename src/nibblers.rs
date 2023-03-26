// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use unicode_segmentation::UnicodeSegmentation;

use crate::Gaze;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NoMatch;

pub fn take_string<'a>(
    to_match: &'a str,
) -> impl Fn(&mut Gaze<&str>) -> Result<&'a str, NoMatch> + 'a {
    let graphemes = to_match.graphemes(true).collect::<Vec<&str>>();
    move |gaze: &mut Gaze<&str>| -> Result<&str, NoMatch> {
        let mut offset = 0usize;
        while offset < graphemes.len() {
            let next_char = gaze.next();
            match next_char {
                Some(next_char) => {
                    if graphemes[offset] == next_char {
                        offset += 1;
                    } else {
                        return Err(NoMatch);
                    }
                }
                None => return Err(NoMatch),
            }
        }
        Ok(to_match)
    }
}

pub fn ignore_all<'a>(
    to_match: Vec<&'a str>, //TODO maybe make this an array instead of Vec
) -> impl Fn(&mut Gaze<&'a str>) -> Result<(), NoMatch> {
    move |gaze: &mut Gaze<&'a str>| -> Result<(), NoMatch> {
        while !gaze.is_complete() {
            let peek = gaze.peek();
            match peek {
                Some(peek) => {
                    if to_match.contains(&peek) {
                        gaze.next();
                    } else {
                        return Ok(());
                    }
                }
                None => return Ok(()),
            }
        }
        Ok(())
    }
}

pub fn take_while_str(
    matcher: impl Fn(&str) -> bool,
) -> impl Fn(&mut Gaze<&str>) -> Result<String, NoMatch> {
    move |gaze: &mut Gaze<&str>| -> Result<String, NoMatch> {
        let mut res = String::new();
        loop {
            let peek = gaze.peek();
            match peek {
                Some(peek) => {
                    if matcher(peek) {
                        gaze.next();
                        res += peek;
                    } else if res.is_empty() {
                        return Err(NoMatch);
                    } else {
                        return Ok(res);
                    }
                }
                None => {
                    if res.is_empty() {
                        return Err(NoMatch);
                    } else {
                        return Ok(res);
                    }
                }
            }
        }
    }
}

pub fn take_while<T>(
    matcher: impl Fn(T) -> bool,
) -> impl Fn(&mut Gaze<T>) -> Result<Vec<T>, NoMatch>
where
    T: Copy,
{
    move |gaze: &mut Gaze<T>| -> Result<Vec<T>, NoMatch> {
        let mut res = Vec::new();
        loop {
            let next = gaze.next();
            match next {
                Some(next) => {
                    if matcher(next) {
                        res.push(next);
                    } else if res.is_empty() {
                        return Err(NoMatch);
                    } else {
                        return Ok(res);
                    }
                }
                None => {
                    if res.is_empty() {
                        return Err(NoMatch);
                    } else {
                        return Ok(res);
                    }
                }
            }
        }
    }
}

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

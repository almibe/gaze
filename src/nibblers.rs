// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{Gaze, Nibbler};

pub struct TakeNblr<I: Clone> {
    pub to_match: I,
}

impl<I: Clone + PartialEq, E> Nibbler<I, I, E> for TakeNblr<I> {
    fn run(&self, gaze: &mut Gaze<I>) -> Result<Option<I>, E> {
        match gaze.next() {
            Some(value) => {
                if value == self.to_match {
                    Ok(Some(self.to_match.clone()))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}

pub struct ConvertNblr<I: Clone, O: Clone> {
    pub to_match: I,
    pub new_value: O,
}

impl<I: Clone + PartialEq, O: Clone, E> Nibbler<I, O, E> for ConvertNblr<I, O> {
    fn run(&self, gaze: &mut Gaze<I>) -> Result<Option<O>, E> {
        match gaze.next() {
            Some(value) => {
                if value == self.to_match {
                    Ok(Some(self.new_value.clone()))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}

pub struct TakeFirstNblr<I, O, E>(pub Vec<Box<dyn Nibbler<I, O, E>>>);

impl<I: Clone + PartialEq, O: Clone + PartialEq, E> Nibbler<I, O, E> for TakeFirstNblr<I, O, E> {
    fn run(&self, gaze: &mut Gaze<I>) -> Result<Option<O>, E> {
        for nibbler in &self.0 {
            if let Ok(Some(value)) = gaze.attempt(nibbler.as_ref()) {
                return Ok(Some(value));
            }
        }
        Ok(None)
    }
}

// pub struct MapNblr<I, O, NO, E> {
//     pub nibbler: Box<dyn Nibbler<I, O, E>>,
//     pub mapper: dyn Fn(O) -> NO,
// }

// impl<I: Clone + PartialEq, O: Clone + PartialEq, NO, E> Nibbler<I, NO, E> for MapNblr<I, O, NO, E> {
//     fn run(&self, gaze: &mut Gaze<I>) -> Result<Option<NO>, E> {
//         gaze.attempt(self.nibbler.as_ref()).map(&self.mapper)
//     }
// }

// pub fn take_string<'a>(
//     to_match: &'a str,
// ) -> impl Fn(&mut Gaze<&str>) -> Option<&'a str> + 'a {
//     let graphemes = to_match.graphemes(true).collect::<Vec<&str>>();
//     move |gaze: &mut Gaze<&str>| -> Option<&str> {
//         let mut offset = 0usize;
//         while offset < graphemes.len() {
//             let next_char = gaze.next();
//             match next_char {
//                 Some(next_char) => {
//                     if graphemes[offset] == next_char {
//                         offset += 1;
//                     } else {
//                         return None;
//                     }
//                 }
//                 None => return None,
//             }
//         }
//         Some(to_match)
//     }
// }

// pub fn ignore_all<'a>(
//     to_match: Vec<&'a str>, //TODO maybe make this an array instead of Vec
// ) -> impl Fn(&mut Gaze<&'a str>) -> Option<()> {
//     move |gaze: &mut Gaze<&'a str>| -> Option<()> {
//         while !gaze.is_complete() {
//             let peek = gaze.peek();
//             match peek {
//                 Some(peek) => {
//                     if to_match.contains(&peek) {
//                         gaze.next();
//                     } else {
//                         return Some(());
//                     }
//                 }
//                 None => return Some(()),
//             }
//         }
//         Some(())
//     }
// }

// pub fn take_while_str(
//     matcher: impl Fn(&str) -> bool,
// ) -> impl Fn(&mut Gaze<&str>) -> Option<String> {
//     move |gaze: &mut Gaze<&str>| -> Option<String> {
//         let mut res = String::new();
//         loop {
//             let peek = gaze.peek();
//             match peek {
//                 Some(peek) => {
//                     if matcher(peek) {
//                         gaze.next();
//                         res += peek;
//                     } else if res.is_empty() {
//                         return None;
//                     } else {
//                         return Some(res);
//                     }
//                 }
//                 None => {
//                     if res.is_empty() {
//                         return None;
//                     } else {
//                         return Some(res);
//                     }
//                 }
//             }
//         }
//     }
// }

// struct TakeWhile<T> { matcher: dyn Fn(T) -> bool }
// impl <T>Nibbler<T, T> for TakeWhile<T> {
//     fn run(&mut self, gaze: &mut Gaze<T>) -> Option<T> {
//         todo!()
//     }
// }

// pub fn take_while<T>(
//     matcher: impl Fn(T) -> bool,
// ) -> impl Fn(&mut Gaze<T>) -> Option<Vec<T>>
// where
//     T: Copy,
// {
//     move |gaze: &mut Gaze<T>| -> Option<Vec<T>> {
//         let mut res = Vec::new();
//         loop {
//             let next = gaze.next();
//             match next {
//                 Some(next) => {
//                     if matcher(next) {
//                         res.push(next);
//                     } else if res.is_empty() {
//                         return None;
//                     } else {
//                         return Some(res);
//                     }
//                 }
//                 None => {
//                     if res.is_empty() {
//                         return None;
//                     } else {
//                         return Some(res);
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

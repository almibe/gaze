// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::LookAhead;
use crate::NibState;
use crate::Raccoon;

/// StringNibbler matches a given String entirely
pub struct StringRaccoon(String);

impl Raccoon for StringRaccoon {
    fn taste(&self, lookAhead: &dyn LookAhead) -> NibState {
        let mut offset = 0usize;
        let to_match = &self.0;
        while offset <= to_match.len() {
            if offset == to_match.len() - 1
                && to_match.chars().nth(offset) == lookAhead.peek(offset)
            {
                return NibState::Complete(offset + 1);
            } else if offset < to_match.len()
                && to_match.chars().nth(offset) == lookAhead.peek(offset)
            {
                offset += 1
            } else {
                return NibState::Cancel;
            }
        }
        NibState::Cancel
    }
}

//**
// * CharNibbler matches input against the passed characters
// */
//struct CharNibbler(chars: Char*) extends Nibbler {
//  override def taste(lookAhead: LookAhead): NibState = {
//    var offset = 0
//    while (lookAhead.peek(offset) != null) {
//      var isMatch = false
//      for ( char <- chars) {
//        if (char == lookAhead.peek(offset)) {
//          isMatch = true //TODO should break
//        }
//      }
//      if (!isMatch) {
//        break
//      } else {
//        offset += 1
//      }
//    }
//    if (offset == 0) {
//      Cancel()
//    } else {
//      Complete (offset)
//    }
//  }
//}

//**
// * RangeNibbler matches characters that exist within a given set of CharRanges
// */
//struct RangeNibbler(ranges: NumericRange.Inclusive[Char]*) extends Nibbler {
//  override def taste(lookAhead: LookAhead): NibState = {
//    var offset = 0
//    while (lookAhead.peek(offset) != null) {
//      var isMatch = false
//      ranges.forEach check
//      @
//      {
//        if (it.contains(lookAhead.peek(offset))) {
//          isMatch = true
//          return
//          @check
//        }
//      }
//      if (!isMatch) {
//        break
//      } else {
//        offset += 1
//      }
//    }
//    if (offset == 0) {
//      Cancel
//    } else {
//      Complete(offset)
//    }
//  }
//}

//**
// * predicateNibbler is helper that checks a single character against a given predicate
// */
//def predicateNibbler(fn: (Char | null) -> Boolean) = Nibbler { lookAhead ->
//    var offset = 0
//    while (lookAhead.peek(offset) != null) {
//        if (fn(lookAhead.peek(offset))) {
//            offset += 1
//        } else {
//            break
//        }
//    }
//    if (offset == 0) {
//        Cancel
//    } else {
//        Complete(offset)
//    }
//}

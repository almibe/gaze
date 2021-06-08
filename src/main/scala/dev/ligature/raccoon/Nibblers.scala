/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.raccoon

///**
// * stringNibbler matches a given String entirely
// */
//def stringNibbler(toMatch: String) = Nibbler { lookAhead ->
//    var offset = 0
//    while (offset <= toMatch.length) {
//        if (offset == toMatch.length - 1 && toMatch.charAt(offset) == lookAhead.peek(offset)) {
//            return Complete(offset + 1)
//        } else if (offset < toMatch.length && toMatch.charAt(offset) == lookAhead.peek(offset)) {
//            offset += 1
//        } else {
//            return Cancel
//        }
//    }
//    return Cancel
//}
//
///**
// * charNibbler matches input against the passed characters
// */
//def charNibbler(chars: Char*) = Nibbler { lookAhead ->
//    var offset = 0
//    while (lookAhead.peek(offset) != null) {
//        var isMatch = false
//        chars.forEach check@{
//            if (it == lookAhead.peek(offset)) {
//                isMatch = true
//                return@check
//            }
//        }
//        if (!isMatch) {
//            break
//        } else {
//            offset += 1
//        }
//    }
//    if (offset == 0) {
//        Cancel
//    } else {
//        Complete(offset)
//    }
//}
//
///**
// * rangeNibbler matches characters that exist within a given set of CharRanges
// */
//def rangeNibbler(ranges: NumericRange.Inclusive[Char]*) = Nibbler { lookAhead ->
//    var offset = 0
//    while (lookAhead.peek(offset) != null) {
//        var isMatch = false
//        ranges.forEach check@{
//            if (it.contains(lookAhead.peek(offset))) {
//                isMatch = true
//                return@check
//            }
//        }
//        if (!isMatch) {
//            break
//        } else {
//            offset += 1
//        }
//    }
//    if (offset == 0) {
//        Cancel
//    } else {
//        Complete(offset)
//    }
//}
//
///**
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

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

/**
 * stringNibbler matches a given String entirely
 */
fun stringNibbler(toMatch: String) = Nibbler { char, lookAhead ->
//    if (current.length == toMatch.length - 1 && toMatch[current.length] == char) {
//        Complete()
//    } else if (current.length < toMatch.length - 1 && toMatch[current.length] == char) {
//        Next
//    } else {
//        Cancel
//    }
    TODO()
}

/**
 * rangeNibbler matches characters that exist within a given set of CharRanges
 */
fun rangeNibbler(vararg ranges: CharRange) = Nibbler { char, lookAhead ->
//    if (char == null) {
//        if (current.isEmpty()) Cancel
//        else Complete()
//    } else {
//        var match = false
//        ranges.forEach check@{
//            if (it.contains(char)) {
//                match = true
//                return@check
//            }
//        }
//        if (match) {
//            Next
//        } else {
//            if (current.isEmpty()) {
//                Cancel
//            } else {
//                Complete(1)
//            }
//        }
//    }
    TODO()
}

/**
 * predicateNibbler is helper that checks a single character against a given predicate
 */
fun predicateNibbler(fn: (Char?) -> Boolean) = Nibbler { char, lookAhead ->
//    if (char == null) {
//        if (current.isEmpty()) Cancel
//        else Complete()
//    }
//    else if (fn(char)) Next
//    else {
//        if (current.isEmpty()) Cancel
//        else Complete(1)
//    }
    TODO()
}

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.Option
import arrow.core.Some
import arrow.core.none

fun interface Pattern {
    fun matches(input: CharSequence): Option<MatchInfo>
}

fun stringPattern(toMatch: String) = Pattern { input ->
    if (input.startsWith(toMatch)) {
        Some(MatchInfo(toMatch.length))
    } else {
        none()
    }
}

fun regexPattern(pattern: Regex) = Pattern { input ->
    val matchRes = pattern.find(input)
    if (matchRes != null && matchRes.range.first == 0) {
        Some(MatchInfo(matchRes.range.last + 1))
    } else {
        none()
    }
}

fun rangePattern(vararg ranges: CharRange) = Pattern { input ->
    fun rangeMatches(char: Char): Boolean {
        ranges.forEach {
            if (it.contains(char)) {
                return true
            }
        }
        return false
    }

    var length = 0
    while (input.length > length && rangeMatches(input[length])) {
        length++
    }
    if (length > 0) {
        Some(MatchInfo(length))
    } else {
        none()
    }
}

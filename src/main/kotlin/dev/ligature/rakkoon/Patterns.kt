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

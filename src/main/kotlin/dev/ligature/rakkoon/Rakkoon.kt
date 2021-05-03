/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.*

sealed class NibState
object Cancel: NibState()
data class Complete(val backtrack: Int = 0): NibState()
object Next: NibState()

fun interface Nibbler {
    fun taste(char: Char?): NibState
}

data class Match(val value: String, val range: IntRange)

class Rakkoon(private var input: CharSequence) {
    private var offset = 0

    fun nibble(nibbler: Nibbler): Option<Match> {
        val start = offset
        while(offset < input.length) {
            when (val res = nibbler.taste(input[offset])) {
                is Cancel -> {
                    offset = start
                    return none()
                }
                is Complete -> {
                    offset++
                    offset -= res.backtrack
                    val finalChar = offset
                    return Some(Match(input.substring(start, finalChar), IntRange(start, finalChar)))
                }
                is Next -> {
                    offset++
                }
            }
        }
        return when (val finalRes = nibbler.taste(null)) {
            is Cancel, Next -> {
                offset = start
                none()
            }
            is Complete -> {
                offset -= finalRes.backtrack
                Some(Match(input.substring(start, offset), IntRange(start, offset)))
            }
        }
    }

    fun currentOffset(): Int = offset

    fun remainingText(): String = input.substring(offset)

    fun isComplete(): Boolean {
        return input.length <= offset
    }
}

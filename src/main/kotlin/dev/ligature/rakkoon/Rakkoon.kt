/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.*

sealed class NibState

/**
 * The Cancel state means that this Nibbler didn't match and Rakkoon should jump back to its position before
 * starting this Nibbler.
 * The nibble method will also return None.
 */
object Cancel: NibState()

/**
 * The Complete state means that this Nibbler completed and Rakkoon should adjust its offset based on the adjust param.
 * A Some(Match) is returned by the nibble method.
 */
data class Complete(val adjust: Int = 0): NibState()

/**
 * The Next state implies that the text so far has passed the taste test and a new character is being requested.
 */
object Next: NibState()

/**
 * The NextSkip state is similar to Next but accepts a UInt that allows you to skip multiple characters in the case
 * that look ahead was used.
 */
data class NextSkip @OptIn(ExperimentalUnsignedTypes::class) constructor(val skip: UInt = 1U): NibState()

fun interface Nibbler {
    fun taste(char: Char?, lookAhead: LookAhead): NibState
}

interface LookAhead {
    @OptIn(ExperimentalUnsignedTypes::class)
    fun peek(distance: UInt = 1U): Char?
}

data class Match(val value: String, val range: IntRange)

class Rakkoon(private var input: CharSequence): LookAhead {
    private var offset = 0

    @OptIn(ExperimentalUnsignedTypes::class)
    override fun peek(distance: UInt): Char? =
        if (offset + distance.toInt() - 1 < input.length) input[offset + distance.toInt() - 1]
        else null

    fun nibble(nibbler: Nibbler): Option<Match> {
        val start = offset
        while(offset < input.length) {
            when (val res = nibbler.taste(input[offset], this)) {
                is Cancel -> {
                    offset = start
                    return none()
                }
                is Complete -> {
                    offset++
                    offset += res.adjust
                    val finalChar = offset
                    return Some(Match(input.substring(start, finalChar), IntRange(start, finalChar)))
                }
                is Next -> {
                    offset++
                }
            }
        }
        return when (val finalRes = nibbler.taste(null, this)) {
            is Cancel, is Next, is NextSkip -> {
                offset = start
                none()
            }
            is Complete -> {
                offset += finalRes.adjust
                Some(Match(input.substring(start, offset), IntRange(start, offset)))
            }
        }
    }

    fun nibble(vararg nibblers: Nibbler): Option<List<Match>> {
        val resultList = mutableListOf<Match>()
        val start = offset
        for (nibbler in nibblers) {
            when (val res = nibble(nibbler)) {
                is None -> {
                    offset = start
                    return none()
                }
                is Some -> {
                    resultList.add(res.value)
                }
            }
        }
        return Some(resultList)
    }

    fun currentOffset(): Int = offset

    fun remainingText(): String = input.substring(offset)

    fun isComplete(): Boolean {
        return input.length <= offset
    }
}

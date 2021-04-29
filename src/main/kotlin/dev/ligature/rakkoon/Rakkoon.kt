/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.*

data class Rule<T>(val pattern: Pattern, val action: Action<T>)
data class MatchInfo(val match: IntRange, val endChar: Int)

sealed class RakkoonError
data class NoMatch(val charOffset: Int): RakkoonError()
data class ActionError(val message: String, val charOffset: Int): RakkoonError()

class Rakkoon(private var input: CharSequence) {
    private var offset = 0

    fun <T>bite(rule: Rule<T>): Either<RakkoonError, T> {
        val matchInfo = rule.pattern.matches(input)
        return if (!isComplete() && matchInfo is Some) {
            val sub = input.substring(matchInfo.value.match.first, matchInfo.value.match.last)
            offset += matchInfo.value.endChar
            input = input.subSequence(matchInfo.value.endChar, input.length)
            rule.action.action(sub)
        } else {
            Either.Left(NoMatch(offset))
        }
    }

    fun currentOffset(): Int = offset

    fun remainingText(): String = input.toString()

    fun isComplete(): Boolean {
        return input.isEmpty()
    }
}

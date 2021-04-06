/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.*

data class Rule<T>(val pattern: Pattern, val action: Action<T>)
data class MatchInfo(val endChar: Int)

sealed class RakkoonError
data class NoMatch(val charOffset: Int): RakkoonError()
data class ActionError(val message: String, val charOffset: Int): RakkoonError()

class Rakkoon(private var input: CharSequence) {
    fun <T>bite(rule: Rule<T>): Either<RakkoonError, T> {
        val matchInfo = rule.pattern.matches(input)
        return if (!isComplete() && matchInfo is Some) {
            val sub = input.substring(0, matchInfo.value.endChar)
            input = input.subSequence(matchInfo.value.endChar, input.length)
            rule.action.action(sub)
        } else {
            Either.Left(NoMatch(0)) //TODO compute actual offset
        }
    }

    fun isComplete(): Boolean {
        return input.isEmpty()
    }
}

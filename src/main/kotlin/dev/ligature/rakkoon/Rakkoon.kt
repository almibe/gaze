/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.*

data class Rule<T>(val pattern: Pattern, val action: Action<T>)

data class MatchInfo(val endChar: Int)

data class RakkoonError(val message: String, val charOffset: Int)

val noMatchFound = RakkoonError("No match found.", 0)

fun interface Pattern {
    fun matches(input: String): Option<MatchInfo>
}

fun interface Action<T> {
    fun action(token: String): Either<RakkoonError, T>
}

val ignore = Action { Either.Right(Unit) }

fun <T>rakkoon(input: String, rule: Rule<T>): Either<RakkoonError, T> {
    val matchInfo = rule.pattern.matches(input)
    return if (matchInfo is Some && input.length == matchInfo.value.endChar-1) {
        val sub = input.substring(0, matchInfo.value.endChar)
        rule.action.action(sub)
    } else {
        Either.Left(noMatchFound)
    }
}

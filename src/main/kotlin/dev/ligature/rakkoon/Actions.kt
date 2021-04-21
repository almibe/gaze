/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.*

fun interface Action<T> {
    fun action(token: CharSequence): Either<ActionError, T>
}

val toIntAction = Action { token ->
    Either.Right(token.toString().toInt())
}

val valueAction = Action { token ->
    Either.Right(token)
}

val ignoreAction = Action { Either.Right(Unit) }

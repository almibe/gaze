/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.Either
import arrow.core.Some
import arrow.core.none
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

private val fivesPattern = Pattern { input ->
    if (input.startsWith("5")) {
        Some(MatchInfo(1))
    } else {
        none()
    }
}

private val fivesAction = Action { token ->
    Either.Right(token.toInt())
}

class FivesSpec : FunSpec() {
    init {
        test("empty input") {
            rakkoon("", Rule(fivesPattern, fivesAction)).shouldBe(listOf<Int>())
        }
    }
}

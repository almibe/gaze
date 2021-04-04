/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.Either
import arrow.core.Some
import arrow.core.none
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

private val fivePattern = Pattern { input ->
    if (input.startsWith("5")) {
        Some(MatchInfo(1))
    } else {
        none()
    }
}

private val fiveAction = Action { token ->
    Either.Right(token.toInt())
}

private val fiveRule = Rule(fivePattern, fiveAction)

class FiveSpec : FunSpec() {
    init {
        test("empty input") {
            rakkoon("", fiveRule).shouldBe(none<Int>())
        }

        test("single 5 input") {
            rakkoon("5", fiveRule).shouldBe(Some<Int>(5))
        }

        test("single 4 input") {
            rakkoon("4", fiveRule).shouldBe(Some<Int>(5))
        }
    }
}

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.Either
import io.kotest.core.annotation.Ignored
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeInstanceOf

private val fivesRule = Rule(stringPattern("5"), toIntAction)

@Ignored
class FivesSpec : FunSpec() {
    init {
        test("empty input") {
            val rakkoon = Rakkoon("")
            rakkoon.bite(fivesRule).shouldBeInstanceOf<Either.Left<RakkoonError>>()
        }

        test("single 5 input") {
            val rakkoon = Rakkoon("5")
            rakkoon.bite(fivesRule).shouldBe(Either.Right(listOf(5)))
        }

        test("single 4 input") {
            val rakkoon = Rakkoon("4")
            rakkoon.bite(fivesRule).shouldBeInstanceOf<Either.Left<RakkoonError>>()
        }

        test("repeating 5s input") {
            val rakkoon = Rakkoon("555555")
            rakkoon.bite(fivesRule).shouldBe(Either.Right(listOf(5,5,5,5,5,5)))
        }
    }
}

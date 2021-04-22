/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.*
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeInstanceOf

private val oneToFiveRule = Rule(rangePattern('1'..'5'), toIntAction)
private val lowerCaseRule = Rule(rangePattern('a'..'z'), valueAction)
private val multiRangeRule = Rule(rangePattern('1'..'5', 'a'..'z'), valueAction)

class RangePatternSpec : FunSpec() {
    init {
        test("empty input") {
            val rakkoon = Rakkoon("")
            rakkoon.bite(oneToFiveRule).shouldBeInstanceOf<Either.Left<RakkoonError>>()
            rakkoon.isComplete().shouldBe(true)
        }

        test("single 5 input") {
            val rakkoon = Rakkoon("5")
            rakkoon.bite(oneToFiveRule).shouldBe(Either.Right(5))
            rakkoon.isComplete().shouldBe(true)
        }

        test("single 7 input") {
            val rakkoon = Rakkoon("7")
            rakkoon.bite(oneToFiveRule).shouldBeInstanceOf<Either.Left<RakkoonError>>()
            rakkoon.isComplete().shouldBe(false)
        }

        test ("multiple 5s input") {
            val rakkoon = Rakkoon("55555")
            rakkoon.bite(oneToFiveRule).shouldBe(Either.Right(55555))
            rakkoon.isComplete().shouldBe(true)
        }

        test("alpha numeric tests") {
            val rakkoon = Rakkoon("1234323243hfjashnvjabjkrbkjab")
            rakkoon.bite(oneToFiveRule).shouldBe(Either.Right(1234323243))
            rakkoon.bite(lowerCaseRule).shouldBe(Either.Right("hfjashnvjabjkrbkjab"))
            rakkoon.isComplete().shouldBe(true)
        }

        test("multiple range test") {
            val rakkoon = Rakkoon("34kl2j4k2n4kl2lh342j34vjg2g4cg")
            rakkoon.bite(multiRangeRule).shouldBe(Either.Right("34kl2j4k2n4kl2lh342j34vjg2g4cg"))
            rakkoon.isComplete().shouldBe(true)
        }
    }
}

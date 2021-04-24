/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.*
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeInstanceOf

private val oneToFiveRule = Rule(regexPattern("[1-5]".toRegex()), toIntAction)
private val oneToFivesRule = Rule(regexPattern("[1-5]+".toRegex()), toIntAction)
private val multiRangeRule = Rule(regexPattern("[1-5a-z]+".toRegex()), valueAction)

class RegexPatternSpec : FunSpec() {
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
            rakkoon.bite(oneToFivesRule).shouldBe(Either.Right(55555))
            rakkoon.isComplete().shouldBe(true)
        }

        test ("multiple 5s and a 7 input") {
            val rakkoon = Rakkoon("555557")
            rakkoon.bite(oneToFivesRule).shouldBe(Either.Right(55555))
            rakkoon.isComplete().shouldBe(false)
        }

        test("alpha numeric tests") {
            val rakkoon = Rakkoon("1234323243hfjashnvjabjkrbkjab")
            rakkoon.bite(oneToFivesRule).shouldBe(Either.Right(1234323243))
            rakkoon.bite(multiRangeRule).shouldBe(Either.Right("hfjashnvjabjkrbkjab"))
            rakkoon.isComplete().shouldBe(true)
        }

        test("multiple range test") {
            val rakkoon = Rakkoon("34kl2j4k2n4kl2lh342j34vjg2g4cg")
            rakkoon.bite(multiRangeRule).shouldBe(Either.Right("34kl2j4k2n4kl2lh342j34vjg2g4cg"))
            rakkoon.isComplete().shouldBe(true)
        }

        test("allow working with multiple lines") {
            val rakkoon = Rakkoon("   1234323243  \n hfjashnvjabjkrbkjab")
            val whiteSpaceIgnoreRule = Rule(regexPattern("[ \n]+".toRegex()), ignoreAction)
            rakkoon.bite(whiteSpaceIgnoreRule)
            rakkoon.bite(whiteSpaceIgnoreRule) //make sure you can repeat
            rakkoon.bite(oneToFivesRule).shouldBe(Either.Right(1234323243))
            rakkoon.bite(whiteSpaceIgnoreRule)
            rakkoon.bite(multiRangeRule).shouldBe(Either.Right("hfjashnvjabjkrbkjab"))
            rakkoon.isComplete().shouldBe(true)
        }
    }
}

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.*
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeInstanceOf

private val fiveRule = Rule(stringPattern("5"), toIntAction)
private val helloRule = Rule(stringPattern("hello"), valueAction)
private val spaceRule = Rule(stringPattern(" "), ignoreAction)
private val worldRule = Rule(stringPattern("world"), valueAction)

class StringPatternSpec : FunSpec() {
    init {
        test("empty input") {
            val rakkoon = Rakkoon("")
            rakkoon.bite(fiveRule).shouldBeInstanceOf<Either.Left<RakkoonError>>()
            rakkoon.isComplete().shouldBe(true)
        }

        test("single 5 input") {
            val rakkoon = Rakkoon("5")
            rakkoon.bite(fiveRule).shouldBe(Either.Right(5))
            rakkoon.isComplete().shouldBe(true)
        }

        test("single 4 input") {
            val rakkoon = Rakkoon("4")
            rakkoon.bite(fiveRule).shouldBeInstanceOf<Either.Left<RakkoonError>>()
            rakkoon.isComplete().shouldBe(false)
        }

        test ("multiple 5s input") {
            val rakkoon = Rakkoon("55555")
            val res = mutableListOf<Int>()
            while(!rakkoon.isComplete()) {
                res.add(rakkoon.bite(fiveRule).getOrElse { TODO() })
            }
            res shouldBe listOf(5,5,5,5,5)
        }

        test("hello work test") {
            val rakkoon = Rakkoon("hello world")
            rakkoon.bite(helloRule).shouldBe(Either.Right("hello"))
            rakkoon.bite(spaceRule).shouldBe(Either.Right(Unit))
            rakkoon.bite(worldRule).shouldBe(Either.Right("world"))
            rakkoon.isComplete().shouldBe(true)
        }
    }
}

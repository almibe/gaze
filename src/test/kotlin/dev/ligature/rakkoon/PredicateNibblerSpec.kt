/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.*
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeInstanceOf

private val fiveNibbler = predicateNibbler { c -> c == '5' }
private val eatAllNibbler = predicateNibbler { c -> c != null }
private val spaceNibbler = predicateNibbler { it?.isWhitespace() ?: false }
private val digitNibbler = predicateNibbler { it?.isDigit() ?: false }

class PredicateNibblerSpec : FunSpec() {
    init {
        test("empty input") {
            val rakkoon = Rakkoon("")
            rakkoon.nibble(fiveNibbler).shouldBeInstanceOf<None>()
            rakkoon.nibble(eatAllNibbler).shouldBeInstanceOf<None>()
            rakkoon.nibble(spaceNibbler).shouldBeInstanceOf<None>()
            rakkoon.nibble(digitNibbler).shouldBeInstanceOf<None>()
            rakkoon.isComplete().shouldBe(true)
        }

        test("single 5 input") {
            val rakkoon = Rakkoon("5")
            rakkoon.nibble(fiveNibbler).shouldBe(Some(Match("5", IntRange(0,1))))
            rakkoon.isComplete().shouldBe(true)
        }

        test("single 4 input") {
            val rakkoon = Rakkoon("4")
            rakkoon.nibble(fiveNibbler).shouldBeInstanceOf<None>()
            rakkoon.isComplete().shouldBe(false)
        }

        test ("multiple 5s input") {
            val rakkoon = Rakkoon("55555")
            val res = rakkoon.nibble(fiveNibbler)
            res shouldBe Some(Match("55555", IntRange(0,5)))
        }

        test("eat all nibbler test") {
            val rakkoon = Rakkoon("hello world")
            rakkoon.nibble(eatAllNibbler).shouldBe(Some(Match("hello world", IntRange(0, 11))))
            rakkoon.isComplete().shouldBe(true)
        }

        test("simple testing vararg overload of nibble") {
            val rakkoon = Rakkoon("5    \t5")
            rakkoon.nibble(fiveNibbler, spaceNibbler, fiveNibbler).shouldBe(Some(listOf(
                Match("5", IntRange(0,1)),
                Match("    \t", IntRange(1,6)),
                Match("5", IntRange(6,7))
            )))
            rakkoon.isComplete().shouldBe(true)
        }

        test("testing vararg overload of nibble with digit nibbler") {
            val rakkoon = Rakkoon("123    \t456")
            rakkoon.nibble(digitNibbler, spaceNibbler, digitNibbler).shouldBe(Some(listOf(
                Match("123", IntRange(0,3)),
                Match("    \t", IntRange(3,8)),
                Match("456", IntRange(8,11))
            )))
            rakkoon.isComplete().shouldBe(true)
        }
    }
}

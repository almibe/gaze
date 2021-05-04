/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.*
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeInstanceOf

private val fiveNibbler = rangeNibbler('5'..'5')
private val digitNibbler = rangeNibbler('0'..'9')
private val alphaNibbler = rangeNibbler('a'..'c', 'x'..'z')

class RangeNibblerSpec : FunSpec() {
    init {
        test("empty input") {
            val rakkoon = Rakkoon("")
            rakkoon.nibble(fiveNibbler).shouldBeInstanceOf<None>()
            rakkoon.nibble(digitNibbler).shouldBeInstanceOf<None>()
            rakkoon.nibble(alphaNibbler).shouldBeInstanceOf<None>()
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

        test ("digit input") {
            val rakkoon = Rakkoon("005676356234534542100000")
            val res = rakkoon.nibble(digitNibbler)
            res shouldBe Some(Match("005676356234534542100000", IntRange(0,24)))
        }

        test("test alpha input") {
            val rakkoon = Rakkoon("abcyyyycccaaazzzz")
            rakkoon.nibble(alphaNibbler).shouldBe(Some(Match("abcyyyycccaaazzzz", IntRange(0, 17))))
            rakkoon.isComplete().shouldBe(true)
        }

        test("simple testing vararg overload of nibble") {
            val rakkoon = Rakkoon("abc456zxya")
            rakkoon.nibble(alphaNibbler, digitNibbler, alphaNibbler).shouldBe(Some(listOf(
                Match("abc", IntRange(0,3)),
                Match("456", IntRange(3,6)),
                Match("zxya", IntRange(6,10))
            )))
            rakkoon.isComplete().shouldBe(true)
        }
    }
}

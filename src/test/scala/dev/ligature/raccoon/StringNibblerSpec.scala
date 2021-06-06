/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.raccoon

import arrow.core.*
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeInstanceOf

private val fiveNibbler = stringNibbler("5")
private val helloNibbler = stringNibbler("hello")
private val spaceNibbler = stringNibbler(" ")
private val worldNibbler = stringNibbler("world")

class StringNibblerSpec : FunSpec() {
    init {
        test("empty input") {
            val rakkoon = Rakkoon("")
            rakkoon.nibble(fiveNibbler).shouldBeInstanceOf<None>()
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
            val res = mutableListOf<Int>()
            while(!rakkoon.isComplete()) {
                val nres = rakkoon.nibble(fiveNibbler)
                if (nres.isNotEmpty()) {
                    res.add((nres as Some).value.value.toInt())
                }
            }
            res shouldBe listOf(5,5,5,5,5)
        }

        test("hello world test") {
            val rakkoon = Rakkoon("hello world")
            rakkoon.nibble(helloNibbler).shouldBe(Some(Match("hello", IntRange(0,5))))
            rakkoon.nibble(spaceNibbler).shouldBe(Some(Match(" ", IntRange(5,6))))
            rakkoon.nibble(worldNibbler).shouldBe(Some(Match("world", IntRange(6,11))))
            rakkoon.isComplete().shouldBe(true)
        }

        test("hello5world test") {
            val rakkoon = Rakkoon("hello5world")
            rakkoon.nibble(helloNibbler).shouldBe(Some(Match("hello", IntRange(0,5))))
            rakkoon.nibble(fiveNibbler).shouldBe(Some(Match("5", IntRange(5,6))))
            rakkoon.nibble(worldNibbler).shouldBe(Some(Match("world", IntRange(6,11))))
            rakkoon.isComplete().shouldBe(true)
        }

        test("simple testing vararg overload of nibble") {
            val rakkoon = Rakkoon("55")
            rakkoon.nibble(fiveNibbler, fiveNibbler).shouldBe(Some(listOf(
                Match("5", IntRange(0,1)),
                Match("5", IntRange(1,2))
            )))
            rakkoon.isComplete().shouldBe(true)
        }

        test("testing vararg overload of nibble") {
            val rakkoon = Rakkoon("5hello5")
            rakkoon.nibble(fiveNibbler, helloNibbler, fiveNibbler).shouldBe(Some(listOf(
                Match("5", IntRange(0,1)),
                Match("hello", IntRange(1,6)),
                Match("5", IntRange(6,7))
            )))
            rakkoon.isComplete().shouldBe(true)
        }

        test("make sure all varargs pass when using overload") {
            val rakkoon = Rakkoon("555hello")
            val expectNone = rakkoon.nibble(fiveNibbler, fiveNibbler, helloNibbler)
            val expectResult = rakkoon.nibble(fiveNibbler, fiveNibbler, fiveNibbler, helloNibbler)
            expectNone.shouldBe(none())
            expectResult.shouldBe(Some(listOf(
                Match("5", IntRange(0,1)),
                Match("5", IntRange(1,2)),
                Match("5", IntRange(2,3)),
                Match("hello", IntRange(3,8))
            )))
        }
    }
}

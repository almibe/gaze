/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.*
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeInstanceOf

private val fiveNibbler = stringNibbler("5")
private val helloNibbler = stringNibbler("hello")
private val spaceNibbler = stringNibbler(" ")
private val worldNibbler = stringNibbler("world")

class StringPatternSpec : FunSpec() {
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

        test("hello work test") {
            val rakkoon = Rakkoon("hello world")
            rakkoon.nibble(helloNibbler).map { it.value }.shouldBe(Some("hello"))
            rakkoon.nibble(spaceNibbler).map { it.value }.shouldBe(Some(" "))
            rakkoon.nibble(worldNibbler).map { it.value }.shouldBe(Some("world"))
            rakkoon.isComplete().shouldBe(true)
        }
    }
}

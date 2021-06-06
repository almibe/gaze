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

class PeekSpec : FunSpec() {
    init {
        test("empty input") {
            val rakkoon = Rakkoon("")
            rakkoon.peek() shouldBe null
            rakkoon.nibble(fiveNibbler).shouldBeInstanceOf<None>()
            rakkoon.peek() shouldBe null
            rakkoon.isComplete().shouldBe(true)
        }

        test("single 5 input") {
            val rakkoon = Rakkoon("5")
            rakkoon.peek() shouldBe '5'
            rakkoon.nibble(fiveNibbler).shouldBe(Some(Match("5", IntRange(0,1))))
            rakkoon.peek() shouldBe null
            rakkoon.isComplete().shouldBe(true)
        }

        test("hello world test") {
            val rakkoon = Rakkoon("hello world")
            rakkoon.peek() shouldBe 'h'
            rakkoon.nibble(helloNibbler).shouldBe(Some(Match("hello", IntRange(0,5))))
            rakkoon.peek() shouldBe ' '
            rakkoon.nibble(spaceNibbler).shouldBe(Some(Match(" ", IntRange(5,6))))
            rakkoon.peek() shouldBe 'w'
            rakkoon.nibble(worldNibbler).shouldBe(Some(Match("world", IntRange(6,11))))
            rakkoon.peek() shouldBe null
            rakkoon.isComplete().shouldBe(true)
        }

        test("hello5world test") {
            val rakkoon = Rakkoon("hello5world")
            rakkoon.peek() shouldBe 'h'
            rakkoon.nibble(helloNibbler).shouldBe(Some(Match("hello", IntRange(0,5))))
            rakkoon.peek() shouldBe '5'
            rakkoon.nibble(fiveNibbler).shouldBe(Some(Match("5", IntRange(5,6))))
            rakkoon.peek() shouldBe 'w'
            rakkoon.nibble(worldNibbler).shouldBe(Some(Match("world", IntRange(6,11))))
            rakkoon.peek() shouldBe null
            rakkoon.isComplete().shouldBe(true)
        }
    }
}

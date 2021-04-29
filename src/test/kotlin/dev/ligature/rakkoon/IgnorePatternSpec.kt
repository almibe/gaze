/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

private val whiteSpace = regexPattern("[ \t]+".toRegex())
private val testPattern = stringPattern("test")
private val prefixRule = Rule(ignorePrefix(whiteSpace, testPattern), valueAction)
private val suffixRule = Rule(ignoreSuffix(whiteSpace, testPattern), valueAction)
private val surroundingRule = Rule(ignoreSurrounding(whiteSpace, testPattern), valueAction)

class IgnorePatternSpec : FunSpec() {
    init {
        test("support ignorePrefix") {
            val rakkoon = Rakkoon("\t  \t   test    \t   ")
            rakkoon.bite(prefixRule).shouldBe(Either.Right("test"))
            rakkoon.isComplete().shouldBe(false)
        }

        test("support ignoreSuffix") {
            val rakkoon = Rakkoon("test    \t   ")
            rakkoon.bite(suffixRule).shouldBe(Either.Right("test"))
            rakkoon.isComplete().shouldBe(true)
        }

        test("support ignoreSurrounding") {
            val rakkoon = Rakkoon("\t  \t   test    \t   ")
            rakkoon.bite(surroundingRule).shouldBe(Either.Right("test"))
            println(rakkoon.remainingText())
            println(rakkoon.currentOffset())
            rakkoon.isComplete().shouldBe(true)
        }

        test("support ignoreSurrounding with no ignore matches") {
            val rakkoon = Rakkoon("test")
            rakkoon.bite(surroundingRule).shouldBe(Either.Right("test"))
            println(rakkoon.remainingText())
            println(rakkoon.currentOffset())
            rakkoon.isComplete().shouldBe(true)
        }
    }
}

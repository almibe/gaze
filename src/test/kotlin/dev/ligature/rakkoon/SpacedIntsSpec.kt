/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.*
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeInstanceOf

private val rule = Rule(stringPattern("5"), toIntAction)

class SpacedIntsSpec : FunSpec() {
    init {
        test("empty input") {
            val rakkoon = Rakkoon("")
            rakkoon.bite(rule).shouldBeInstanceOf<Either.Left<RakkoonError>>()
        }

        test("single 5 input") {
            val rakkoon = Rakkoon("5")
            rakkoon.bite(rule).shouldBe(Either.Right(listOf(5)))
        }

        test("single 4 input") {
            val rakkoon = Rakkoon("277")
            rakkoon.bite(rule).shouldBe(Either.Right(listOf(277)))
        }

        test("repeating 5s input") {
            val rakkoon = Rakkoon("555 556")
            rakkoon.bite(rule).shouldBe(Either.Right(listOf(555, 556)))
        }

        test("repeating 5s input") {
            val rakkoon = Rakkoon("555\t556")
            rakkoon.bite(rule).shouldBe(Either.Right(listOf(555, 556)))
        }

        test("repeating 5s input") {
            val rakkoon = Rakkoon("555\n556")
            rakkoon.bite(rule).shouldBe(Either.Right(listOf(555, 556)))
        }

        test("repeating 5s input") {
            val rakkoon = Rakkoon("\t 555 \t\n \t\n   556\n\n\n44")
            rakkoon.bite(rule).shouldBe(Either.Right(listOf(555, 556, 44)))
        }
    }
}

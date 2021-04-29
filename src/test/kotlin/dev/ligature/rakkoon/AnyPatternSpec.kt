/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

private val truePattern = stringPattern("true")
private val falsePattern = stringPattern("false")
private val booleanRule = Rule(anyPattern(truePattern, falsePattern), valueAction)

class AnyPatternSpec : FunSpec() {
    init {
        test("test with multiple stringPatterns") {
            val rakkoon = Rakkoon("false")
            rakkoon.bite(booleanRule).shouldBe(Either.Right("false"))
            rakkoon.isComplete().shouldBe(true)
        }
    }
}

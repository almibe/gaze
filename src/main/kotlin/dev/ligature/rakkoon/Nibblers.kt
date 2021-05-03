/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.rakkoon

fun stringNibbler(toMatch: String) = object : Nibbler {
    var offset = 0

    override fun taste(char: Char?): NibState =
        if (offset == toMatch.length - 1 && toMatch[offset] == char) {
            Complete()
        } else if (offset < toMatch.length - 1 && toMatch[offset] == char) {
            offset++
            Next
        } else {
            Cancel
        }
}

//fun ignorePrefix(prefix: Pattern, pattern: Pattern, stop: Pattern? = null) = Pattern { input ->  //TODO add break pattern
//    val prefixRes = prefix.matches(input)
//    if (prefixRes.isEmpty()) {
//        pattern.matches(input)
//    } else {
//        val offset = prefixRes.getOrElse { TODO() }.endChar
//        val newInput = input.drop(offset)
//        when (val matchRes = pattern.matches(newInput)) {
//            is None -> none()
//            is Some -> Some(MatchInfo(IntRange(offset, offset + matchRes.value.endChar), offset + matchRes.value.endChar))
//        }
//    }
//}
//
//fun ignoreSuffix(suffix: Pattern, pattern: Pattern) = Pattern { input ->
//    val patternRes = pattern.matches(input)
//    if (patternRes.isEmpty()) {
//        none()
//    } else {
//        val offset = patternRes.getOrElse { TODO() }.endChar
//        val newInput = input.drop(offset)
//        when (val suffixRes = suffix.matches(newInput)) {
//            is None -> patternRes
//            is Some -> Some(MatchInfo(IntRange(0, offset), offset + suffixRes.value.endChar))
//        }
//    }
//}
//
//fun ignoreSurrounding(ignore: Pattern, pattern: Pattern) = Pattern { input ->
//    val ignorePrefixRes = ignorePrefix(ignore, pattern).matches(input)
//    if (ignorePrefixRes.isEmpty()) {
//        none()
//    } else {
//        val prefixMatchInfo = ignorePrefixRes.getOrElse { TODO() }
//        val newInput = input.subSequence(0, prefixMatchInfo.endChar)
//        val suffixMatch = ignore.matches(newInput)
//        if (suffixMatch.isEmpty()) {
//            ignorePrefixRes
//        } else {
//            val suffixMatchInfo = suffixMatch.getOrElse { TODO() }
//            Some(MatchInfo(prefixMatchInfo.match, prefixMatchInfo.endChar + suffixMatchInfo.endChar + 1))
//        }
//    }
//}
//
//fun anyPattern(vararg patterns: Pattern) = Pattern { input ->
//    var res: Option<MatchInfo> = none()
//    for(pattern in patterns) {
//        res = pattern.matches(input)
//        if (res.isNotEmpty()) {
//            break
//        }
//    }
//    res
//}
//
//fun regexPattern(pattern: Regex) = Pattern { input ->
//    val matchRes = pattern.find(input)
//    if (matchRes != null && matchRes.range.first == 0) {
//        Some(MatchInfo(IntRange(0, matchRes.range.last + 1), matchRes.range.last + 1))
//    } else {
//        none()
//    }
//}
//
//fun rangePattern(vararg ranges: CharRange) = Pattern { input ->
//    fun rangeMatches(char: Char): Boolean {
//        ranges.forEach {
//            if (it.contains(char)) {
//                return true
//            }
//        }
//        return false
//    }
//
//    var length = 0
//    while (input.length > length && rangeMatches(input[length])) {
//        length++
//    }
//    if (length > 0) {
//        Some(MatchInfo(IntRange(0, length), length))
//    } else {
//        none()
//    }
//}

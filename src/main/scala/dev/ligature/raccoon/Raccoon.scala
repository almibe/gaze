/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

package dev.ligature.raccoon

import scala.collection.immutable.NumericRange

sealed trait NibState
/**
 * The Cancel state means that this Nibbler didn't match and Rakkoon should jump back to its position before
 * starting this Nibbler.
 * The nibble method will also return None.
 */
case class Cancel() extends NibState
/**
 * The Complete state means that this Nibbler completed and Rakkoon should adjust its offset based on the adjust param.
 * A Some(Match) is returned by the nibble method.
 */
case class Complete(adjust: Int = 0) extends NibState

trait Nibbler {
  def taste(lookAhead: LookAhead): NibState
}

trait LookAhead {
  def peek(distance: Int = 0): Char
}

case class Match(value: String, range: NumericRange[Int])

class Raccoon(private var input: CharSequence) extends LookAhead {
  private var offset = 0

  override def peek(distance: Int): Option[Char] =
    if (offset + distance < input.length) Some(input.charAt(offset + distance))
    else None

  def bite(distance: Int) = {
    offset = offset + distance
  }

  def nibble(nibbler: Nibbler): Option[Match] = {
    val start = offset
    val res = nibbler.taste(this)
    res match {
      case Cancel() => {
        offset = start
        None
      }
      case Complete(adjust) => {
        offset = offset + adjust
        Some(Match(input.substring(start, offset), start to offset))
      }
    }
  }

  def nibble(nibblers: Nibbler*): Option[List[Match]] = {
    val resultList: MutableList[Match] = MutableList[Match]()
    val start = offset
    for (nibbler <- nibblers) {
      val res = nibble(nibbler)
      res match {
        case None => {
          offset = start
          return None
        }
        case Some => {
          resultList.add(res.value)
        }
      }
    }
    Some(resultList)
  }

  def currentOffset(): Int = offset

  def remainingText(): String = input.substring(offset)

  def isComplete(): Boolean = input.length <= offset
}

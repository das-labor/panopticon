/*
 * Panopticon - A libre disassembler (https://panopticon.re/)
 * Copyright (C) 2014-2016 Kai Michaelis
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

import QtQuick 2.3
import QtQuick.Controls 1.2 as Ctrl
import QtGraphicalEffects 1.0
import QtQuick.Layouts 1.1
import Panopticon 1.0

Monospace {
  property real nodeX: 0
  property real nodeY: 0
  property var nodeValue: null
  property string alt: nodeValue ? nodeValue.operandAlt[0] : ""
  property string kind: nodeValue ? nodeValue.operandKind[0] : ""
  property string ddata: nodeValue ? nodeValue.operandData[0] : ""
  property string display: nodeValue ? nodeValue.operandDisplay[0] : ""

  x: nodeX - width / 2
  y: nodeY + height / 3
  width: contentWidth
  height: Panopticon.basicBlockLineHeight
  verticalAlignment: Text.AlignVCenter
  font {
    capitalization: Font.AllLowercase
    pointSize: 11
  }
  color: alt == "" ? "black" : "#297f7a"
  text: display
}

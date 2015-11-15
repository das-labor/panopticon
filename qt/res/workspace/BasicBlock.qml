/*
 * Panopticon - A libre disassembler (https://panopticon.re/)
 * Copyright (C) 2014-2015 Kai Michaelis
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

import QtQuick 2.0
import Panopticon 1.0
import QtQuick.Controls 1.3


Item {
	id: bblock

	readonly property int xPadding: 5
	readonly property int yPadding: 2
	property var contents: [];
	property int opcodeWidth: 0;
	property int argsWidth: 0;

	width: childrenRect.width - childrenRect.x
	height: childrenRect.height
	opacity: 0.3

	Item {
		height: txt.childrenRect.height
		width: opcodeWidth + argsWidth + 6 + 2 * bblock.xPadding

		MouseArea {
			anchors.fill: parent
			cursorShape: Qt.ArrowCursor
			enabled: false
		}

		Column {
			id: txt
			x: bblock.xPadding

			Repeater {
				model: bblock.contents
				delegate: Item {
					width: comment.x + comment.width
					height: Math.max(opcode.height,Math.max(args.height,comment.height)) + 2 * bblock.yPadding

					Rectangle {
						color: (index % 2 == 0 ? "#e6f7f4" : "white")
						height: parent.height
						x: -1 * bblock.xPadding
						width: args.x + args.width + 2 * bblock.xPadding
					}

					Text {
						id: opcode
						text: modelData.opcode
						font.family: "Monospace"
						width: bblock.opcodeWidth
						height: contentHeight
						color: "black"
						y: bblock.yPadding

						Component.onCompleted: {
							bblock.opcodeWidth = Math.max(bblock.opcodeWidth,opcode.contentWidth)
						}
					}

					Text {
						id: args
						text: modelData.args
						font.family: "Monospace"
						width: bblock.argsWidth
						height: contentHeight
						anchors.left: opcode.right
						anchors.leftMargin: 6
						color: "black"
						y: bblock.yPadding

						Component.onCompleted: {
							bblock.argsWidth = Math.max(bblock.argsWidth,args.contentWidth)
						}
					}

					MouseArea {
						anchors.top: parent.top
						anchors.bottom: parent.bottom
						anchors.left: parent.left
						anchors.right: args.right

						onDoubleClicked: {
							comment.focus = true;
							comment.forceActiveFocus();
						}
					}

					Rectangle {
						anchors.fill: comment
						color: "#e8ebe7";
						visible: comment.activeFocus
						y: bblock.yPadding
					}

					TextEdit {
						id: comment
						font.family: "Sans"
						anchors.left: args.right
						anchors.leftMargin: text == "" && !activeFocus ? 0 : 20
						width: contentWidth
						height: contentHeight
						y: bblock.yPadding

						Keys.priority: Keys.BeforeItem
						Keys.onReturnPressed: {
							if (event.modifiers & Qt.ShiftModifier) {
								event.accepted = false
							} else {
								Panopticon.setComment(modelData.region,modelData.offset,comment.text.replace("\n","\\n"));
								comment.focus = false
							}
						}

						Component.onCompleted: {
							if (modelData.comment !== undefined) {
								text = modelData.comment
							}
						}

						MouseArea {
							anchors.fill: parent
							cursorShape: Qt.IBeamCursor
							enabled: false
						}
					}
				}
			}
		}

		Rectangle {
			anchors.fill: parent;
			color: "transparent";
			border.width: 1;
			radius: 3
			border.color: "#666666";
		}
	}
}

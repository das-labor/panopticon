/*
 * Panopticon - A libre disassembler (https://panopticon.re/)
 * Copyright (C) 2014,2015,2017 Panopticon authors
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
import QtQuick.Controls.Styles 1.2 as Style
import QtQuick.Layouts 1.1

import Panopticon 1.0
import ".."

Item {
	id: basicBlock

	property int nodeId: -1;
	property var code: [];

	signal startComment(int x,int y,string address,string comment)
	signal displayPreview(rect bb)
	signal showControlFlowGraph(string uuid)

	width: childrenRect.width + childrenRect.x
	height: childrenRect.height + childrenRect.y

	Component.onCompleted: {
		console.log("completed bb " + nodeId.toString())
	}

	Rectangle {
		id: basicBlockRect

		property int commentRowStart: 0
		property int opcodeRowStart: 0

		x: Panopticon.basicBlockMargin
		y: Panopticon.basicBlockMargin
		width: commentRowStart - opcodeRowStart
		height: basicBlockGrid.height + 8 + 3
		color: "#ffffff"
		radius: 2
		border {
			width: .7
			color: "#939393"
		}

	MouseArea {
			id: mouseArea
			anchors.left: basicBlockGrid.left
			anchors.top: parent.top
			anchors.bottom: parent.bottom
			anchors.right: basicBlockRect.right
			hoverEnabled: true
		}

		Item {
			id: editOverlay

			property string variable: ""

			function open(x,y,v) {
				editOverlay.x = x;
				editOverlay.y = y;
				editOverlay.visible = true
				editOverlayField.focus = true
				editOverlay.variable = v;
			}

			function close() {
				editOverlayField.focus = false
				editOverlay.visible = false;
				editOverlayField.text = "";
			}

			z: 2
			visible: false

			Canvas {
				id: editOverlayTip

				x: parent.width - width / 2
				y: 0
				width: 20
				height: 12

				onPaint: {
					var ctx = editOverlayTip.getContext('2d');

					ctx.fillStyle = "#a9a9a9";

					ctx.beginPath();
					ctx.moveTo(0,editOverlayTip.height);
					ctx.lineTo(editOverlayTip.width / 2,0);
					ctx.lineTo(editOverlayTip.width,editOverlayTip.height);
					ctx.closePath();
					ctx.fill();
				}
			}

			Rectangle {
				anchors.top: editOverlayTip.bottom
				anchors.horizontalCenter: parent.left
				width: editOverlayField.width + 10
				height: editOverlayField.height + 10
				color: "#a9a9a9"
				radius: 2

				Ctrl.TextField {
					id: editOverlayField

					x: 5
					y: 5

					/*style: Style.TextFieldStyle {
						background: Rectangle { color: "white"; opacity: .9; border { width: 1 } }
					}*/

					onAccepted: {
						Panopticon.set_value_for(editOverlay.variable,editOverlayField.text);
						//editOverlay.close();
					}

					onEditingFinished: { editOverlay.close() }
				}
			}
		}

		GridLayout {
			id: basicBlockGrid
			columnSpacing: 0
			rowSpacing: 0
			x: Panopticon.basicBlockPadding
			y: Panopticon.basicBlockPadding

			// address
			Repeater {
				model: code
				delegate: Monospace {
					Layout.column: 0
					Layout.row: index
					Layout.rightMargin: 20
					Layout.maximumHeight: Panopticon.basicBlockLineHeight
					Layout.preferredHeight: Panopticon.basicBlockLineHeight

					Behavior on opacity { NumberAnimation { duration: 150 } }

					text: "0x" + modelData.offset
					font {
						pointSize: 10
					}
					color: "#b4b4b4"
					opacity: (mouseArea.containsMouse ? 1. : 0)
				}
			}

			// opcode
			Repeater {
				model: code
				delegate: Monospace {
					Layout.column: 1
					Layout.row: index
					Layout.rightMargin: 26
					Layout.maximumHeight: Panopticon.basicBlockLineHeight
					Layout.preferredHeight: Panopticon.basicBlockLineHeight

					text: modelData.opcode
					font {
						pointSize: 10
					}

					onXChanged: {
						basicBlockGrid.x = -x + 8
						basicBlockRect.opcodeRowStart = Math.max(basicBlockRect.opcodeRowStart,x - 8)
					}
				}
			}

			// arguments
			Repeater {
				model: code
				delegate: RowLayout {
					Layout.column: 2
					Layout.row: index
					Layout.rightMargin: 15
					Layout.maximumHeight: Panopticon.basicBlockLineHeight
					Layout.preferredHeight: Panopticon.basicBlockLineHeight
					spacing: 0

					Repeater {
						model: modelData.args
						delegate: Monospace {
							id: operandLabel
							text: modelData.display
							font {
						 		capitalization: Font.AllLowercase
								pointSize: 10
							}

							MouseArea {
								anchors.fill: parent
								hoverEnabled: true
								visible: modelData.kind == "variable"
								cursorShape: Qt.IBeamCursor

								onExited: {
									editOverlay.close()
								}

								onClicked: {
									var pnt = parent.mapToItem(editOverlay.parent,x + width / 2,y + height);
									editOverlay.open(pnt.x,pnt.y,modelData.data)
								}
							}

							MouseArea {
								anchors.fill: parent
								hoverEnabled: true
								visible: modelData.kind == "function"
								cursorShape: Qt.PointingHandCursor
								onClicked: {
									Panopticon.display_preview_for(modelData.data)
									var bb = mapToItem(basicBlock,
									                   operandLabel.x,
																		 operandLabel.y,
																		 operandLabel.width,
																		 operandLabel.height);
									displayPreview(bb)
								}

								onDoubleClicked: {
									controlflow.showControlFlowGraph(modelData.data)
								}
							}
						}
					}
				}
			}

			// comments
			Repeater {
				model: code
				delegate: Item {
					Layout.column: 3
					Layout.row: index
					Layout.maximumHeight: Panopticon.basicBlockLineHeight
					Layout.preferredHeight: Panopticon.basicBlockLineHeight
					Layout.maximumWidth: Panopticon.basicBlockCommentWidth

					onXChanged: {
						basicBlockRect.commentRowStart = Math.max(basicBlockRect.commentRowStart,x - 8)
					}

					z: (mouseArea.containsMouse ? 2 : 1)

					Rectangle {
						id: commentBackground
						width: commentLabel.contentWidth
						height: commentLabel.contentHeight
						color: "#fafafa"
						visible: mouseArea.containsMouse
						z: (mouseArea.containsMouse || commentMouseArea.containsMouse ? 2 : 1)
					}

					Label {
						id: commentLabel
						height: Panopticon.basicBlockLineHeight
						width: Panopticon.basicBlockCommentWidth
						clip: !commentMouseArea.containsMouse
						verticalAlignment: Text.AlignHCenter
						z: (mouseArea.containsMouse || commentMouseArea.containsMouse ? 3 : 1)
						text: {
							if(modelData.comment === "") {
								return "+ add comment";
							} else {
								return modelData.comment;
							}
						}
						font {
							pointSize: 12
							italic: true
						}
						color: (modelData.comment === "" ? "#cdcdcd" : "black")
						opacity: (modelData.comment !== "" || commentMouseArea.containsMouse ? 1. : 0)
						elide: ((mouseArea.containsMouse && !new String(modelData.comment).search("\n")) || commentMouseArea.containsMouse ? Text.ElideNone : Text.ElideRight)
					}

					MouseArea {
						id: commentMouseArea
						anchors.fill: commentLabel
						hoverEnabled: true
						cursorShape: Qt.PointingHandCursor

						onClicked: {
							var pnt = mapToItem(basicBlock,commentLabel.x,commentLabel.y + commentLabel.height / 2);
							startComment(pnt.x,pnt.y,modelData.offset,modelData.comment)
						}
					}
				}
			}
		}

	}
}

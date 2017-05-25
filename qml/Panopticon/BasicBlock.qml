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
import QtGraphicalEffects 1.0
import Panopticon 1.0

Item {
	id: basicBlock

	property real nodeX: 0;
	property real nodeY: 0;
	property int nodeId: -1;
	property var code: [];
	property string uuid: "";

	signal startComment(int x,int y,string address,string comment)
	signal displayPreview(rect bb,string uuid)
	signal showControlFlowGraph(string newUuid)

	x: nodeX - width / 2 - basicBlockRect.x / 2
	y: nodeY - height / 2 - basicBlockRect.y / 2
	width: basicBlockRect.width + basicBlockRect.x
	height: basicBlockRect.height + basicBlockRect.y

	Rectangle {
		id: basicBlockRect

		x: addressColumn.width - Panopticon.basicBlockMargin
		y: -Panopticon.basicBlockMargin
		width: argumentColumn.x + argumentColumn.width - opcodeColumn.x + 2*Panopticon.basicBlockMargin
		height: basicBlockGrid.height + 2*Panopticon.basicBlockMargin
		color: "#ffffff"
		radius: 2
		border {
			width: .7
			color: "#939393"
		}

		MouseArea {
			property int hoveredRow: -1

			id: mouseArea
			anchors.fill: parent
			hoverEnabled: true

			function updateHoveredRow(y) {
				if(y < 0 || y > mouseArea.height) {
					mouseArea.hoveredRow = -1;
				} else {
					mouseArea.hoveredRow = Math.floor(y / Panopticon.basicBlockLineHeight);
				}
			}

			onExited: {
				mouseArea.hoveredRow = -1;
			}

			onPositionChanged: {
				updateHoveredRow(mouseY)
			}
		}
	}

	EditPopover {
		id: editOverlay
	}

	Row {
    id: basicBlockGrid

		// address
		Column {
			id: addressColumn
			Repeater {
				model: code
				delegate: Monospace {
					width: contentWidth + 20
					height: Panopticon.basicBlockLineHeight
					verticalAlignment: Text.AlignVCenter

					Behavior on opacity { NumberAnimation { duration: 150 } }

					text: "0x" + modelData.offset.toString(16)
					font {
						pointSize: 10
					}
					color: "#b4b4b4"
					opacity: (mouseArea.containsMouse ? 1. : 0)
				}
			}
		}

		Column {
			id: opcodeColumn
			Repeater {
				model: code
				delegate: Monospace {
					text: modelData.opcode
					width: contentWidth + 26
					height: Panopticon.basicBlockLineHeight
					verticalAlignment: Text.AlignVCenter
					font {
						pointSize: 10
					}
				}
			}
		}

		Column {
			id: argumentColumn
			Repeater {
				model: code
				delegate: Row {
					id: argumentRow
					property var argumentModel: modelData

					Item {
						id: padder
						height: Panopticon.basicBlockLineHeight
						width: 1
						visible: modelData.operandDisplay.length == 0
					}

					Repeater {
						model: modelData.operandDisplay
						delegate: Monospace {
							property string alt: argumentRow.argumentModel ? argumentRow.argumentModel.operandAlt[index] : ""
							property string kind: argumentRow.argumentModel ? argumentRow.argumentModel.operandKind[index] : ""
							property string ddata: argumentRow.argumentModel ? argumentRow.argumentModel.operandData[index] : ""

							id: operandLabel
							width: contentWidth
							height: Panopticon.basicBlockLineHeight
							verticalAlignment: Text.AlignVCenter
							font {
						 		capitalization: Font.AllLowercase
								pointSize: 10
							}
							color: alt == "" ? "black" : "#297f7a"
							text: modelData

							MouseArea {
								id: operandMouseArea
								anchors.fill: parent
								hoverEnabled: true
								visible: kind == "variable"
								cursorShape: Qt.IBeamCursor

								onExited: {
									var y = mapToItem(mouseArea,0,mouseY).y;
									mouseArea.updateHoveredRow(y);
									editOverlay.close()
								}

								onEntered: {
									var y = mapToItem(mouseArea,0,mouseY).y;
									mouseArea.updateHoveredRow(y);
								}

								onPositionChanged: {
									var y = mapToItem(mouseArea,0,mouseY).y;
									mouseArea.updateHoveredRow(y);
								}

								onClicked: {
									var pnt = parent.mapToItem(editOverlay.parent,x + width / 2,y + height);
									editOverlay.open(pnt.x,pnt.y + 3,ddata,uuid)
								}
							}

							MouseArea {
								anchors.fill: parent
								hoverEnabled: true
								visible: kind == "function"
								cursorShape: Qt.PointingHandCursor
								onClicked: {
									var bb = mapToItem(basicBlock,
									                   operandLabel.x,
																		 operandLabel.y,
																		 operandLabel.width,
																		 operandLabel.height);
									displayPreview(bb,ddata)
								}

								onExited: {
									var y = mapToItem(mouseArea,0,mouseY).y;
									mouseArea.updateHoveredRow(y);
								}

								onEntered: {
									var y = mapToItem(mouseArea,0,mouseY).y;
									mouseArea.updateHoveredRow(y);
								}

								onPositionChanged: {
									var y = mapToItem(mouseArea,0,mouseY).y;
									mouseArea.updateHoveredRow(y);
								}

								onDoubleClicked: {
									basicBlock.showControlFlowGraph(ddata)
								}
							}
						}
					}
				}
			}
		}

		Column {
			id: commentColumn

			// comments
			Repeater {
				model: code
				delegate: Item {
					x: 2*Panopticon.basicBlockMargin
					z: (mouseArea.containsMouse ? 2 : 1)
					height: Panopticon.basicBlockLineHeight
					width: Panopticon.basicBlockCommentWidth

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
						elide: (modelData.comment !== undefined && ((mouseArea.containsMouse && !new String(modelData.comment).search("\n")) || commentMouseArea.containsMouse ? Text.ElideNone : Text.ElideRight))
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

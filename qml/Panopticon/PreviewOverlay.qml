import QtQuick 2.4
import QtQuick.Controls 1.3 as Ctrl
import QtQuick.Layouts 1.1
import QtQuick.Controls.Styles 1.2
import Panopticon 1.0

import "."

MouseArea {
	id: overlay

	property rect boundingBox: "0,0,0x0"
	property var code: []
	property string uuid: ""

	signal showControlFlowGraph(string uuid)

	function open(bb,uuid) {
		overlay.boundingBox = bb;
		overlay.visible = true
		overlay.uuid = uuid;
	}

	x: overlay.boundingBox.x
	y: overlay.boundingBox.y
	width: overlay.boundingBox.width
	height: overlay.boundingBox.height
	hoverEnabled: true

	onExited: {
		overlay.visible = false
	}

	onWheel: {
		overlay.visible = false
	}

	onClicked: {
		overlay.visible = false
		if(overlay.uuid != "") {
			showControlFlowGraph(overlay.uuid);
		}
	}

	Rectangle {
		id: overlayBox

		anchors.left: overlayTip.right
		anchors.top: overlayTip.top
		anchors.topMargin: -5
		anchors.leftMargin: -2

		width: basicBlockGrid.width + basicBlockGrid.x
		height: basicBlockGrid.height + basicBlockGrid.y + Panopticon.basicBlockPadding
		color: "#fafafa"
		radius: 2
		border {
			width: .7
			color: "#d8dae4"
		}

		Label {
			anchors.left: parent.left
			anchors.leftMargin: Panopticon.basicBlockPadding
			anchors.bottom: parent.top
			anchors.bottomMargin: Panopticon.basicBlockPadding

			text: overlay.code.length > 0 ? "0x" + overlay.code[0].offset.toString(16) : ""
			font {
				pointSize: 12
			}
			color: "#d8dae4"
		}

		Row {
			id: basicBlockGrid

			x: Panopticon.basicBlockMargin
			y: Panopticon.basicBlockPadding

			Column {
				id: opcodeColumn
				Repeater {
					model: overlay.code
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
					model: overlay.code
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
							}
						}
					}
				}
			}
		}
	}

	Canvas {
		id: overlayTip

		x: parent.width + 3
		y: parent.height / 2 - height / 2
		width: 6
		height: 20

		onPaint: {
			var ctx = overlayTip.getContext('2d');

			ctx.fillStyle = "#fafafa";
			ctx.strokeStyle = "#d8dae4";
			ctx.lineWidth = 1;

			ctx.beginPath();
			ctx.moveTo(overlayTip.width,0);
			ctx.lineTo(0,(overlayTip.height - 1)/ 2);
			ctx.lineTo(overlayTip.width,overlayTip.height - 1);
			ctx.closePath();
			ctx.fill();

			ctx.beginPath();
			ctx.moveTo(overlayTip.width - 1,0);
			ctx.lineTo(0,(overlayTip.height - 1)/ 2);
			ctx.lineTo(overlayTip.width -1,overlayTip.height - 1);
			ctx.stroke();
		}
	}
}

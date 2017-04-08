import QtQuick 2.4
import QtQuick.Controls 1.3 as Ctrl
import QtQuick.Layouts 1.1
import QtQuick.Controls.Styles 1.2

MouseArea {
	id: overlay

	property rect boundingBox: "0,0,0x0"

	signal showControlFlowGraph(string uuid)

	function open(bb) {
		overlay.boundingBox = bb;
		overlay.visible = true
		overlayBox.code = JSON.parse(Panopticon.previewNode);
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
		showControlFlowGraph(Panopticon.previewFunction);
	}

	Rectangle {
		id: overlayBox

		property var code: [];

		anchors.left: overlayTip.right
		anchors.top: overlayTip.top
		anchors.topMargin: -5
		anchors.leftMargin: -2

		width: basicBlockGrid.width + basicBlockGrid.x
		height: basicBlockGrid.height + basicBlockGrid.y + Panopticon.basicBlockPadding
		color: "#ffffff"
		radius: 2
		border {
			width: .7
			color: "#939393"
		}

		Label {
			anchors.left: parent.left
			anchors.leftMargin: Panopticon.basicBlockPadding
			anchors.bottom: parent.top
			anchors.bottomMargin: Panopticon.basicBlockPadding

			text: overlayBox.code.length > 0 ? "0x" + overlayBox.code[0].offset : ""
			font {
				pointSize: 12
			}
			color: "#939393"
		}

		GridLayout {
			id: basicBlockGrid

			x: Panopticon.basicBlockMargin
			y: Panopticon.basicBlockPadding
			columnSpacing: 0
			rowSpacing: 0

			// opcode
			Repeater {
				model: overlayBox.code
				delegate: Monospace {
					Layout.column: 0
					Layout.row: index
					Layout.rightMargin: 26
					Layout.maximumHeight: Panopticon.basicBlockLineHeight
					Layout.preferredHeight: Panopticon.basicBlockLineHeight

					text: modelData.opcode
					font {
						pointSize: 10
					}
				}
			}

			// arguments
			Repeater {
				model: overlayBox.code
				delegate: RowLayout {
					Layout.column: 1
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
		width: 20
		height: 20

		onPaint: {
			var ctx = overlayTip.getContext('2d');

			ctx.fillStyle = "white";
			ctx.strokeStyle = "#939393";
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

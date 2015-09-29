import QtQuick 2.0
import Panopticon 1.0
import QtQuick.Controls 1.3


Item {
	id: bblock

	readonly property int padding: 5
	property var contents: [];
	property int opcodeWidth: 0;
	property int argsWidth: 0;

	width: childrenRect.width
	height: childrenRect.height

	Rectangle {
		height: txt.childrenRect.height + 2 * bblock.padding
		width: opcodeWidth + argsWidth + 6 + 2 * bblock.padding
		color: "steelblue";
		border.width: 1;
		border.color: "black";

		MouseArea {
			anchors.fill: parent
			cursorShape: Qt.ArrowCursor
			enabled: false
		}

		Column {
			id: txt
			x: bblock.padding
			y: bblock.padding

			Repeater {
				model: bblock.contents
				delegate: Item {
					width: comment.x + comment.width
					height: Math.max(opcode.height,Math.max(args.height,comment.height))

					Text {
						id: opcode
						text: modelData.opcode
						font.family: "Monospace"
						width: bblock.opcodeWidth
						height: contentHeight

						Component.onCompleted: {
							bblock.opcodeWidth = Math.max(bblock.opcodeWidth,opcode.contentWidth)
						}
					}

					Text {
						id: args
						text: modelData.args.join(", ")
						font.family: "Monospace"
						width: bblock.argsWidth
						height: contentHeight
						anchors.left: opcode.right
						anchors.leftMargin: 6

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
						color: "green";
						visible: comment.activeFocus
					}

					TextEdit {
						id: comment
						font.family: "Sans"
						anchors.left: args.right
						anchors.leftMargin: text == "" && !activeFocus ? 0 : 20
						width: contentWidth
						height: contentHeight

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
	}
}

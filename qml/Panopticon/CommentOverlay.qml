import QtQuick 2.4
import QtQuick.Controls 1.3 as Ctrl
import QtQuick.Layouts 1.1
import QtQuick.Controls.Styles 1.2
import Panopticon 1.0

import "."

MouseArea {
	id: overlay

	property int tipX: 0
	property int tipY: 0
	property string address: ""

	signal accepted;
	signal canceled;

	function open(tipX,tipY,address,comment) {
		overlay.tipX = tipX;
		overlay.tipY = tipY;
		overlay.address = address;
		overlayComment.text = comment;
		overlay.visible = true
	}

	onAccepted: {
		Panopticon.commentOn(overlay.address,overlayComment.text)
		overlay.visible = false
		overlayComment.text = ""
	}

	onCanceled: {
		overlay.visible = false
		overlayComment.text = ""
	}

	visible: false
	hoverEnabled: true

	Rectangle {
		anchors.fill: parent
		color: "black"
		opacity: .8
	}

	Canvas {
		id: overlayTip
		x: overlay.tipX
		y: overlay.tipY - height / 2
		width: 20
		height: 20
		visible: false

		onPaint: {
			var ctx = overlayTip.getContext('2d');

			ctx.fillStyle = "white";
			ctx.moveTo(overlayTip.width,0);
			ctx.lineTo(0,(overlayTip.height - 1)/ 2);
			ctx.lineTo(overlayTip.width,overlayTip.height - 1);
			ctx.closePath();
			ctx.fill();
		}
	}

	Rectangle {
		id: overlayBox

		anchors.centerIn: parent
		//anchors.left: overlayTip.right
		//y: overlay.tipY - overlayTip.height - 5

		width: 350
		height: 300
		radius: 2
		color: "#fafafa"

		MouseArea {
			anchors.fill: parent
			onClicked: {}
		}

		Image {
			id: overlayClose

			anchors.right: parent.right
			anchors.top: parent.top
			anchors.topMargin: 12
			anchors.rightMargin: 12
			width: 22
			height: 22
			//Layout.preferredHeight: 22
			//Layout.alignment: Qt.AlignRight

			source: "../icons/cross.svg"
			fillMode: Image.PreserveAspectFit
			mipmap: true

			MouseArea {
				anchors.fill: parent
				onClicked: {
					overlay.canceled()
				}
			}
		}

		ColumnLayout {
			anchors.fill: parent
			anchors.margins: 20
			spacing: 30

			RowLayout {
				Layout.row: 0

				Label {
					Layout.fillWidth: true

					font {
						pointSize: 15
						weight: Font.DemiBold
						family: "Source Sans Pro"
					}
					text: "Add Comment"
				}
			}

			Ctrl.TextArea {
				id: overlayComment

				Layout.row: 1
				Layout.fillHeight: true
				Layout.fillWidth: true

				Keys.onReturnPressed: {
					if(event.modifiers & Qt.ControlModifier) {
						overlay.accepted()
					} else {
						event.accepted = false
					}
				}
				Keys.onEscapePressed: {
					overlay.canceled()
				}

				font {
					pointSize: 12
					family: "Source Sans Pro"
				}
				focus: overlay.visible
			}

			Ctrl.Label {
				Layout.row: 2
				Layout.alignment: Qt.AlignRight

				function getPlatformCommentShortcut() {
					if (Qt.platform.os == "osx") {
						return "Comment (⌘+Return)";
					} else {
						return "Comment (Ctrl+Return)";
					}
				}

				text: getPlatformCommentShortcut()
				horizontalAlignment: Text.AlignRight
				font {
					family: "Source Sans Pro"
					pointSize: 12
					underline: mousearea.containsMouse
				}
				color: "#4a95e2"

				MouseArea {
					id: mousearea
					anchors.fill: parent
					hoverEnabled: true
					cursorShape: (containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor)
					onClicked: {
						overlay.accepted()
					}
				}
			}
		}
	}

	onClicked: {
		overlay.canceled()
	}

	onWheel: {}
}

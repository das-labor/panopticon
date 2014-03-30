import QtQuick 2.0

Item {
	property var icon: null
	property string title: "to title"
	property string description: "no description"

	signal activated()

	height: childrenRect.height
	width: childrenRect.width

	MouseArea {
		anchors.fill: rootItem
		hoverEnabled: true

		onEntered: rootItem.state = "HOVER"
		onExited: rootItem.state = "DEFAULT"
		onPressed: { activated() }
	}

	Rectangle {
		id: rootItem
		height: childrenRect.height + 20
		width: childrenRect.width + 20
		border.width: 2
		state: "DEFAULT"

		states: [
			State {
				name: "DEFAULT"
        PropertyChanges { target: rootItem.border; color: "#ffffff" }
			},
			State {
				name: "HOVER"
        PropertyChanges { target: rootItem.border; color: "#ff0000"}
			}
		]

		transitions: [
			Transition {
				from: "DEFAULT"
				to: "HOVER"

				ColorAnimation {
					target: rootItem.border
					duration: 300
					easing.type: Easing.OutQuad
				}
			},
			Transition {
				from: "HOVER"
				to: "DEFAULT"

				ColorAnimation {
					target: rootItem.border
					duration: 300
					easing.type: Easing.OutQuad
				}
			}
		]

		Rectangle {
			id: iconItem
			x: 10; y: 10
			width: 90; height: 90
			color: icon
		}

		Text {
			id: titleItem
			anchors.left: iconItem.right
			anchors.leftMargin: 20
			y: 20

			text: title
			font { pointSize: 12; family: "Sans"; weight: Font.Bold }
		}

		Text {
			anchors.left: iconItem.right
			anchors.leftMargin: 20
			anchors.top: titleItem.bottom
			anchors.topMargin: 20

			text: description
			font { pointSize: 8; family: "Sans" }
		}
	}
}

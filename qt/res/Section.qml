import QtQuick 1.0

Rectangle {
	id: main
	property int rows: 0
	property string name: "(noname)"

	width: 450
	height: { rows * 10 }

	color: "#1155aa"
	border.color: "black"
	border.width: 2

	Text {
		id: name_element
		text: { parent.name }
		color: "black"
		font.pointSize: 8
		transform: Rotation {
			origin.x: { name_element.width / 2 }
			origin.y: { name_element.height / 2}
			angle: 45
		}
		anchors.horizontalCenter: main.horizontalCenter
		anchors.verticalCenter: main.verticalCenter
	}
}

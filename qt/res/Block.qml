import QtQuick 2.0
import Panopticon 1.0

Item {
	width: childrenRect.width
	height: childrenRect.height

	Rectangle {
		id: rect
		width: 100
		height: 40
		color: true ? "green" : "yellow"

		Text {
			anchors.centerIn: parent
			text: "name"
		}
	}
}

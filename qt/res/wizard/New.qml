import QtQuick 2.0
import "../"

Item {
	Loader {
		id: loader
		height: parent.height
		width: parent.width
		anchors.left: root.right
	}

	Page {
		id: root
		anchors.fill: parent
		primaryTitle: "New Session"
		secondaryTitle: "Select file"
		primaryAction: "Analyze"

		Behavior on x {
			NumberAnimation { duration: 300 }
		}

		Item {
			anchors.fill: parent

			Rectangle {
				id: filepicker
				x: 50
				y: 100

				height: root.height - 200
				width: root.width / 2 - 50
				color: "#999"
			}

			Rectangle {
				x: filepicker.x + filepicker.width + 50
				y: 100

				height: root.height * 0.6666666 - 200
				width: root.width / 2 - 100
				color: "#aaa"
			}
		}
	}
}

import QtQuick 2.0
import Panopticon 1.0
import "../"

Item {
	Loader {
		id: loader
		height: parent.height
		width: parent.width
		anchors.left: root.right
	}

	Connections {
		target: loader.item
		onBack: {
			root.x = 0
		}
	}

	Page {
		id: root
		anchors.fill: parent
		primaryTitle: "Panopticon"
		secondaryTitle: "Version 0.9"
		primaryAction: "Quit"
		onPrimary: Qt.quit()

		Behavior on x {
			NumberAnimation { duration: 300 }
		}

		Item {
			height: childrenRect.height
			width: childrenRect.width
			anchors.centerIn: parent

			Column {
				spacing: 150

				Option {
					icon: "blue"
					title: "New session"
					description: "Pick a file and analyze it in a new session."

					onActivated: {
						root.anchors.fill = undefined
						root.x = -1 * root.width
						loader.source = "New.qml"
					}
				}

				Option {
					icon: "green"
					title: "Continue session"
					description: "Open a session saved previously and continue analysis."

					onActivated: {
						root.anchors.fill = undefined
						root.x = -1 * root.width
						loader.source = "Continue.qml"
					}
				}
			}
		}

		Text {
			anchors.bottom: root.bottom
			anchors.right: root.right

			text: "Built " + Panopticon.buildDate
			font.pointSize: 10
		}
	}
}

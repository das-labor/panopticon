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
		primaryTitle: "Panopticon"
		secondaryTitle: "Version 0.9"
		primaryAction: "Quit"

		Behavior on x {
			NumberAnimation { duration: 300 }
		}

		Item {
			height: childrenRect.height
			width: childrenRect.width
			anchors.centerIn: parent

			Column {
				spacing: 100

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

				Option {
					icon: "red"
					title: "Quit Panopticon"
					description: "Close all open sessions and exit the application."

					onActivated: { Qt.quit() }
				}
			}
		}
	}
}

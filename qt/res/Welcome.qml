import QtQuick 2.0

Item {
	id: root
	anchors.fill: parent

	Rectangle {
		border.color: "red"
		anchors.fill: parent
	}

	Column {
		spacing: 50
		anchors.top: parent.top

		Option {
			icon: "blue"
			title: "New session"
			description: "Pick a file and analyze it in a new session."

			MouseArea {
				anchors.fill: parent

				onClicked: {
					info.x = 100
				}
			}
		}

		Option {
			icon: "green"
			title: "Continue session"
			description: "Open a session saved previously and continue analysis."
		}

		Option {
			icon: "red"
			title: "Quit Panopticon"
			description: "Close all open sessions and exit the application."
		}
	}

	Rectangle {
		x: { parent.width - 400 }
		y: { parent.height - 400 }
		height: 400
		width: 400
		z: -1

		color: "gray"
	}

	Rectangle {
		id: info
		x: { root.x + root.width }
		y: 0

		height: 200; width: 200
		color: "gray"

		Behavior on x { NumberAnimation { duration: 1000; easing.type: Easing.OutQuad } }
	}
}

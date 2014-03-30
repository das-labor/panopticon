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
		primaryTitle: "Continue Session"
		secondaryTitle: "Recent sessions"
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

				Repeater {
					model: 3
					delegate: Item {
						height: 80
						width: 300

						MouseArea {
							anchors.fill: parent

							onPressed: {
								root.anchors.fill = undefined
								root.x = -1 * root.width
								loader.source = "../workspace/Workspace.qml"
							}
						}

						Rectangle {
							color: "green"
							anchors.fill: parent
						}
					}
				}
			}
		}
	}
}

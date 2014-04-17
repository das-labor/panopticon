import QtQuick 2.0
import Panopticon 1.0
import "../"
import Qt.labs.settings 1.0

Item {
	signal back()

	Loader {
		property variant session: null

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
		primaryAction: "Back"

		Behavior on x {
			NumberAnimation { duration: 300 }
		}

		Settings {
			id: settings
			property string recent: ""
		}

		onPrimary: {
			back()
		}

		Item {
			height: childrenRect.height
			width: childrenRect.width
			anchors.centerIn: parent

			Column {
				spacing: 100

				Repeater {
					model: settings.recent.split(",").filter(function(a) { return a.length > 0 })
					delegate: Item {
						height: 80
						width: 300

						Text {
							anchors.centerIn: parent
							text: modelData
						}

						MouseArea {
							anchors.fill: parent

							onPressed: {
								root.anchors.fill = undefined
								root.x = -1 * root.width
								loader.session = Panopticon.openSession("old.panop")
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

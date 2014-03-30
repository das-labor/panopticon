import QtQuick 2.0
import QtQuick.Controls 1.0
import "../"
import Qt.labs.folderlistmodel 1.0

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

			ScrollView {
				id: filepicker
				x: 50
				y: 100

				height: root.height - 200
				width: root.width / 2 - 50

				frameVisible: true

				ListView {
					id: view
					anchors.fill: parent
					focus: true
					highlight: Rectangle { color: "lightsteelblue"; radius: 5 }

					model: FolderListModel {
						id: filepicker_model
						showDirsFirst: true
						showDotAndDotDot: true
					}

					delegate: Component { Rectangle {
							width: label.width
							height: label.height

							Text {
								id: label
								text: fileName
								anchors.horizontalCenter: parent.horizontalCenter
							}

							MouseArea {
								anchors.fill: parent

								onClicked: {
									if(filepicker_model.isFolder(index)) {
										filepicker_model.folder += "/" + fileName
										console.log(fileName)
									} else {
										root.anchors.fill = undefined
										root.x = -1 * root.width
										loader.source = "../workspace/Workspace.qml"
									}
								}
							}
						}
					}
				}
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

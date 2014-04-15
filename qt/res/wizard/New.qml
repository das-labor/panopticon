import QtQuick 2.0
import QtQuick.Controls 1.0
import "../"
import "../workspace"
import Qt.labs.folderlistmodel 1.0
import Panopticon 1.0
import Qt.labs.settings 1.0

Item {
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
		primaryTitle: "New Session"
		secondaryTitle: "Select file"
		primaryAction: "Analyze"

		Behavior on x {
			NumberAnimation { duration: 300 }
		}

		Settings {
			id: settings
			property string recent: ""
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

					model: FolderListModel {
						id: filepicker_model
						showDirsFirst: true
						showDotAndDotDot: true
					}

					delegate: Component {
						Rectangle {
							width: label.width
							height: label.height

							Text {
								id: label
								text: fileName
								anchors.horizontalCenter: parent.horizontalCenter
							}

							MouseArea {
								property variant sess: null

								id: mouseArea
								anchors.fill: parent

								Component {
									id: comp

									Workspace {
										session: mouseArea.sess
									}
								}

								onClicked: {
									if(filepicker_model.isFolder(index)) {
										filepicker_model.folder += "/" + fileName
									} else {
										root.anchors.fill = undefined
										root.x = -1 * root.width

										var path = Qt.resolvedUrl(filepicker_model.folder + "/" + fileName).toString()
										var mru = settings.recent.split(",").filter(function(a) { return a.length > 0 }).filter(function(a) { return a !== path })

										mru.unshift(path)
										settings.recent = mru.slice(0,6).join(",")

										console.log(settings.recent)
										mouseArea.sess = Panopticon.newSession(path)
										loader.sourceComponent = comp
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

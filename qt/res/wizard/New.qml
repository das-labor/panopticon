import QtQuick 2.0
import QtQuick.Controls 1.0
import "../"
import "../workspace"
import Qt.labs.folderlistmodel 1.0
import Panopticon 1.0
import Qt.labs.settings 1.0

Item {
	id: root


	Item {
		id: menu

		property bool alignLeft: true
		property color primaryColor: "#34495e"
		property color secondaryColor: "#1bbc9b"

		Rectangle {
			anchors.fill: parent
			color: menu.primaryColor
		}

		Column {
			y: 5
			width: parent.width
			spacing: 4

			Repeater {
				model: [ "Open session", "Empty session" ]
				delegate: Item {
					id: itm
					height: 30
					width: menu.width - 10
					x: (parent.width - width) / 2

					state: {
						if(menu.activeItem == modelData) {
							"active"
						} else if(mouseArea.containsMouse) {
							"hover"
						} else {
							""
						}
					}

					Rectangle {
						visible: itm.state == "hover"
						anchors.fill: parent
						radius: 5
						border { color: menu.secondaryColor; width: 1 }
						color: "#00000000"
					}

					Rectangle {
						visible: itm.state == "active"
						anchors.fill: parent
						radius: 5
						color: menu.secondaryColor
					}

					Text {
						anchors.leftMargin: 5
						anchors.rightMargin: 5
						anchors.fill: parent

						text: modelData
						elide: Text.ElideRight
						verticalAlignment: Text.AlignVCenter
						color: "white"
						font {
							family: "Monospace"
							pixelSize: 16
						}
					}

					MouseArea {
						id: mouseArea
						hoverEnabled: true
						anchors.fill: parent
						onClicked: { selected(modelData) }
					}
				}
			}
		}
	}
}
/*
	signal back

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
		primaryAction: "Back"

		Behavior on x {
			NumberAnimation { duration: 300 }
		}

		onPrimary: {
			back()
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
										mouseArea.sess = Panopticon.createSession(path.substring(7, path.length))
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
}*/

import QtQuick 2.0

Item {
	id: root

	property bool alignLeft: true
	property color primaryColor: "#34495e"
	property color secondaryColor: "#1bbc9b"
	property var model: []
	property var activeItem: null
	property var icon: null

	signal selected(var i)

	Rectangle {
		anchors.fill: parent
		color: root.primaryColor
	}

	Behavior on x {
			NumberAnimation { duration: 100; easing.type: Easing.OutCubic }
	}

	Item {
		id: head
		y: 5
		width: parent.width
		height: 80

		Image {
			id: iconImg
			anchors.centerIn: parent
			source: root.icon
		}
	}

	Column {
		anchors.top: head.bottom
		anchors.topMargin: 5
		width: parent.width
		spacing: 4

		Repeater {
			model: root.model
			delegate: Item {
				id: itm
				height: 30
				width: root.width - 10
				x: (parent.width - width) / 2

				state: {
					if(root.activeItem == modelData) {
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
					border { color: root.secondaryColor; width: 1 }
					color: "#00000000"
				}

				Rectangle {
					visible: itm.state == "active"
					anchors.fill: parent
					radius: 5
					color: root.secondaryColor
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
/*
	Item {
		id: mainData

		height: parent.height
		width: 200
		x: root.state == "data" ? parent.width - width : parent.width

		Rectangle {
			anchors.fill: parent
			color: "#1c1c1c"
		}

		Behavior on x {
        NumberAnimation { duration: 100; easing.type: Easing.InOutElastic }
    }

		Column {
			y: 5
			width: parent.width
			spacing: 4

			Repeater {
				model: root.procedures
				delegate: Item {
					height: 30
					width: 190
					x: (parent.width - width) / 2

					Rectangle {
						visible: modelData == root.select
						anchors.fill: parent
						radius: 5
						color: "#bed83f"
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
						anchors.fill: parent
						onClicked: { root.select = modelData }
					}
				}
			}
		}
	}
}*/

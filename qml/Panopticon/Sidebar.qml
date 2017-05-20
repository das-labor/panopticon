import QtQuick 2.4
import QtQuick.Controls 1.3 as Ctrl
import QtQuick.Layouts 1.1
import Panopticon 1.0

Rectangle {
	signal showControlFlowGraph(string uuid)

	id: root
	color: "white"

	Accessible.name: "Sidebar"
	Accessible.role: Accessible.Pane

	Rectangle {
		id: divider

		anchors.right: parent.right
		anchors.top: parent.top
		anchors.bottom: parent.bottom
		width: 1
		color: "#d8dae4"
		Accessible.role: Accessible.Separator
	}

	Ctrl.Label {
		anchors.centerIn: parent
		width: 140
		font {
			family: "Source Sans Pro"; pointSize: 20;
		}
		visible: listView.count == 0
		text: "Please open a program"
		color: "#a2a2a2"
		horizontalAlignment: Text.AlignHCenter
		wrapMode: Text.WordWrap
	}

	Item {
		id: functionList

		anchors.left: parent.left
		anchors.top: parent.top
		anchors.bottom: parent.bottom
		anchors.right: divider.left

		Accessible.role: Accessible.List
		Accessible.name: "Function List"
		visible: listView.count > 0

		ListView {
			id: listView
			anchors.left: parent.left
			anchors.right: parent.right
			anchors.top: parent.top
			anchors.bottom: parent.bottom
			anchors.topMargin: 10
			anchors.leftMargin: 5
			anchors.rightMargin: 5

			model: Panopticon.sidebar
			delegate: Item {
				width: listView.width
				height: childrenRect.height

				Accessible.name: title
				Accessible.role: Accessible.ListItem

				Rectangle {
					id: rowMarker
					anchors.fill: parent
					anchors.rightMargin: 16+5+5
					color: "#eee"
					radius: 3
					visible: rowMouseArea.containsMouse && !labelMouseArea.containsMouse
				}

				MouseArea {
					id: rowMouseArea
					anchors.fill: parent
					anchors.rightMargin: 16+5
					hoverEnabled: true

					Accessible.name: "Open " + title
					Accessible.role: Accessible.Button

					function activate() {
						console.log("display cfg for " + uuid);
						root.showControlFlowGraph(uuid);
					}

					Accessible.onPressAction: activate()
					onClicked: activate()
				}

				RowLayout {
					id: row
					width: parent.width

					Accessible.ignored: true

					Ctrl.Label {
						Layout.leftMargin: 10
						Layout.fillWidth: true
        		Accessible.ignored: true

						id: titleLabel
						text: title
						font { pointSize: 11; family: "Source Sans Pro" }
					}

					Ctrl.Label {
						Layout.leftMargin: 5
						Layout.rightMargin: 10

						Accessible.description: "Function entry point"

						id: subtitleLabel
						text: subtitle
						color: "#316460"
						font { pointSize: 11; family: "Source Code Pro" }
						horizontalAlignment: Text.AlignLeft
					}

					Image {
						Layout.preferredWidth: 16
						Layout.preferredHeight: 16
						Layout.rightMargin: 5

						id: editLabel
						source: "../icons/pencil.svg"
						fillMode: Image.PreserveAspectFit
						mipmap: true
						opacity: labelMouseArea.containsMouse ? 1 : (rowMouseArea.containsMouse ? .6 : 0)

						Accessible.name: "Rename function " + title
						Accessible.role: Accessible.Button
						Accessible.onPressAction: labelMouseArea.activate()

						MouseArea {
							id: labelMouseArea
							width: parent.width
							height: row.height
							hoverEnabled: true
							onClicked: activate()
        			Accessible.ignored: true

							function activate() {
								console.log("rename " + uuid);
								var mapped = row.mapToItem(root,row.x,row.y,row.width,row.height);
								renamePopover.open(mapped,title,uuid);
							}
						}
					}
				}
			}
		}
	}

	RenamePopover {
		id: renamePopover
	}
}

import QtQuick 2.4
import QtQuick.Controls 1.3 as Ctrl
import QtQuick.Layouts 1.1

Rectangle {
	signal showCallGraph(string uuid)
	signal showControlFlowGraph(string uuid)

	id: root
	color: "white"

	Rectangle {
		id: divider

		anchors.right: parent.right
		anchors.top: parent.top
		anchors.bottom: parent.bottom
		width: 1
		color: "#d8dae4"
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

		visible: listView.count > 0

		Rectangle {
			id: searchBar

			anchors.left: parent.left
			anchors.right: parent.right
			anchors.top: parent.top

			height: 50

			color: "white"


			Row {
				anchors.centerIn: parent
				spacing: 10
			Image {
				source: "../icons/search.svg"
				fillMode: Image.PreserveAspectFit
				width: 18
				mipmap: true
			}

			Label {
				height: 18
				width: 140
				text: "Search"
				font {
					pointSize: 11
				}
					color: "#a2a2a2"
				verticalAlignment: Text.AlignVCenter
			}
		}
		}

		ListView {
			id: listView
			anchors.left: parent.left
			anchors.right: parent.right
			anchors.top: searchBar.bottom
			anchors.bottom: parent.bottom
			anchors.topMargin: 10
			anchors.leftMargin: 5
			anchors.rightMargin: 5

			model: Panopticon.functions
			delegate: Item {
				height: 18
				width: listView.width

				RowLayout {
					id: row
					anchors.fill: parent

					Ctrl.Label {
						Layout.leftMargin: 5 + hierarchy * 15

						id: titleLabel
						text: title
						font { pointSize: 11; family: "Source Sans Pro" }

						MouseArea {
							id: mouseArea
							anchors.top: parent.top
							anchors.bottom: parent.bottom
							x: 0; width: titleLabel.width + 5 + subtitleLabel.width
							hoverEnabled: true
							onClicked: {
																console.log("display cfg for " + uuid);
									root.showControlFlowGraph(uuid);
														}
						}

						Rectangle {
							anchors.top: parent.top
							anchors.bottom: parent.bottom
							x: -3; width: titleLabel.width + 5 + subtitleLabel.width + 6
							z: -1
							color: "#eee"
							radius: 3
							visible: mouseArea.containsMouse
						}
					}

					Ctrl.Label {
						Layout.leftMargin: 5
						Layout.fillWidth: hierarchy > 0

						id: subtitleLabel
						text: subtitle
						color: "#316460"
						font { pointSize: 11; family: "Source Code Pro" }
						horizontalAlignment: Text.AlignLeft
					}

					Item {
						Layout.leftMargin: 10
						Layout.rightMargin: 10
						Layout.preferredHeight: 15
						Layout.preferredWidth: 15
						Layout.fillWidth: true

						visible: hierarchy == 0

						Rectangle {
							anchors.margins: -5
							anchors.fill: icon
							color: "#eee"
							radius: width / 2
							visible: collapseMouseArea.containsMouse
						}

						MouseArea {
							id: collapseMouseArea
							anchors.fill: icon
							hoverEnabled: true
							onClicked: {
								Panopticon.toggle_sidebar_item(index)
							}
						}

						Image {
							id: icon
							anchors.top: parent.top
							anchors.bottom: parent.bottom
							anchors.right: parent.right
							source: (childrenCollapsed ? "../icons/chevron-up.svg" : "../icons/chevron-down.svg")
							fillMode: Image.PreserveAspectFit
						}
					}
				}
			}
		}
	}
}

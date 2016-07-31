/*
 * Panopticon - A libre disassembler (https://panopticon.re/)
 * Copyright (C) 2014,2015,2016 Kai Michaelis
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

import QtQuick 2.4
import QtQuick.Controls 1.3
import QtQuick.Layouts 1.1
import QtQuick.Window 2.1

Window {
	flags: Qt.Dialog
	visible: true
	y: (Screen.desktopAvailableHeight - height) / 2
	x: (Screen.desktopAvailableWidth - width) / 2
	width: 790
	height: 400

	onClosing: {
		sessionView.deleteSessions();
	}

	Rectangle {
		anchors.fill: parent
		color: "white"
	}

	// Version
	Label {
		y: 6
		anchors.right: parent.right
		anchors.rightMargin: 6

		text: "v0.15.0"
		font {
			pointSize: 11
			family: "Source Sans Pro"
		}
	}

	// Panopticon logo font
	Image {
		y: 51
		anchors.horizontalCenter: parent.horizontalCenter
		source: "icons/logo.svg"
	}

	Row {
		y: 154
		spacing: 74
		anchors.horizontalCenter: parent.horizontalCenter
		height: parent.height - 154

		// Menu
		Column {
			id: menu
			spacing: 27

			Repeater {
				model: ListModel {
					ListElement { title: "Open"; description: "Start disassembly of a new file"; icon: "icons/open-icon.svg" }
					ListElement { title: "Sandbox"; description: "Open an empty workspace"; icon: "icons/sandbox-icon.svg" }
					ListElement { title: "Example"; description: "Try out Panopticon"; icon: "icons/example-icon.svg" }
				}
				delegate: MouseArea {
					id: root
					width: childrenRect.width
					height: childrenRect.height
					hoverEnabled: true

					GridLayout {
						id: grid
						columnSpacing: 14
						rowSpacing: 0

						Item {
							height: img.height
							width: img.width
							Layout.rowSpan: 2

							Image {
								id: img
								source: icon
							}
						}
						Label {
							Layout.column: 1
							Layout.row: 0
							Layout.alignment: Qt.AlignBottom | Qt.AlignLeft
							Layout.fillWidth: true
							text: title
							font {
								pointSize: 13
								weight: Font.DemiBold
								family: "Source Sans Pro"
								underline: root.containsMouse
							}
						}

						Label {
							Layout.column: 1
							Layout.row: 1
							Layout.alignment: Qt.AlignTop | Qt.AlignLeft
							Layout.fillWidth: true
							text: description
							font {
								pointSize: 13
								family: "Source Sans Pro"
								underline: root.containsMouse
							}
						}
					}
				}
			}
		}

		// Recent session list
		Column {
			spacing: 27
			height: parent.height
			width: 308

			ListModel {
				id: sessionModel
			}

			ListView {
				property var toDelete: []

				function deleteSessions() {
					for(var i = 0; i < toDelete.length; i++) {
						var res = JSON.parse(Panopticon.deleteSession(toDelete[i]));

						if(res.status == "err") {
							console.error(res.error);
						}
					}
				}

				id: sessionView
				width: 330
				height: menu.height
				clip: true
				maximumFlickVelocity: 500
				boundsBehavior: Flickable.StopAtBounds
				model: sessionModel
				delegate: MouseArea {
					id: elem
					width: childrenRect.width
					height: childrenRect.height + 23
					hoverEnabled: true

					GridLayout {
						columnSpacing: 10
						rowSpacing: 0

						Label {
							text: title
							elide: Text.ElideRight
							Layout.minimumWidth: 190
							Layout.maximumWidth: 190
							font {
								strikeout: elem.state == "DELETE"
								pointSize: 11
								weight: Font.DemiBold
								family: "Source Sans Pro"
								underline: openArea.containsMouse
							}

							MouseArea {
								id: openArea
								anchors.fill: parent
								hoverEnabled: true

								onClicked: {
									Panopticon.path = path
									Panopticon.type = "project"
									Qt.quit()
								}
							}
						}

						Label {
							text: age
							color: "#b3b3b3"
							font {
								strikeout: elem.state == "DELETE"
								pointSize: 11
								family: "Source Sans Pro"
							}
						}

						Image {
							opacity: elem.containsMouse ? 1.0 : 0.0
							source: "icons/delete-icon.svg"

							MouseArea {
								id: deleteArea
								anchors.fill: parent
								hoverEnabled: true
								onClicked: {
									if(elem.state == "") {
										elem.state = "DELETE"
										sessionView.toDelete.push(file)
									} else {
										var idx = sessionView.toDelete.indexOf(file);

										if(idx >= 0) {
											sessionView.toDelete.splice(idx,1)
										}
										elem.state = ""
									}
								}
							}
						}
					}
				}

				Component.onCompleted: {
					var res = JSON.parse(Panopticon.sessions())

					if(res.status === "ok") {
						for(var i = 0; i < res.payload.length; i++) {
							var t = res.payload[i];
							sessionModel.append({
								"title": t.title,
								"age": t.age,
								"file": t.file,
								"path": t.path
							});
						}
					} else {
						console.error(res.error)
					}
				}
			}

			Rectangle {
				anchors.left: sessionView.right
				anchors.leftMargin: 10
				opacity: sessionView.moving ? 1.0 : 0.2
				y: sessionView.visibleArea.yPosition * sessionView.height
				width: 4
				height: sessionView.visibleArea.heightRatio * sessionView.height
				color: "#aaaaaa"
				radius: 3

				Behavior on opacity {
					NumberAnimation { duration: 1000 }
				}
			}
		}
	}
}

/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Kai Michaelis
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

import QtQuick 2.0
import Panopticon 1.0
import Qt.labs.settings 1.0
import Qt.labs.folderlistmodel 1.0
import "../workspace"

Loader {
	Component {
		id: rootComponent

		Rectangle {
			Component {
				id: menuItemDelegate

				Item {
					id: itm
					height: 45
					width: menu.width - 10
					x: (parent.width - width) / 2

					state: {
						if(root.state == modelData.state) {
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
						anchors.leftMargin: 10
						anchors.rightMargin: 10
						anchors.fill: parent

						text: modelData.title
						elide: Text.ElideRight
						verticalAlignment: Text.AlignVCenter
						color: "white"
						font {
							family: "Sans"
							pixelSize: 20
						}
					}

					MouseArea {
						id: mouseArea
						hoverEnabled: true
						anchors.fill: parent
						onClicked: { root.state = modelData.state }
					}
				}
			}

			Component {
				id: workspace
				Workspace {
					session: root.session
					clip: true
					focus: true
				}
			}

			id: root

			property variant session: null
			property color primaryColor: "#34495e"
			property color secondaryColor: "#1bbc9b"

			anchors.fill: parent
			color: "#eeeeee"
			state: "recent"
			focus: true

			Settings {
				id: settings
				property string recent: ""
			}

			Item {
				id: menu

				property bool alignLeft: true
						property var model: [[
					{"title":"Open session", "state":"open"}
				],[
					{"title":"AVR image", "state":"avr"},
					{"title":"Raw data", "state":"raw"}
				]]
				property var activeItem: null
				property var icon: null

				height: parent.height
				width: 200

				Rectangle {
					anchors.fill: parent
					color: root.primaryColor
				}

				Column {
					y: 5
					width: parent.width
					spacing: 4

					Repeater {
						model: menu.model[0]
						delegate: menuItemDelegate
					}

					Item {
						height: 40
						width: parent.width

						Rectangle {
							color: "#eeeeee"
							x: 25
							y: 19
							width: parent.width - 50
							height: 1
							radius: 2
						}
					}

					Repeater {
						model: menu.model[1]
						delegate: menuItemDelegate
					}
				}
			}

			Item {
				id: recent

				anchors.left: menu.right
				anchors.right: parent.right
				height: parent.height

				visible: root.state == "recent"

				Grid {
					anchors.fill: parent
					anchors.margins: 25
					spacing: 25

					Repeater {
						model: settings.recent.split(",").filter(function(a) { return a.length > 0 })
						delegate: Item {
							height: 300
							width: 250
							clip: true

							Rectangle {
								radius: 3
								border { width: 2; color: root.secondaryColor }
								color: root.primaryColor
								anchors.fill: parent

								Text {
									y: parent.height - 40 - contentHeight
									width: parent.width - 20
									x: 10
									elide: Text.ElideLeft
									verticalAlignment: Text.AlignVCenter
									color: "white"
									font {
										family: "Sans"
										pixelSize: 20
									}
									text: modelData
								}

								Image {
									source: "../panop.png"
									x: (parent.width - sourceSize.width) / 2
									y: (parent.width * parent.height) / (parent.height + parent.width) - sourceSize.height / 2
								}
							}

							MouseArea {
								id: mouseArea
								anchors.fill: parent

								onPressed: {
									root.session = Panopticon.openSession(modelData.substring(7))

									if(root.session == null) {
										messageDialog.state = "visible"
										messageDialog.title = "Error: Can't load session"
										messageDialog.message = "The file '" + modelData.substring(modelData.lastIndexOf("/") + 1) + "' is not a valid Panopticon session."
										messageDialog.callback = function() { root.enabled = true }
									} else {
										loader.setSource("../workspace/Workspace.qml",{ "session": Panopticon.session })
									}
								}
							}
						}
					}
				}
			}

			Item {
				id: open

				anchors.left: menu.right
				anchors.right: parent.right
				height: parent.height

				visible: { root.state == "open" || root.state == "avr" || root.state == "raw" }

				Text {
					x: 50
					y: 50
					text: {
						if(root.state == "open") {
							"Open session"
						} else if(root.state == "avr") {
							"Disassemble AVR image"
						} else if(root.state == "raw") {
							"Open plain file"
						} else {
							""
						}
					}
					font {
						family: "Sans"
						pixelSize: 36
					}
					color: root.primaryColor
				}

				Text {
					x: 50
					y: 100
					text: { filepicker_model.folder.toString().substring(7) }
				}

				ListView {
					id: view
					focus: true

					x: 50
					y: 140
					height: parent.height - y
					width: parent.width - x

					model: FolderListModel {
						id: filepicker_model
						showDirsFirst: true
						showDotAndDotDot: true
						nameFilters: {
							if(root.state == "open") {
								["*.panop"]
							} else {
								[]
							}
						}
					}

					delegate: Component {
						Item {
							visible: fileName != "."
							width: view.width
							height: label.height + 8

							Text {
								id: label
								text: fileName + (filepicker_model.isFolder(index) ? "/" : "")
								font {
									pixelSize: 18
									family: "Sans"
								}
								color: (mouseArea.containsMouse ? root.secondaryColor : "#1e1e1e")

								anchors.verticalCenter: parent.verticalCenter
								x: 0
							}

							MouseArea {
								property variant sess: null

								id: mouseArea
								anchors.fill: parent
								hoverEnabled: true

								onClicked: {
									var b = filepicker_model.folder.toString().substring(7).split("")
									var e = b.reduce(function (acc,x) {
										if(x == "/" && !acc.escape) {
											acc.ret.push("")

											return {
												"escape": false,
												"ret": acc.ret
											}
										} else {
											acc.ret[acc.ret.length - 1] += x

											if(x == "\\") {
												if(acc.escape) {
													return {
														"escape": false,
														"ret": acc.ret
													}
												} else {
													return {
														"escape": true,
														"ret": acc.ret
													}
												}
											} else {
												return {
													"escape": false,
													"ret": acc.ret
												}
											}
										}
									},{"escape": false, "ret": [""]}).ret

									if(filepicker_model.isFolder(index)) {
										if(fileName == "..") {
											if(e.length > 0) {
												e = e.slice(0,e.length - 1)
											}
										} else if(fileName == ".") {
											;
										} else {
											e.push(fileName)
										}

										filepicker_model.folder = Qt.resolvedUrl("file://" + e.join("/"))
									} else {
										var path = Qt.resolvedUrl(filepicker_model.folder + "/" + fileName).toString()
										var mru = settings.recent.split(",").filter(function(a) { return a.length > 0 }).filter(function(a) { return a !== path })

										if(root.state == "open") {
											mru.unshift(path)
											settings.recent = mru.slice(0,6).join(",")
										}

										if(root.state == "open") {
											root.session = Panopticon.openSession(path.substring(7, path.length))
										} else if(root.state == "avr") {
											root.session = Panopticon.createAvrSession(path.substring(7, path.length))
										} else if(root.state == "raw") {
											root.session = Panopticon.createRawSession(path.substring(7, path.length))
										} else {
											console.error("BUG: invalid menu state")
										}

										if(root.session == null) {
											messageDialog.state = "visible"
											messageDialog.title = "Error: Can't create session"
											messageDialog.message = "The file '" + path.substring(path.lastIndexOf("/") + 1) + "' can't be opened into a new Panopticon session."
											messageDialog.callback = function() { root.enabled = true }
										} else {
											loader.setSource("../workspace/Workspace.qml",{ "session": Panopticon.session })
										}
									}
								}

							}
						}
					}
				}
			}
		}
	}

	Rectangle {
		id: messageDialog

		property string title: "Error"
		property string message: "Unknown error."
		property var callback: function() {}

		Keys.onPressed: {
			if(event.key == Qt.Key_Return && messageDialog.visible) {
				event.accepted = true
				messageDialog.state = "hidden"
				messageDialog.callback()
			}
		}

		anchors.centerIn: parent
		color: "#ee1e1e"
		height: titleItem.font.pixelSize + 10 + 3 + 40 + (messageItem.contentHeight) + 20 + button.height + 10
		width: 600
		z: 1
		radius: 15
		border { width: 3; color: "#aa1e1e" }
		opacity: 0
		scale: .666

		states: [ State {
			name: "hidden"
			PropertyChanges {
				target: messageDialog
				opacity: 0
				scale: .666
				focus: false
			}
		}, State {
			name: "visible"
			PropertyChanges {
				target: messageDialog
				opacity: .95
				scale: 1
				focus: true
			}
		} ]

		state: "hidden"

		Behavior on opacity { NumberAnimation { duration: 100 } }
		Behavior on scale { NumberAnimation { duration: 100 } }

		Text {
			id: titleItem
			anchors.top: parent.top
			anchors.topMargin: 5
			text: messageDialog.title
			font {
				family: "Sans"
				pixelSize: 36
			}
			x: (parent.width - contentWidth) / 2
			color: "white"
		}

		Rectangle {
			anchors.topMargin: titleItem.anchors.topMargin
			anchors.top: titleItem.bottom
			height: 3
			width: parent.width
			color: "#aa1e1e"
		}

		Text {
			id: messageItem
			text: messageDialog.message
			color: "white"
			anchors.left:  parent.left
			anchors.right:  parent.right
			anchors.top:  titleItem.bottom
			anchors.bottom:  parent.bottom
			anchors.margins: 40

			font {
				family: "Sans"
				pixelSize: 28
			}
			horizontalAlignment: Text.AlignTop
			wrapMode: Text.WordWrap
		}

		Rectangle {
			id: button
			color: "#aa1e1e"
			opacity: 1
			radius: 15
			anchors.bottom: parent.bottom
			anchors.bottomMargin: 5
			height: 45
			width: 100
			x: (parent.width - width) / 2

			Text {
				text: "Okay"
				color: "white"
				font {
					family: "Sans"
					pixelSize: 28
				}
				anchors.fill: parent
				verticalAlignment: Text.AlignVCenter
				horizontalAlignment: Text.AlignHCenter
			}

			MouseArea {
				anchors.fill: parent

				onClicked: {
					messageDialog.state = "hidden"
					messageDialog.callback()
				}
			}
		}
	}

	id: loader
	width: 1000
	height: 1000
	sourceComponent: rootComponent
	focus: true
}

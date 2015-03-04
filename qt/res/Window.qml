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

import QtQuick.Controls 1.1
import QtQuick.Dialogs 1.1
import QtQuick 2.1
import Panopticon 1.0

ApplicationWindow {
	id: mainWindow

	MessageDialog {
		id: saveStaleDialog
		title: "Unsaved changes"
		text: "Do you want to save the changes made to the current session?"
		icon: StandardIcon.Question
		standardButtons: StandardButton.Yes | StandardButton.No | StandardButton.Abort

		property var next: function() {}

		onYes: {
			if(Panopticon.session.savePath != "") {
				Panopticon.session.save(Panopticon.session.savePath)
				next()
			} else {
				fileSaveDialog.next = saveStaleDialog.next
				fileSaveDialog.open()
			}
		}

		onNo: {
			next()
		}

		onRejected: {}
	}

	function saveStaleSession(next) {
		if(Panopticon.session) {
			saveStaleDialog.next = next
			saveStaleDialog.open()
		} else {
			next()
		}
	}

	title: "Panopticon"
	height: 1000
	width: 1000
	menuBar: MenuBar {
		Menu {
			title: "File"
			Menu {
				title: "New..."

				MenuItem {
					text: "Relocated AVR image"
					shortcut: "Ctrl+A"
					onTriggered: {
						saveStaleSession(function() {
							fileNewDialog.openFunction = Panopticon.createAvrSession
							fileNewDialog.open()
						})
					}
				}

				MenuItem {
					text: "Uninterpreted data"
					shortcut: "Ctrl+R"
					onTriggered: {
						saveStaleSession(function() {
							fileNewDialog.openFunction = Panopticon.createRawSession
							fileNewDialog.open()
						});
					}
				}
			}

			MenuItem {
				text: "Open"
				shortcut: "Ctrl+O"
				onTriggered: {
					saveStaleSession(fileOpenDialog.open);
				}
			}
			MenuItem {
				text: "Save"
				shortcut: "Ctrl+S"
				enabled: Panopticon.session && Panopticon.session.dirty
				onTriggered: {
					if(Panopticon.session.savePath != "") {
						Panopticon.session.save(Panopticon.session.savePath)
					} else {
						fileSaveDialog.open()
					}
				}
			}
			MenuItem {
				text: "Save As"
				shortcut: "Ctrl+Shift+S"
				enabled: Panopticon.session
				onTriggered: { fileSaveDialog.open() }
			}

			MenuSeparator {}

			MenuItem {
				text: "Quit"
				shortcut: "Ctrl+Q"
			}
		}
	}

	FileDialog {
		id: fileSaveDialog
		title: "Save current session to..."
		selectExisting: false
		selectFolder: false

		property var next: function() {}

		onAccepted: {
			console.log("You saved to: " + fileSaveDialog.fileUrls)
			Panopticon.session.save(fileSaveDialog.fileUrls)
			next()
		}
	}

	FileDialog {
		id: fileOpenDialog
		title: "Open new session..."
		selectExisting: true
		selectFolder: false

		property var next: function() {}

		onAccepted: {
			// cut off the "file://" part
			var path = fileOpenDialog.fileUrls.toString().substring(7)
			var sess = Panopticon.openSession(path)

			if(sess == null) {
				console.log("The file '" + path + "' is not a valid Panopticon session.")
			} else {
				loader.setSource("workspace/Workspace.qml",{ "session": Panopticon.session })
			}

			next()
		}
	}

	FileDialog {
		id: fileNewDialog
		title: "Start new session..."
		selectExisting: true
		selectFolder: false

		property var openFunction: null
		property var next: function() {}

		onAccepted: {
			console.log("You opened: " + fileNewDialog.fileUrls)

			// cut off the "file://" part
			var path = fileNewDialog.fileUrls.toString().substring(7)
			var sess = fileNewDialog.openFunction(path)

			if(sess == null) {
				console.log("The file '" + path + "' is not a valid Panopticon session.")
			} else {
				loader.setSource("workspace/Workspace.qml",{ "session": Panopticon.session })
			}

			next()
		}
	}

	Loader {
		focus: true
		id: loader
		anchors.fill: parent
		sourceComponent: Item {
			anchors.fill: parent

			Item {
				anchors.centerIn: parent
				height: childrenRect.height
				width: childrenRect.width

				Image {
					id: panopLogo
					source: "qrc:///panop.png"
				}
				Text {
					anchors.verticalCenter: panopLogo.verticalCenter
					anchors.left: panopLogo.right
					anchors.leftMargin: 10
					text: "PANOPTICON"
					font {
						pixelSize: panopLogo.height
					}
				}
			}
		}
	}

	Component.onCompleted: {
		if(Panopticon.session) {
			loader.setSource("workspace/Workspace.qml",{ "session": Panopticon.session })
		}
	}
}

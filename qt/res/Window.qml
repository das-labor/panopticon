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
import QtQuick.Dialogs 1.0
import QtQuick 2.1
import Panopticon 1.0

ApplicationWindow {
	id: mainWindow
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
						fileNewDialog.openFunction = Panopticon.newAvrSession
						fileNewDialog.open()
					}
				}

				MenuItem {
					text: "Uninterpreted data"
					shortcut: "Ctrl+R"
					onTriggered: {
						fileNewDialog.openFunction = Panopticon.newRawSession
						fileNewDialog.open()
					}
				}
			}

			MenuItem {
				text: "Open"
				shortcut: "Ctrl+O"
				onTriggered: { fileOpenDialog.open() }
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

		onAccepted: {
			console.log("You saved to: " + fileSaveDialog.fileUrls)
			Panopticon.session.savePath(fileSaveDialog.fileUrls)
		}
	}

	FileDialog {
		id: fileOpenDialog
		title: "Open new session..."
		selectExisting: true
		selectFolder: false

		onAccepted: {
			console.log("You opened: " + fileOpenDialog.fileUrls)

			// cut off the "file://" part
			var path = fileOpenDialog.fileUrls.toString().substring(7)
			var sess = Panopticon.openSession(path)

			if(sess == null) {
				console.log("The file '" + path + "' is not a valid Panopticon session.")
			} else {
				loader.setSource("workspace/Workspace.qml",{ "session": Panopticon.session })
			}
		}
	}

	FileDialog {
		id: fileNewDialog
		title: "Start new session..."
		selectExisting: true
		selectFolder: false

		property var openFunction: null

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
		}
	}

	statusBar: Rectangle {
		height: 40
		color: '#ff11ff'
	}

	Loader {
		focus: true
		id: loader
		anchors.fill: parent
	}

	Component.onCompleted: {
		if(Panopticon.session) {
			loader.setSource("workspace/Workspace.qml",{ "session": Panopticon.session })
		} else {
			loader.setSource("wizard/Main.qml")
		}
	}
}

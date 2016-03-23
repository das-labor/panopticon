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

import QtQuick.Controls 1.1
import QtQuick.Dialogs 1.2
import QtQuick.Layouts 1.1
import QtQuick 2.1
import Panopticon 1.0
import "workspace";
import "popup";
import "action" as Act;

ApplicationWindow {
	id: mainWindow

	property bool enabled: true
	property bool workspaceLoaded: false

	title: "Panopticon"
	height: 1000
	width: 1000
	visible: true

	menuBar: MenuBar {
		Menu {
			title: "Project"
			id: projectMenu

			MenuItem {
				action: Act.Open {
					window: mainWindow
				}
			}

			MenuItem {
				action: Act.SaveAs {
					window: mainWindow
				}
			}

			MenuSeparator {}

			MenuItem {
				action: Act.Quit {
					window: mainWindow
				}
			}
		}
	}

	/*FileDialog {
		id: fileSaveDialog
		title: "Save current project to..."
		selectExisting: false
		selectFolder: false
		nameFilters: [ "Panopticon projects (*.panop)", "All files (*)" ]

		property var next: function() {}

		onAccepted: {
			var path = fileSaveDialog.fileUrls.toString().substring(7)

			if (path.substring(path.length - 6) != ".panop") {
				path += ".panop"
			}

			if (mainWindow.savePath == "") {
				mainWindow.savePath = path;
			}

			var res = JSON.parse(Panopticon.snapshotProject(path))

			if(res.state == "ok") {
				next()
			} else {
				errorDialog.text = res.error;
				errorDialog.open();
			}
		}
	}

	FileDialog {
		id: fileOpenDialog
		title: "Open new project..."
		selectExisting: true
		selectFolder: false
		nameFilters: [ "Panopticon projects (*.panop)", "All files (*)" ]

		property var next: function() {}

		onAccepted: {
			// cut off the "file://" part
			var path = fileOpenDialog.fileUrls.toString().substring(7)
			var res = JSON.parse(Panopticon.openProject(path));

			if(res.status == "ok") {
				loader.setSource("workspace/Workspace.qml")
				next()
			} else {
				errorDialog.text = res.error;
				errorDialog.open();
			}
		}
	}

	FileDialog {
		id: fileNewDialog
		title: "Start new project..."
		selectExisting: true
		selectFolder: false

		property var next: null

		onAccepted: {
			// cut off the "file://" part
			var path = fileNewDialog.fileUrls.toString().substring(7)
			next(path)
		}
	}*/

	Component {
		id: welcomeScreen

		Item {
			anchors.fill: parent

			Item {
				anchors.centerIn: parent
				height: childrenRect.height
				width: childrenRect.width

				Image {
					id: panopLogo
					source: "panop.png"
				}

				Text {
					anchors.verticalCenter: panopLogo.verticalCenter
					anchors.left: panopLogo.right
					anchors.leftMargin: 10
					text: "PANOPTICON"
					color: "#1e1e1e";
					font {
						pixelSize: panopLogo.height
					}
				}
			}
		}
	}

	Loader {
		focus: true
		id: loader
		anchors.fill: parent
		sourceComponent: welcomeScreen
	}

	Component.onCompleted: {
		Panopticon.onStateChanged.connect(function() {
			switch(Panopticon.state) {
				case "":
				case "NEW": {
					workspaceLoaded = false;
					loader.sourceComponent = welcomeScreen;
					break;
				}

				case "SYNC":
				case "DIRTY": {
					if(!mainWindow.workspaceLoaded) {
						workspaceLoaded = true;
						loader.setSource("workspace/Workspace.qml")
					}
					break;
				}

				default: {
					console.error("Unknown state: " + Panopticon.state);
				}
			}
		})
	}
}

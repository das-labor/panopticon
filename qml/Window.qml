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
import QtQuick.Controls 1.3 as Ctrl
import QtQuick.Layouts 1.1

import Panopticon 1.0

import "."

Ctrl.ApplicationWindow {
	id: mainWindow

	function serveRequest(req) {
		targetSelect.request = req

		switch(req.kind) {
			case "panop": {
				var res = JSON.parse(Panopticon.openProject(req.path))
				if(res.status == "err") {
					console.exception(res.error);
				}
				break;
			}
			case "mach-o": {
				var res = JSON.parse(Panopticon.createProject(req.path))
				if(res.status == "err") {
					console.exception(res.error);
				}
				break;
			}
			case "elf": {
				var res = JSON.parse(Panopticon.createProject(req.path))
				if(res.status == "err") {
					console.exception(res.error);
				}
				break;
			}
			case "pe": {
				var res = JSON.parse(Panopticon.createProject(req.path))
				if(res.status == "err") {
					console.exception(res.error);
				}
				break;
			}
			case "raw": {
				targetSelect.visible = true;
				break;
			}
			case "sandbox":{
				// do nothing
				break;
			}
			case "avr": {
				var res = JSON.parse(Panopticon.createRawProject(req.path,"atmega88",0,-1))
				if(res.status == "err") {
					console.exception(res.error);
				}
				break;
			}
			default:
				console.exception("Unknown request kind " + req.kind);
		}
	}

	Component {
		id: fileBrowser
		FileBrowser {}
	}

	Component {
		id: errorPopup
		ErrorPopup {}
	}

	property bool enabled: true
	property bool workspaceLoaded: false

	title: "Panopticon"
	height: 1000
	width: 1000
	visible: true

	menuBar: Ctrl.MenuBar {
		Ctrl.Menu {
			title: "Project"
			id: projectMenu

			Ctrl.MenuItem {
				text: action.text
				action: Open {
					window: mainWindow
					fileBrowser: fileBrowser;
					errorPopup: errorPopup;
				}
			}

			Ctrl.MenuItem {
				text: action.text
				action: SaveAs {
					window: mainWindow
					fileBrowser: fileBrowser;
					errorPopup: errorPopup;
				}
			}

			Ctrl.MenuSeparator {}

			Ctrl.MenuItem {
				text: action.text
				action: Quit {
					window: mainWindow
					errorPopup: errorPopup;
				}
			}
		}
	}

	Workspace {
		id: workspace
		visible: !targetSelect.visible
		anchors.fill: parent
	}

	Rectangle {
		id: targetSelect

		property var request: null

		anchors.fill: parent
		color: "#eeeeee"
		visible: false

		Item {
			anchors.horizontalCenter: parent.horizontalCenter
			y: 0.25 * parent.height
			width: childrenRect.width
			height: childrenRect.height

			Column {
				spacing: 30

				Row {
					anchors.horizontalCenter: parent.horizontalCenter
					spacing: 27
					Image {
						width: sourceSize.width
						height: sourceSize.height
						source: "icons/warning-icon.svg"
						fillMode: Image.Pad
					}

					Label {
						text: "Cannot recognize file type"
						font {
							pointSize: 28
						}
						color: "#555555"
					}
				}

				Rectangle {
					color: "#888888"
					width: 560
					height: 1
				}

				Column {
					anchors.horizontalCenter: parent.horizontalCenter
					spacing: 18

					Label {
						width: 500
						text: "<strong>Microcontroller to assume for analysis</strong>. This option defines what instructions are supported and the size of the Program Counter register."
						wrapMode: Text.WordWrap
						font {
							pointSize: 12
						}
					}

					Ctrl.ComboBox {
						id: targetCombobox
						model: targetModel
						width: 140

						ListModel {
							id: targetModel
							ListElement {
								text: "MOS 6502"
								ident: "mos6502"
							}
							ListElement {
								text: "ATmega103"
								ident: "atmega103"
							}
							ListElement {
								text: "ATmega16"
								ident: "atmega16"
							}
							ListElement {
								text: "ATmega8"
								ident: "atmega8"
							}
							ListElement {
								text: "ATmega88"
								ident: "atmega88"
							}
						}
					}
				}

				Ctrl.Button {
					anchors.right: parent.right
					text: "Apply"

					onClicked: {
						var tgt = targetModel.get(targetCombobox.currentIndex).ident;
						var res = JSON.parse(Panopticon.createRawProject(targetSelect.request.path,tgt,0,-1))
						if(res.status == "ok") {
							targetSelect.visible = false;
						} else {
							console.exception(res.error);
						}
					}
				}
			}
		}
	}

	Component.onCompleted: {
		console.log(Panopticon.state);
		if(Panopticon.state == "NEW") {
			var res = JSON.parse(Panopticon.request());

			if(res.status == "ok" && res.payload != null) {
				serveRequest(res.payload)
			}
		}
	}
}

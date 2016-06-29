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

import Panopticon 1.0

import "."

ApplicationWindow {
	id: mainWindow

	Component {
		id: fileBrowser
		FileBrowser {}
	}

	Component {
		id: targetPopup
		TargetPopup {}
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

	menuBar: MenuBar {
		Menu {
			title: "Project"
			id: projectMenu

			MenuItem {
				text: action.text
				action: Open {
					window: mainWindow
					fileBrowser: fileBrowser;
					errorPopup: errorPopup;
					targetPopup: targetPopup;
				}
			}

			MenuItem {
				text: action.text
				action: SaveAs {
					window: mainWindow
					fileBrowser: fileBrowser;
					errorPopup: errorPopup;
				}
			}

			MenuSeparator {}

			MenuItem {
				text: action.text
				action: Quit {
					window: mainWindow
					errorPopup: errorPopup;
				}
			}
		}
	}

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

	Component {
		id: workspace
		Workspace {}
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
						loader.sourceComponent = workspace;
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

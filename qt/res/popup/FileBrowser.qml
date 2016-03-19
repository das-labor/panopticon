/*
 * Panopticon - A libre disassembler (https://panopticon.re/)
 * Copyright (C) 2016 Kai Michaelis
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

import Panopticon 1.0
import QtQuick 2.3
import QtQuick.Dialogs 1.2
import QtQuick.Controls 1.2
import QtQuick.Window 2.2
import Qt.labs.folderlistmodel 2.1
import QtQuick.Layouts 1.2

Popup {
	id: browser

	// READ or WRITE
	property string mode: "READ"
	property bool valid: false
	property string willOverwrite: ""
	property string selectedFile: currentPath + "/" + currentFile
	property string currentPath: ""
	property string currentFile: ""

	function readFile() {
		browser.mode = "READ";
		return show();
	}

	function writeFile() {
		browser.mode = "WRITE"
		return show();
	}

	buttons: {
		if (browser.mode == "READ") {
			return [{"title":"Open","enabled":browser.valid},{"title":"Cancel","enabled":"true"}]
		} else {
			return [{
				"title":"Save",
				"enabled":browser.valid,
				"confirm":(willOverwrite !== "" && browser.valid ? "Yes, overwrite " + willOverwrite : undefined)
			},{
				"title":"Cancel",
				"enabled":"true"
			}]
		}
	}
	title: "Select a File"
	component: Component {
		Item {
			FontLoader {
				source: "./fontawesome-webfont.ttf"
			}

			function chdir(_p) {
				var p = _p.toString();
				var path = (p.substr(0,7) == "file://" ? p.substr(7) : p);
				var _res = Panopticon.readDirectory(path);

				console.log(_res);
				folder.clear();

				var res = JSON.parse(_res);

				if(res.status == "ok") {
					res.payload.listing.sort(function(a,b) {
						if(a.is_folder == b.is_folder) {
							return a.name.localeCompare(b.name);
						} else {
							return (a.is_folder ? 0 : 1) - (b.is_folder ? 0 : 1);
						}
					});

					for(var i = 0; i < res.payload.listing.length; i++) {
						if(res.payload.listing[i].name.substr(0,1) != ".") {
							folder.append(res.payload.listing[i]);
						}
					}

					folderView.currentIndex = -1;
					pathInput.text = res.payload.current
					browser.currentPath = res.payload.current
					upButton.parentPath = res.payload.parent
				} else {
					errorPopup.displayMessage("Failed to open directory: " + res.error);
				}

				console.log("chdir() to '" + path.toString() + "'");
			}

			function mark(p) {
				fileInput.text = p
				browser.currentFile = p
				console.log("mark() '" + p.toString() + "'");
			}

			Component.onCompleted: {
				chdir("file:///")
			}

			width: 650; height: 450

			GridLayout {
				rows: 3
				columns: 2
				anchors.fill: parent
				anchors.margins: 5

				Button {
					property string parentPath: "/"

					id: upButton
					text: "Up"
					Layout.row: 0
					Layout.column: 0
					onClicked: chdir(parentPath)
				}

				TextField {
					id: pathInput
					Layout.row: 0
					Layout.column: 1
					Layout.fillWidth: true
					onAccepted: {
						chdir(text)
					}
					Keys.onPressed: {
						folderView.currentIndex = -1
					}
				}

				ScrollView {
					Layout.row: 1
					Layout.column: 0
					Layout.columnSpan: 2
					Layout.fillWidth: true
					Layout.fillHeight: true
					frameVisible: true

					ListView {
						property int titleWidth: 0;

						id: folderView
						anchors.fill: parent
						model: ListModel {
							id: folder
						}
						section.property: "is_folder"
						section.criteria: ViewSection.FullString
						delegate: Item {
							Behavior on height {
								SmoothedAnimation { velocity: 300 }
							}

							width: parent.width
							height: detailsLabel.y + detailsLabel.height
							clip: true

							Rectangle {
								anchors.fill: parent
								border.color: "lightsteelblue"
								border.width: 1
								visible: mousearea.containsMouse
							}

							Rectangle {
								anchors.fill: parent
								color: "lightsteelblue"
								visible: parent.ListView.isCurrentItem
							}

							RowLayout {
								id: entry
								anchors.left: parent.left
								anchors.right: parent.right
								anchors.leftMargin: 3
								anchors.rightMargin: 3
								height: childrenRect.height

								Text {
									verticalAlignment: Text.AlignVCenter;
									font.family: "FontAwesome"
									font.pixelSize: 16
									text: {
										if(is_folder) {
											return (parent.ListView.isCurrentItem ? "\uf07b" : "\uf114");
										} else {
											return (parent.ListView.isCurrentItem ? "\uf15b" : "\uf016");
										}
									}
								}

								Label {
									id: entryName
									Layout.fillWidth: true
									text: name
								}

								Label {
									text: {
										if(!is_folder) {
											var res = JSON.parse(Panopticon.fileDetails(path));
											if(res.status == "ok") {
												if(res.payload.format == "elf") {
													return "ELF";
												} else if(res.payload.format == "panop") {
													return "Panopticon Project";
												} else if(res.payload.format == "pe") {
													return "PE";
												} else {
													return "";
												}

												return res.payload.format;
											}
										}
										return ""
									}
								}
							}

							Column {
								id: detailsLabel
								anchors.top: entry.bottom
								anchors.left: parent.left
								anchors.right: parent.right
								anchors.topMargin: 3
								anchors.leftMargin: entryName.x
								clip: true;

								Repeater {
									model: {
										if(!is_folder) {
											var res = JSON.parse(Panopticon.fileDetails(path));
											if(res.status == "ok") {
												return res.payload.info;
											}
										}
										return [];
									}
									Label {
										height: (parent.parent.ListView.isCurrentItem ? contentHeight : 0)
										text: modelData
									}
								}
							}

							MouseArea {
								id: mousearea
								hoverEnabled: true
								anchors.fill: entry

								onDoubleClicked: {
									folderView.currentIndex = index
									if(is_folder) {
										chdir(path);
									} else {
										mark(name)
									}
									fileInput.accept()
								}

								onClicked: {
									folderView.currentIndex = index
									if(is_folder) {
										chdir(path);
									} else {
										mark(name)
									}
								}
							}
						}
					}
				}

				TextField {
					id: fileInput
					Layout.row: 2
					Layout.column: 0
					Layout.columnSpan: 2
					Layout.fillWidth: true

					placeholderText: "Filename"

					function accept() {
						if(browser.valid) {
							if(browser.willOverwrite != "" && browser.mode == "WRITE") {
								for(var i = 0; i < browser.children.length; i++) {
									if(browser.children[i] != errorPopup &&
										browser.children[i] != confirmOverwrite) {
										browser.children[i].enabled = false;
									}
								}

								var res = confirmOverwrite.displayMessage("Overwrite '" + browser.willOverwrite + "'?");

								for(i = 0; i < browser.children.length; i++) {
									browser.children[i].enabled = true;
								}

								if(res == 0) {
									return;
								}
							}
							browser.done(0)
						}
					}

					onAccepted: accept()
					onTextChanged: {
						browser.valid = (text != "")
						browser.willOverwrite = ""

						if(text == "." || text == "..") {
							browser.valid = false;
						}

						for(var i = 0; i < folder.count; i++) {
							if(folder.get(i).name == text) {
								if(folder.get(i).is_folder) {
									browser.valid = false;
								} else {
									browser.willOverwrite = text
									browser.currentFile = text;
									return;
								}
							}
						}
						browser.currentFile = text;
					}
				}
			}
		}
	}

	ErrorPopup {
		id: confirmOverwrite
		title: "Overwrite?"
		buttons: [{
			"title": "Overwrite",
			"enabled": true
		},{
			"title": "Don't overwrite",
			"enabled": true
		}]
	}

	ErrorPopup {
		id: errorPopup
		title: "Error while browsing"
	}
}

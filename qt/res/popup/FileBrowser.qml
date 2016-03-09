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

	// NEW, OPEN or SAVE
	property string mode: "NEW"
	property bool valid: false
	property string willOverwrite: ""
	property string selectedFile: ""

	function newFile() {
		browser.mode = "NEW"
		return show()
	}

	function openFile() {
		browser.mode = "OPEN"
		return show()
	}

	function saveFile() {
		browser.mode = "SAVE"
		return show()
	}

	buttons: {
		if(browser.mode == "NEW") {
			return [{"title":"Cancel","enabled":"true"}]
		} else if (browser.mode == "OPEN") {
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
						if(a.category == b.category) {
							var r = a.name.localeCompare(b.name);
						} else {
							var ord = ["folder","file","misc"];
							var r = ord.indexOf(a.category) - ord.indexOf(b.category);
						}
						return r;
					});

					for(var i = 0; i < res.payload.listing.length; i++) {
						if(res.payload.listing[i].name.substr(0,1) != ".") {
							folder.append(res.payload.listing[i]);
						}
					}

					folderView.currentIndex = -1;
					pathInput.text = res.payload.current
					upButton.parentPath = res.payload.parent
				} else {
					errorPopup.displayMessage("Failed to open directory: " + res.error);
				}

				console.log("chdir() to '" + path.toString() + "'");
			}

			function mark(p) {
				fileInput.text = p
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
						id: folderView
						anchors.fill: parent
						model: ListModel {
							id: folder
						}
						section.property: "category"
						section.criteria: ViewSection.FullString
						delegate: Item {
							height: shortLabel.height + detailsLabel.height
							width: parent.width

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

							Text {
								x: 3
								height: shortLabel.height
								verticalAlignment: Text.AlignVCenter;
								font.family: "FontAwesome"
								text: {
									if(category == "folder") {
										return (parent.ListView.isCurrentItem ? "\uf07b" : "\uf114");
									} else if(category == "file") {
										return (parent.ListView.isCurrentItem ? "\uf15c" : "\uf0f6");
									} else {
										return (parent.ListView.isCurrentItem ? "\uf15b" : "\uf016");
									}
								}
							}

							Label {
								id: shortLabel
								x: 25
								text: name
								width: parent.width
							}

							RowLayout {
								id: detailsLabel
								anchors.top: shortLabel.bottom
								anchors.topMargin: 3
								anchors.bottomMargin: 5
								x: 25
								clip: true;
								height: { parent.ListView.isCurrentItem && category != "folder" && browser.mode == "NEW" ? childrenRect.height + 5 : 0 }
								width: parent.width
								visible: browser.mode == "NEW"

								Behavior on height {
									SmoothedAnimation { velocity: 300 }
								}

								Column {
									Repeater {
										model: details.length
										Label {
											text: details[index]
										}
									}
								}

								Button {
									Layout.alignment: (details.length === "0" ? Qt.AlignLeft : Qt.AlignRight)
									Layout.rightMargin: 50
									Layout.bottomMargin: 10

									text: "Open"
									menu: Menu {
										MenuItem {
											text: "...as ELF"
										}

										MenuItem {
											text: "...as raw data"
										}

										MenuItem {
											text: "...as AVR image"
										}

										MenuItem {
											text: "...as MOS 6502 image"
										}
									}
								}
							}

							MouseArea {
								id: mousearea
								hoverEnabled: true
								anchors.fill: shortLabel

								onDoubleClicked: {
									fileInput.accept()
								}

								onClicked: {
									console.log(JSON.stringify(details));
									folderView.currentIndex = index
									if(category == "folder") {
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
							if(browser.willOverwrite != "") {
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
									browser.done(0);
								}
							}
							browser.selectedFile = pathInput.text + "/" + fileInput.text
							browser.done(1)
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
								if(folder.get(i).category == "folder") {
									browser.valid = false;
								} else {
									browser.willOverwrite = text
								}
							}
						}
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

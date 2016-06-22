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

import QtQuick 2.3
import QtQuick.Controls 1.2
import QtQuick.Layouts 1.1

import Panopticon 1.0

Popup {
	id: browser

	// READ or WRITE
	property string mode: "READ"
	property bool valid: false
	property string willOverwrite: ""
	property string selectedFile: currentPath + Panopticon.pathDelimiter + currentFile
	property string currentPath: ""
	property string currentFile: ""
	property string message: ""

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
					view.state = "FOLDER";
				} else {
					view.state = "ERROR";
					errorView.text = "Failed to open directory";
				}

				console.log("chdir() to '" + path.toString() + "'");
			}

			function mark(p) {
				fileInput.text = p
				browser.currentFile = p
				console.log("mark() '" + p.toString() + "'");
			}

			Component.onCompleted: {
				chdir("")
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

				Item {
					id: view
					Layout.row: 1
					Layout.column: 0
					Layout.columnSpan: 2
					Layout.fillWidth: true
					Layout.fillHeight: true

					// FOLDER or ERROR
					state: "FOLDER"

					Label {
						id: errorView
						anchors.fill: parent
						visible: parent.state == "ERROR"
						horizontalAlignment: Text.AlignHCenter
						verticalAlignment: Text.AlignVCenter
						wrapMode: Text.WordWrap
						font.pixelSize: 21
						color: "#333"
					}

					ScrollView {
						frameVisible: true
						anchors.fill: parent
						visible: parent.state == "FOLDER"

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

								width: parent == null ? 1 : parent.width
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
				}

				TextField {
					id: fileInput
					Layout.row: 2
					Layout.column: 0
					Layout.columnSpan: 2
					Layout.fillWidth: true

					textColor: (browser.valid ? "#000" : "#ff0000")
					placeholderText: "Filename"

					function accept() {
						if(browser.valid && !(browser.willOverwrite != "" && browser.mode == "WRITE")) {
							browser.done(0)
						}
					}

					onAccepted: accept()
					onDisplayTextChanged: {
						if (displayText != "") {
							var res = JSON.parse(Panopticon.fileDetails(currentPath + Panopticon.pathDelimiter + displayText))

							if(res.status == "ok") {
								switch(res.payload.state) {
									case "readable": {
										browser.valid = (browser.mode == "READ");
										browser.willOverwrite = displayText
										browser.message = (browser.mode == "READ" ? "" : "File exists and can't be overwritten.")
										break;
									}
									case "writable": {
										browser.valid = true;
										browser.willOverwrite = displayText
										browser.message = "";
										break;
									}
									case "directory": {
										browser.valid = false;
										browser.willOverwrite = displayText
										browser.message = "There already exists a directory with that name."
										break;
									}
									case "inaccessible": {
										browser.valid = false;
										browser.willOverwrite = ""
										browser.message = "No write permissions."
										break;
									}
									case "free": {
										browser.valid = (browser.mode == "WRITE");
										browser.willOverwrite = ""
										browser.message = (browser.mode == "WRITE" ? "" : "No such file.")
										break;
									}
									default: {
										console.error("Invalid state '" + res.payload.state.toString() + "'")
									}
								}
							} else {
								console.error(res.error);
							}
						} else {
							browser.valid = false;
							browser.willOverwrite = ""
							browser.message = ""
						}

						browser.currentFile = displayText;
					}

					Canvas {
						readonly property int tipHeight: 12
						readonly property int bubblePadding: 8
						readonly property int bubbleRadius: 4

						id: edit
						width: messageLabel.width + 2 * bubblePadding
						height: messageLabel.height + 2 * bubblePadding + tipHeight
						anchors.top: parent.bottom
						anchors.left: parent.left
						visible: browser.message !== ""

						onVisibleChanged: requestPaint()

						onPaint: {
							var ctx = edit.getContext('2d');

							const corner_sz = edit.bubbleRadius;
							const tip_apex = 25;
							const tip_w = 20;
							const tip_h = edit.tipHeight;

							/*
							 *       tip_apex
							 *          /\
							 * .-------'  `-----. - top
							 * |       | tip_end|
							 * |    tip_start   |
							 * '----------------' - bottom
							 */

							const top = tip_h;
							const bottom = edit.height - 1;
							const tip_start = tip_apex - tip_w / 2;
							const tip_end = tip_start + tip_w;
							const end = edit.width - 1;


							ctx.fillStyle = "#efecca";
							ctx.strokeStyle = "black";
							ctx.lineWidth = 0.5;

							ctx.clearRect(0,0,width,height);
							ctx.beginPath();

							ctx.moveTo(1 + corner_sz,top);
							ctx.lineTo(tip_start,top);
							ctx.lineTo(tip_apex,0);
							ctx.lineTo(tip_end,top);
							ctx.lineTo(end - corner_sz,top);
							ctx.arc(end - corner_sz,top + corner_sz,corner_sz,1.5 * Math.PI,0,false);
							ctx.lineTo(end,bottom - corner_sz);
							ctx.arc(end - corner_sz,bottom - corner_sz,corner_sz,0,0.5 * Math.PI,false);
							ctx.lineTo(1 + corner_sz,bottom);
							ctx.arc(1 + corner_sz,bottom - corner_sz,corner_sz,0.5 * Math.PI,Math.PI,false);
							ctx.lineTo(1,top + corner_sz);
							ctx.arc(1 + corner_sz,top + corner_sz,corner_sz,Math.PI,1.5 * Math.PI,false);

							ctx.fill();
							ctx.stroke();
						}
					}

					Label {
						id: messageLabel
						text: browser.message
						anchors.top: parent.bottom
						anchors.left: parent.left
						anchors.topMargin: 12 + 8
						anchors.leftMargin: 8
						width: contentWidth
						height: contentHeight
						visible: browser.message != ""
					}
				}
			}
		}
	}
}

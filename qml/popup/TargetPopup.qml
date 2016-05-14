import QtQuick 2.3
import QtQuick.Controls 1.2
import QtQuick.Layouts 1.1

import Panopticon 1.0

Popup {
	id: root

	property bool valid: true
	property var openFunction: null

	buttons: [
		{
			"title":"Open",
			"enabled":root.valid,
		},{
			"title":"Cancel",
			"enabled":"true"
		}]
	title: "Select a File"
	component: Component {
		Item {
			width: 650; height: 620

			FontLoader {
				source: "./fontawesome-webfont.ttf"
			}

			ScrollView {
				anchors.leftMargin: 10
				anchors.rightMargin: 10
				anchors.topMargin: 10
				anchors.bottomMargin: 10
				anchors.fill: parent

				Column {
					ColumnLayout {
						spacing: 10

						Label {
							font.pixelSize: 20
							Layout.bottomMargin: 10
							text: "Overview"
						}

						Label {
							text: "File format"
							font.pixelSize: 16
						}

						Label {
							Layout.leftMargin: 20
							Layout.maximumWidth: 400
							wrapMode: Text.WordWrap
							text: "Select how the contents of the file should be interpreted."
						}

						ComboBox {
							id: formatBox
							Layout.preferredWidth: 150
							Layout.leftMargin: 20
							model: ["Unrecognized", "Memory Image", "COM"]
							currentIndex: 1
							onCurrentIndexChanged: {
								switch(currentIndex) {
									case 0: {
										pageLoader.sourceComponent = emptyPage;
										root.valid = true;
										root.openFunction = function(path) { return Panopticon.createDataProject(path) };
										break;
									}
									case 1: {
										pageLoader.sourceComponent = rawPage;
										break;
									}
									case 4: {
										pageLoader.sourceComponent = emptyPage;
										root.valid = true;
										root.openFunction = function(path) { return Panopticon.createComProject(path) };
										break;
									}
									default: {
										console.error("Unknown format");
										break;
									}
								}
							}
						}
					}

					Loader {
						id: pageLoader
						sourceComponent: rawPage
					}
				}
			}

			Component {
				id: emptyPage
				Item {}
			}

			Component {
				id: rawPage

				ColumnLayout {
					spacing: 10

					function updateValid() {
						root.valid = !visible ||
						((!singleEntry.checked || entryPoint.state == "") && loadAddress.state == "");
					}

					Component.onCompleted: {
						updateValid()
						root.openFunction = function(path) {
							var tgt = targetModel.get(targetSelect.currentIndex).ident
							return Panopticon.createRawProject(path,tgt,loadAddress.text,singleEntry.checked ? entryPoint.text : -1)
						};
					}

					Label {
						Layout.topMargin: 20
						text: "Basic"
						font.pixelSize: 16
					}

					Label {
						Layout.leftMargin: 20
						Layout.maximumWidth: 400
						wrapMode: Text.WordWrap
						text: "<strong>Microcontroller to assume for analysis</strong>. This option defines what instructions are supported and the size of the Program Counter register."
					}

					ComboBox {
						Layout.leftMargin: 20
						Layout.bottomMargin: 20
						Layout.preferredWidth: 150
						id: targetSelect
						model: targetModel

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
								text: "ATmega8"
								ident: "atmega8"
							}
							ListElement {
								text: "ATmega88"
								ident: "atmega88"
							}
						}
					}

					Label {
						Layout.leftMargin: 20
						Layout.maximumWidth: 400
						wrapMode: Text.WordWrap
						text: "<strong>Image load address</strong>. Start of the image inside uC flash. Setting the to something other than 0 is useful if the file to analyse isn't a complete flash dump but needs to be loaded at a certain address. You may need to change the entry point too."
					}

					TextField {
						Layout.leftMargin: 20
						Layout.bottomMargin: 20
						id: loadAddress
						text: "0"
						state: ""
						textColor: {
							if(state == "") {
								return "#000000";
							} else {
								return "#ff0000";
							}
						}
						onTextChanged: {
							var num = parseInt(text);
							if(num != NaN && num >= 0) {
								state = ""
							} else {
								state = "INVALID"
							}
							updateValid();
						}
					}

					Label {
						Layout.leftMargin: 20
						Layout.maximumWidth: 400
						wrapMode: Text.WordWrap
						text: "<strong>Entry point(s) of the image.</strong> This option sets the starting point(s) for disassembly. The default is to expect the standard interrupt vector table at the start of the image. In case the image is not a full flash dump a single entry point can be set here."
					}

					ColumnLayout {
						Layout.leftMargin: 20
						ExclusiveGroup { id: entryPointGroup }
						RadioButton {
							text: "Interrupt Vector Table"
							checked: true
							exclusiveGroup: entryPointGroup
							onCheckedChanged: updateValid();
						}

						RadioButton {
							id: singleEntry
							text: "Single Entry Point"
							exclusiveGroup: entryPointGroup
							onCheckedChanged: updateValid();
						}

						TextField {
							Layout.leftMargin: 18
							Layout.bottomMargin: 20

							enabled: singleEntry.checked
							id: entryPoint
							text: "0"
							textColor: {
								if(state == "") {
									return "#000000";
								} else {
									return "#ff0000";
								}
							}
							onTextChanged: {
								var num = parseInt(text);
								if(num != NaN && num >= 0) {
									state = ""
								} else {
									state = "INVALID"
								}
								updateValid();
							}
						}
					}
				}
			}
		}
	}
}

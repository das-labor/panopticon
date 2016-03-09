import QtQuick 2.5
import QtQuick.Controls 1.4
import QtQuick.Layouts 1.2

ApplicationWindow {
	width: 1000; height: 1000

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
					Layout.leftMargin: 20
					model: ["Unregocnized", "Memory Image", "ELF", "PE", "COM"]
					onCurrentIndexChanged: {
						switch(currentIndex) {
							default:
							case 0: pageLoader.sourceComponent = rawPage; break;
							case 1: pageLoader.sourceComponent = avrPage; break;
							case 2: pageLoader.sourceComponent = rawPage; break;
							case 3: pageLoader.sourceComponent = rawPage; break;
							case 4: pageLoader.sourceComponent = rawPage; break;
							case 5: pageLoader.sourceComponent = rawPage; break;
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
		id: rawPage

		Item {}
	}

	Component {
		id: avrPage

		ColumnLayout {
			spacing: 10

			Label {
				Layout.topMargin: 20
				text: "Basic"
				font.pixelSize: 16
			}

			Label {
				Layout.leftMargin: 20
				Layout.maximumWidth: 400
				wrapMode: Text.WordWrap
				text: "<h4>Microcontroller to assume for analysis</h4>. This option defines what instructions are supported and the size of the Program Counter register."
			}

			ComboBox {
				Layout.leftMargin: 20
				Layout.bottomMargin: 20
				model: ["Atmega 103", "Atmega 88", "Atmega 8", "MOS-6502", "Intel 386 (16 Bits)", "Intel 686 (32 Bits)", "AMD64" ]
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
				text: "0"
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
				}

				RadioButton {
					id: singleEntry
					text: "Single Entry Point"
					exclusiveGroup: entryPointGroup
				}

				TextField {
					enabled: singleEntry.checked

					Layout.leftMargin: 18
					Layout.bottomMargin: 20
					text: "0"
				}
			}
		}
	}
}

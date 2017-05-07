import QtQuick 2.4
import QtQuick.Controls 1.3 as Ctrl
import QtQuick.Layouts 1.1
import QtQuick.Dialogs 1.0

Rectangle {
	color: "#fdfdfd"

	FontLoader { source: "../fonts/SourceSansPro-Semibold.ttf" }
	FontLoader { source: "../fonts/SourceSansPro-Regular.ttf" }
	FontLoader { source: "../fonts/SourceCodePro-Regular.ttf" }

	function open() {
		scrolled.open()
	}

	Ctrl.ScrollView {
		id: scroll
		anchors.fill: parent

		Item {
			id: scrolled
			height: childrenRect.height + childrenRect.y + 100
			width: Math.max(childrenRect.width + childrenRect.x,scroll.viewport.width)

			function open() {
				var diag = fileDialog.createObject(view)
				diag.open();
			}

			// Panopticon logo font
			Image {
				id: logo
				x: 68; y: 70
				source: "../icons/logo.png"
			}

			Component {
				id: fileDialog

				FileDialog {
					id: fileDialog
					title: "Please choose a file"
					folder: shortcuts.home
					onAccepted: {
						var p = fileDialog.fileUrls.toString().substring(7);
						Panopticon.open_program(p)
					}
					Component.onCompleted: visible = true
				}
			}

			// Welcome text
			Ctrl.Label {
				id: view
				anchors.left: logo.left
				anchors.top: logo.bottom
				anchors.topMargin: 50

				width: logo.width + 10
				height: 250
				textFormat: Text.RichText
				wrapMode: Text.Wrap
				lineHeight: 1.2

				font {
					family: "Source Sans Pro"; pointSize: 13
				}

				text: '<style>body { font-family: "Source Sans Pro"; font-size: 35px; line-height: 1.6; letter-spacing: 0px; background: #fdfdfd; } a { color: #4a95e2; } tt { font-family: "Source Code Pro"; font-size: 16px; }</style>This is version <b>0.16.0</b> of Panopticon. To start working you need to <a href="panop:open">open a new file</a> or pick one from your previous sessions below.</p><p>For more <b>usage</b> information check out <tt>panopticon.re</tt>. If you think you found a bug please open an issue in our <b>bug tracker</b> at <tt>https://github.com/das-labor/panopticon/issues</tt>.</p>'

				onLinkActivated: {
					if(link == "panop:open") {
						scrolled.open()
					}
				}

				MouseArea {
					cursorShape: (view.hoveredLink != "" ? Qt.PointingHandCursor : Qt.ArrowCursor)
					anchors.fill: parent
					acceptedButtons: Qt.NoButton
				}
			}

		// Recent sessions
			GridLayout {
				id: layout
				anchors.left: view.left
				anchors.top: view.bottom
				anchors.topMargin: 20

				visible: Panopticon.haveRecentSessions
				width: logo.width
				columns: 4
				rowSpacing: 15
				columnSpacing: 40

				// Modified
				Repeater {
					model: Panopticon.recentSessions
					Ctrl.Label {
						Layout.column: 0;
						Layout.row: (index + 1) * 2
						Layout.alignment: Qt.AlignHCenter

						color: "#aaa9b0"
						text: "Yesterday"
						font {
							family: "Source Sans Pro"; pointSize: 11
						}
					}
				}

				// Title
				Repeater {
					model: Panopticon.recentSessions
					Column {
						Layout.column: 1;
						Layout.row: (index + 1) * 2
						Layout.fillWidth: true
						Layout.maximumWidth: Number.POSITIVE_INFINITY
						Layout.minimumWidth: 100
						Ctrl.Label {
							text: model.title
							font {
								family: "Source Sans Pro"; pointSize: 13; weight: Font.DemiBold;
							}
							horizontalAlignment: Text.AlignLeft
						}
						Ctrl.Label {
							text: model.typ
							font {
								family: "Source Sans Pro"; pointSize: 11
							}
							horizontalAlignment: Text.AlignLeft
						}
					}


				}
				// Actions
				Repeater {
					model: Panopticon.recentSessions
					Ctrl.Label {
						Layout.column: 3;
						Layout.row: (index + 1) * 2
						Layout.alignment: Qt.AlignHCenter

						text: "Continue"
						horizontalAlignment: Text.AlignHCenter
						font {
							family: "Source Sans Pro"; pointSize: 11; underline: mousearea.containsMouse
						}
						color: "#4a95e2"

						MouseArea {
							id: mousearea
							anchors.fill: parent
							hoverEnabled: true
							cursorShape: (containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor)
							onClicked: {
								Panopticon.open_program(path)
							}
						}
					}
				}
			}
		}
	}
}

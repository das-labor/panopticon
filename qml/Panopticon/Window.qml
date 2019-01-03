import QtQuick 2.4
import QtQuick.Controls 1.3 as Ctrl
import QtQuick.Layouts 1.1
import QtQuick.Dialogs 1.0
import QtGraphicalEffects 1.0
import Panopticon 1.0

import "."

Ctrl.ApplicationWindow {
	id: mainWindow

	title: "Panopticon"
	width: 1024
	height: 768
	visible: true
	menuBar: Ctrl.MenuBar {
		Ctrl.Menu {
			title: "File"
			Ctrl.MenuItem {
				text: "Open"
				action: Ctrl.Action {
					text: "Open"
					shortcut: StandardKey.Open
					enabled: Panopticon.currentSession == ""
					onTriggered: {
						workspace.state = "welcomeState"
						welcome.open()
					}
				}
			}
			Ctrl.MenuItem {
				text: "Save"
				action: Ctrl.Action {
					text: "Save"
					shortcut: StandardKey.Save
					enabled: Panopticon.currentSession != ""
					onTriggered: { Panopticon.saveSession(Panopticon.currentSession) }
				}
			}
			Ctrl.MenuItem {
				text: "Save as..."
				action: Ctrl.Action {
					text: "Save as..."
					shortcut: StandardKey.SaveAs
					enabled: Panopticon.currentSession != ""
					onTriggered: {
						var diag = fileDialog.createObject(mainWindow)
						diag.open();
					}
				}
			}
			Ctrl.MenuItem {
				text: "Quit"
				action: Ctrl.Action {
					text: "Quit"
					shortcut: StandardKey.Quit
					onTriggered: { Qt.quit() }
				}
			}
		}

		Ctrl.Menu {
			title: "Edit"
			Ctrl.MenuItem {
				text: "Undo"
				action: Ctrl.Action {
					text: "Undo"
					shortcut: StandardKey.Undo
					enabled: Panopticon.canUndo
					onTriggered: { Panopticon.undo() }
				}
			}
			Ctrl.MenuItem {
				text: "Redo"
				action: Ctrl.Action {
					text: "Redo"
					shortcut: StandardKey.Redo
					enabled: Panopticon.canRedo
					onTriggered: { Panopticon.redo() }
				}
			}
			//Ctrl.MenuItem { text: "Erase Values" }
		}

		Ctrl.Menu {
			title: "View"
			//Ctrl.MenuItem { text: "Back" }
			//Ctrl.MenuItem { text: "Forward" }
			//Ctrl.MenuItem { text: "Jump To..." }
			Ctrl.MenuItem {
				action: Ctrl.Action {
					text: "Center Entry Point"
					enabled: workspace.state == "functionState"
					onTriggered: { controlflow.centerEntryPoint() }
				}
			}
		}

		Ctrl.Menu {
			title: "Help"
			//Ctrl.MenuItem { text: "Documentation" }
			Ctrl.MenuItem {
				text: "About"
				action: Ctrl.Action {
					text: "About"
					onTriggered: { workspace.state = "welcomeState" }
				}
			}
		}
	}

	Component.onCompleted: {
		if(Panopticon.initialFile !== "") {
			Panopticon.openProgram(Panopticon.initialFile);
		}
	}

	Component {
		id: fileDialog

		FileDialog {
			id: fileDialog
			title: "Please choose a file"
			folder: shortcuts.home
			selectExisting: false
			selectMultiple: false
			onAccepted: {
				var p = fileDialog.fileUrls.toString().substring(7);
				Panopticon.saveSession(p)
			}
			Component.onCompleted: visible = true
		}
	}

	Item {
		id: workspace

		anchors.fill: parent
		state: "welcomeState"
		states: [
			State {
				name: "functionState"
				PropertyChanges { target: controlflow; visible: true }
				PropertyChanges { target: welcome; visible: false }
			},
			State {
				name: "welcomeState"
				PropertyChanges { target: controlflow; visible: false }
				PropertyChanges { target: welcome; visible: true }
			}
		]

		Sidebar {
			id: bar
			anchors.top: parent.top
			anchors.bottom: parent.bottom
			width: Math.min(parent.width * 0.3, 400)
			z: 2

			onShowControlFlowGraph: {
				controlflow.showControlFlowGraph(uuid)
				parent.state = "functionState"
			}
		}

		Welcome {
			id: welcome
			anchors.left: bar.right
			anchors.right: parent.right
			anchors.top: parent.top
			anchors.bottom: parent.bottom
		}

		ControlFlowWidget {
			id: controlflow
			anchors.left: bar.right
			anchors.right: parent.right
			anchors.top: parent.top
      anchors.bottom: parent.bottom

      onFunctionUuidChanged: {
        bar.functionUuid = functionUuid;
      }
		}

		LinearGradient {
			id: gradient
			anchors.left: parent.left
			anchors.right: parent.right
			anchors.top: parent.top
			height: 3
			start: Qt.point(0, 0)
			end: Qt.point(0, 3)
			gradient: Gradient {
				GradientStop { position: 0.0; color: "#f0f0f0" }
				GradientStop { position: 1.0; color: "transparent" }
			}
		}
	}
}

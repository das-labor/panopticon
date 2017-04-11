import QtQuick 2.4
import QtQuick.Controls 1.3 as Ctrl
import QtQuick.Layouts 1.1
import QtGraphicalEffects 1.0
import Panopticon 1.0

Ctrl.ApplicationWindow {
	id: mainWindow

	title: "Panopticon"
	width: 1024
	height: 768
	visible: true
	menuBar: Ctrl.MenuBar {
		Ctrl.Menu {
			title: "File"
			Ctrl.MenuItem { text: "Open..." }
			Ctrl.MenuItem { text: "Save" }
			Ctrl.MenuItem { text: "Save As..." }
			Ctrl.MenuItem { text: "Quit" }
		}

		Ctrl.Menu {
			title: "Edit"
			Ctrl.MenuItem { text: "Undo" }
			Ctrl.MenuItem { text: "Redo" }
			Ctrl.MenuItem { text: "Erase Values" }
		}

		Ctrl.Menu {
			title: "View"
			Ctrl.MenuItem { text: "Back" }
			Ctrl.MenuItem { text: "Forward" }
			Ctrl.MenuItem { text: "Jump To..." }
			Ctrl.MenuItem { text: "Center Entry Point" }
		}

		Ctrl.Menu {
			title: "Help"
			Ctrl.MenuItem { text: "Documentation" }
			Ctrl.MenuItem { text: "About" }
		}
	}

	Timer {
		id: callbackTimer
		interval: 1
		running: false
		onTriggered: {
			Panopticon.callback()
		}
	}

	Component.onCompleted: {
		Panopticon.call_me_maybe.connect(function() {
			callbackTimer.start()
		});

		if(Panopticon.initialFile !== "") {
			Panopticon.open_program(Panopticon.initialFile);
		}
	}

	Item {
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
			width: 250

			onShowControlFlowGraph: {
				controlflow.functionUuid = uuid
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

		 ControlFlowGraph {
			 id: controlflow
			 anchors.left: bar.right
			 anchors.right: parent.right
			 anchors.top: parent.top
			 anchors.bottom: parent.bottom

			 onShowControlFlowGraph: {
				 controlflow.functionUuid = uuid
				 parent.state = "functionState"
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

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

	Item {
		anchors.fill: parent
		state: "welcomeState"
		states: [
			State {
				name: "functionState"
				PropertyChanges { target: controlflow; visible: true }
				PropertyChanges { target: callgraph; visible: false }
				PropertyChanges { target: welcome; visible: false }
			},
			State {
				name: "programState"
				PropertyChanges { target: controlflow; visible: false }
				PropertyChanges { target: callgraph; visible: true }
				PropertyChanges { target: welcome; visible: false }
			},
			State {
				name: "welcomeState"
				PropertyChanges { target: controlflow; visible: false }
				PropertyChanges { target: callgraph; visible: false }
				PropertyChanges { target: welcome; visible: true }
			}
		]

		Sidebar {
			id: bar
			anchors.top: parent.top
			anchors.bottom: parent.bottom
			width: 250

			onShowCallGraph: {
				callgraph.programUuid = uuid
				parent.state = "programState"
			}

			onShowControlFlowGraph: {
				controlflow.functionUuid = uuid
				parent.state = "functionState"
			}
		}

		/*
		 Rectangle {
			 id: toolbar

			 anchors.left: parent.left
			 anchors.right: parent.right
			 anchors.top: parent.top

			 height: 40
			 color: "white"

			 RowLayout {
				 anchors.leftMargin: 10
				 anchors.rightMargin: 10
				 anchors.fill: parent
				 spacing: 15

				 ToolbarItem {
					 Layout.preferredHeight: 18
					 Layout.alignment: Qt.AlignVCenter
					 icon: '../icons/arrow-left.svg'

					 onActivate: {
						 console.log("back");
					 }
				 }

				 ToolbarItem {
					 Layout.preferredHeight: 18
					 icon: '../icons/arrow-right.svg'

					 onActivate: {
						 console.log("forward");
					 }
				 }

				 Item { Layout.fillWidth: true }

				 ToolbarItem {
					 Layout.preferredHeight: 18
					 icon: '../icons/undo.svg'
					 title: 'Undo'

					 onActivate: {
						 console.log("undo");
					 }
				 }

				 ToolbarItem {
					 Layout.preferredHeight: 18
					 icon: '../icons/repeat.svg'
					 title: 'Redo'

					 onActivate: {
						 console.log("redo");
					 }
				 }

				 ToolbarItem {
					 Layout.leftMargin: 25
					 Layout.preferredHeight: 18
					 icon: '../icons/home.svg'
					 title: 'Entry'

					 onActivate: {
						 console.log("entry");
					 }
				 }

				 ToolbarItem {
					 Layout.preferredHeight: 18
					 icon: '../icons/eraser.svg'
					 title: 'Reset'

					 onActivate: {
						 console.log("reset");
					 }
				 }
			 }
		 }*/

		 Welcome {
			 id: welcome
			 anchors.left: bar.right
			 anchors.right: parent.right
			 anchors.top: parent.top
			 anchors.bottom: parent.bottom
		 }

		 CallGraph {
			 id: callgraph
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

		 Rectangle {
			 id: undo
			 width: 100; height: 100
			 color: "red"
			 visible: Panopticon.canUndo

			 Text {
				 anchors.fill: parent
				 text: "can undo"
			 }

			 MouseArea {
				 anchors.fill: parent
				 onClicked: {
					 Panopticon.undo()
				 }
			 }
		 }
		 Rectangle {
			 anchors.top: undo.bottom
			 width: 100; height: 100
			 color: "red"
			 visible: Panopticon.canRedo

			 Text {
				 anchors.fill: parent
				 text: "can redo"
			 }

			 MouseArea {
				 anchors.fill: parent
				 onClicked: {
					 Panopticon.redo()
				 }
			 }
		 }
	 }
 }

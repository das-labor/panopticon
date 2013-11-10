import QtQuick.Controls 1.0
import QtQuick.Dialogs 1.0
import QtQuick 2.0

ApplicationWindow {
	id: mainWindow
	title: "Panopticon"
	height: 1000
	width: 1000

	Action {
		id: openAction
		text: "Open"
		shortcut: "Ctrl+O"
		onTriggered: { stackView.push(Qt.createComponent("Workspace.qml")) }//{ fileDialog.open() }
		tooltip: "Open an Blob"
	}

	Action {
		id: closeAction
		text: "Close"
		shortcut: "Ctrl+C"
		onTriggered: { fileDialog.open() }
		tooltip: "Open an Blob"
	}

	FileDialog {
		id: fileDialog
		title: "Please choose a file"

		onAccepted: { stackView.push(Qt.createComponent("Workspace.qml")) }
		onRejected: { console.log("Canceled") }
	}

	StackView {
		id: stackView
		anchors.fill: parent

		Button {
			anchors.centerIn: parent
			action: openAction
		}
	}
}

import QtQuick.Controls 1.0
import QtQuick.Dialogs 1.0
import QtQuick 2.0

ApplicationWindow {
	id: mainWindow
	title: "Panopticon"
	height: 1000
	width: 1000

	Loader {
		anchors.fill: parent
		source: "wizard/Main.qml"
	}
}

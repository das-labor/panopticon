import QtQuick.Controls 1.0
import QtQuick.Dialogs 1.0
import QtQuick 2.0
import Panopticon 1.0

ApplicationWindow {
	id: mainWindow
	title: "Panopticon"
	height: 1000
	width: 1000

	Loader {
		id: loader
		anchors.fill: parent
	}

	Component.onCompleted: {
		if(Panopticon.session)
		{
			loader.setSource("workspace/Workspace.qml",{ "session": Panopticon.session })
		}
		else
			loader.setSource("wizard/Main.qml")
	}
}

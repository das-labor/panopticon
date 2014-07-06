import QtQuick 2.0
import QtQuick.Controls 1.0
import Panopticon 1.0

Item {
	id: root
	property variant session: null

	Component {
		id: hexdump

		Linear {
			anchors.fill: parent
			//session: root.session
		}
	}

	TabView {
		id: notebook
		anchors.fill: parent
	}

	Component.onCompleted: { notebook.addTab("Hexdump",hexdump) }
}

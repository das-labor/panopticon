import QtQuick 2.0
import QtQuick.Controls 1.0
import Panopticon 1.0

Item {
	id: root

	property variant session: null

	anchors.fill: parent

	ListView {
		width: 150
		height: root.height
		model: root.session.procedures
		delegate: Text {
			height: 40
			text: modelData
			verticalAlignment: Text.AlignVCenter
		}
	}

	Linear {
		id: lst1
		session: root.session
		width: root.width - 150
		x: 150
		height: root.height
	}
}

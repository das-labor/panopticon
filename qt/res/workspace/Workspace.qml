import QtQuick 2.0
import QtQuick.Controls 1.0
import Panopticon 1.0

Item {
	id: root

	property variant session: null

	anchors.fill: parent
	focus: true
	state: "graph"
	Keys.enabled: true
	Keys.onPressed: {
		if(event.key == Qt.Key_Space) {
			if(root.state == "graph") {
				root.state = "linear"
			} else {
				root.state = "graph"
			}
			event.accepted = true
		}
	}

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

	Graph {
		width: root.width - 150
		x: 150
		height: root.height
		id: grph
		session: root.session
		visible: root.state == "graph"
	}

	Linear {
		width: root.width - 150
		x: 150
		height: root.height
		id: lst1
		session: root.session
		visible: root.state == "linear"
	}
}

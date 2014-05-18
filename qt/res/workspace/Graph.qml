import QtQuick 2.1
import QtQuick.Controls 1.1
import Panopticon 1.0

Item {
	id: root
	property color edgeColor: "red"
	property int edgeWidth: 3

	Component {
		id: node

		Rectangle {
			color: "red"

			/*ListView {
				model: mnemonics
				delegate: Text {
					text: mnemonic
				}
			}*/
			height: 100; width: 100

			MouseArea {
				anchors.fill: parent
				drag.target: parent
				hoverEnabled: true

				onPositionChanged: {
					if(pressed) {
						sugiyama.direct = true
						edgeColor = "gray"
						sugiyama.route()
					}
				}
				onReleased: {
					sugiyama.direct = false
					edgeColor = "red"
					sugiyama.route()
				}
				onEntered: {
					for(var i in incoming) {
						incoming[i].color = "blue"
					}
				}
				onExited: {
					for(var i in incoming) {
						incoming[i].color = "red"
					}
				}
			}
		}
	}

	Component {
		id: arrow

		Rectangle {
			color: "green"
			height: 30; width: 30
		}
	}

	ScrollView {
		anchors.fill: parent

		Sugiyama {
			id: sugiyama

			height: childrenRect.height + childrenRect.y
			width: childrenRect.width + childrenRect.x

			delegate: node

			Edge { id: e1; from: 0; to: 1; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e2; from: 0; to: 2; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e3; from: 2; to: 3; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e4; from: 3; to: 4; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e5; from: 3; to: 5; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e6; from: 3; to: 6; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e7; from: 5; to: 6; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e8; from: 6; to: 7; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e9; from: 6; to: 8; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e10; from: 7; to: 8; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e11; from: 6; to: 6; color: root.edgeColor; width: root.edgeWidth; head: arrow }

			vertices: [0,1,2,3,4,5,6,7,8]
			edges: [e1,e2,e3,e4,e5,e6,e7,e8,e9,e10,e11]
		}
	}
}

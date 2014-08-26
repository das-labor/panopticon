import QtQuick 2.0
import QtQuick.Controls 1.0
import Panopticon 1.0

Item {
	id: root
	property color edgeColor: "red"
	property int edgeWidth: 3
	property var session: null

	onSessionChanged: {
		if(session != null) {
			session.graph.jumpsChanged.connect(sugiyama.rebuildEdges)
			sugiyama.rebuildEdges()
		}
	}

	Component {
		id: node

		Rectangle {
			color: "red"

			Text {
				anchors.fill: parent
				text: modelData
			}

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

	Component {
		id: edge

		Edge {
			color: root.edgeColor
			width: root.edgeWidth
			head: arrow
		}
	}

	ScrollView {
		anchors.fill: parent

		Sugiyama {
			id: sugiyama

			height: childrenRect.height + childrenRect.y
			width: childrenRect.width + childrenRect.x
			delegate: node

			function rebuildEdges() {
				for(var a in root.session.graph.jumps) {
					var e = eval(root.session.graph.jumps[a])
					var x = edge.createObject(sugiyama,e)

					sugiyama.edges = [].concat(sugiyama.edges,[x])
				}
			}

			/*Edge { id: e2; from: 0; to: 2; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e3; from: 2; to: 3; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e4; from: 3; to: 4; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e5; from: 3; to: 5; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e6; from: 3; to: 6; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e7; from: 5; to: 6; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e8; from: 6; to: 7; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e9; from: 6; to: 8; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e10; from: 7; to: 8; color: root.edgeColor; width: root.edgeWidth; head: arrow }
			Edge { id: e11; from: 6; to: 6; color: root.edgeColor; width: root.edgeWidth; head: arrow }*/

			vertices: root.session ? root.session.graph.blocks.map(function(a) { return eval(a) }) : []
			edges: []
		}
	}
}

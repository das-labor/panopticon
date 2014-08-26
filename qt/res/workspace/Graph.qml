import QtQuick 2.0
import QtQuick.Controls 1.0
import Panopticon 1.0

Item {
	id: root
	property color edgeColor: "black"
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
			readonly property real hue: 0

			color: Qt.hsla(hue,.7617187500,.8125,1)
			border { width: 1; color: Qt.hsla(hue,1,.2421875,1) }
			radius: 3
			smooth: true

			z: 2

			property int bbid: modelData

			Column {
				id: col
				x: 15; y: 15
				spacing: 3
				property int mnemonicWidth: 0

				Repeater {
					model: eval(session.graph.mnemonics)[bbid]
					delegate: Row {
						spacing: 5

						Text {
							text: modelData.op
							width: col.mnemonicWidth
							font { family: "Monospace" }

							Component.onCompleted: {
								col.mnemonicWidth = Math.max(col.mnemonicWidth,contentWidth)
							}

						}

						Repeater {
							model: modelData.args
							delegate: Text {
								font { family: "Monospace" }
								text: modelData
							}
						}
					}
				}
			}

			height: col.height + 30
			width: col.width + 30

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
					edgeColor = "black"
					sugiyama.route()
				}
				onEntered: {
					for(var i in incoming) {
						incoming[i].color = "blue"
					}
				}
				onExited: {
					for(var i in incoming) {
						incoming[i].color = "black"
					}
				}
			}
		}
	}

	Component {
		id: arrow

		Canvas {
			id: cv
			height: 40; width: 20

			onPaint: {
				var ctx = cv.getContext("2d")

				if(ctx != null) {
					ctx.lineWidth = 0

					ctx.beginPath()
					ctx.fillStyle = "black"
					ctx.moveTo(.5 * cv.width,.5 * cv.height);
					ctx.lineTo(0,cv.height - 1);
					ctx.lineTo(.5 * cv.width,.75 * cv.height);
					ctx.lineTo(cv.width - 1,cv.height - 1);
					ctx.lineTo(.5 * cv.width,.5 * cv.height);
					ctx.fill()
				}
			}
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

			vertices: root.session ? root.session.graph.blocks.map(function(a) { return eval(a) }) : []
			edges: []
		}
	}
}

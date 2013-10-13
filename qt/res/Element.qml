import QtQuick 2.0
import Panopticon 1.0

Item {
	width: childrenRect.width
	height: childrenRect.height
	id: hex

	property string hexData: ""
	property bool selected: false
	property variant globalAnchors: null
	property bool component: true

	Loader {
		id: loader
		sourceComponent: component ? comp1 : comp2
	}

	Component {
		id: comp1

		Item {
			width: globalAnchors.dataColumnWidth + globalAnchors.xMargin * 2
			height: text.height + 2 * globalAnchors.yMargin

			Rectangle {
				anchors.fill: parent
				color: selected ? "red" : "lightblue"
				border.color: "black"
			}

			Text {
				id: text
				text: hex.hexData
				anchors.centerIn: parent

				Component.onCompleted: {
					globalAnchors.dataColumnWidth = Math.max(globalAnchors.dataColumnWidth,width)
				}
			}
		}
	}

	Component {
		id: comp2

		Item {
			Rectangle {
				anchors.fill: parent
				color: parent.selected ? "green" : "gray"
				border.color: "black"
			}

			Text {
				id: text
				anchors.centerIn: parent
				text: hex.hexData

				Component.onCompleted: {
					globalAnchors.dataColumnWidth = Math.max(globalAnchors.dataColumnWidth,width)
					hex.height = Qt.binding(function() { text.height + 2 * globalAnchors.yMargin })
				}
			}
		}
	}
}

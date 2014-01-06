import QtQuick 2.0
import Panopticon 1.0

Item {
	id: item
	width: childrenRect.width
	height: childrenRect.height

	property int elementWidth: 0
	property var payload: ["a"]
	property int address: 0
	property int row: 0

	signal elementClicked(int col, int row)
	signal elementEntered(int col, int row)

	function mousePressed(x,y)
	{
		if(x >= row.x && x < row.x + row.width)
			elementClicked(Math.floor((x - row.x) / elementWidth),item.row)
	}

	function mouseMoved(x,y)
	{
		if(x >= row.x && x < row.x + row.width)
			elementEntered(Math.floor((x - row.x) / elementWidth),item.row)
	}

	Address {
		address: item.address
		context: linearViewContext
		row: parent
	}

	Row {
		id: row
		x: linearViewContext.columnWidth + 5

		Repeater {
			model: item.payload
			delegate: Rectangle {
				width: elementWidth
				height: hex.height + 10
				color: "white"

				Text {
					id: hex
					text: modelData
					anchors.centerIn: parent
				}

				Component.onCompleted: { elementWidth = Math.max(elementWidth,hex.width + 10) }
			}
		}
	}
}

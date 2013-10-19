import QtQuick 2.0
import Panopticon 1.0

Item {
	id: element
	height: childrenRect.height
	width: childrenRect.width

	Address {
		id: address
		address: offset
		globalAnchors: element.parent.globalAnchors
	}


	Repeater {
		id: column
		model: payload

		Item {
			width: childrenRect.width
			height: childrenRect.height
			x: globalAnchors.addressColumnWidth + addressDataMargin + index * ( globalAnchors.dataColumnWidth + 2 * globalAnchors.xMargin )

			Rectangle {
				id: rect
				color: modelData.selected ? "red" : "lightblue"
				border.color: "black"
				width: globalAnchors.dataColumnWidth + 2 * globalAnchors.xMargin
				height: text.height + 2 * globalAnchors.yMargin
			}

			Text {
				id: text
				text: modelData.data
				anchors.centerIn: rect

				Component.onCompleted: { globalAnchors.dataColumnWidth = Math.max(globalAnchors.dataColumnWidth,width) }
			}
		}
	}

	function indexAt(x,y)
	{
		var i = 0
		while(i < column.count)
		{
			var item = column.itemAt(i)
			if(item.x <= x && item.x + item.width >= x)
				return i
			else
				++i
		}
		return null
	}
}

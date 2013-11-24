import QtQuick 2.0
import Panopticon 1.0

Item {
	id: item
	x: linearViewContext.columnWidth + 5

	function attach(first,last)
	{
		item.anchors.top = first.top
		item.height = Qt.binding(function() { return last.y + last.height - first.y })
		item.width = Qt.binding(function() { return Math.max(first.width,last.width) - linearViewContext.columnWidth - 5 })
		item.z = 1
	}

	Rectangle {
		id: rect
		color: Qt.rgba(1,1,0,0.5)
		border.width: 3
		border.color: "#be9700"
		anchors.fill: parent
	}
}

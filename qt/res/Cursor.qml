import QtQuick 2.0
import Panopticon 1.0

Item {
	id: item

	function attach(first,last)
	{
		item.anchors.top = first.top
		item.anchors.left = first.left
		item.height = Qt.binding(function() { return last.y + last.height - first.y })
		item.width = 100
		item.z = 1
	}

	Rectangle {
		id: rect
		color: "yellow"
		anchors.fill: parent
	}
}

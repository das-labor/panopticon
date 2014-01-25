import QtQuick 2.0
import Panopticon 1.0

Item {
	property string address: "none"
	property var context: null
	property var row: null

	id: root
	width: context.columnWidth
	height: row.height

	Text {
		id: text
		text: address
		anchors.verticalCenter: parent.verticalCenter
		x: parent.width - text.width

		onWidthChanged: { context.columnWidth = Math.max(context.columnWidth,text.width) }
	}
}

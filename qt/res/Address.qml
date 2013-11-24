import QtQuick 2.0
import Panopticon 1.0

Item {
	property string address: "none"
	property var context: null
	property var row: null

	width: context.columnWidth
	height: row.height

	Text {
		id: text
		text: address
		anchors.verticalCenter: parent.verticalCenter
	}

	Component.onCompleted: { context.columnWidth = Math.max(context.columnWidth,text.width) }
}

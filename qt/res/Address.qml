import QtQuick 2.0
import Panopticon 1.0

Item {
	property string address: "none"
	property var context: null

	width: childrenRect.width
	height: childrenRect.height

	Text {
		id: text
		text: address
	}

	Component.onCompleted: { context.columnWidth = Math.max(context.columnWidth,text.width) }
}

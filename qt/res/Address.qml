import QtQuick 2.0
import Panopticon 1.0

Item {
	property string address: "none"
	property var globalAnchors: null

	width: globalAnchors.addressColumnWidth + globalAnchors.xMargin
	height: childrenRect.height

	Text {
		id: text
		text: address
		Component.onCompleted: { globalAnchors.addressColumnWidth = Math.max(globalAnchors.addressColumnWidth,text.width) }
	}
}

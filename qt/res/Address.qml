import QtQuick 2.0
import Panopticon 1.0

Item {
	property string address: "none"
	property var globalAnchors: null

	//width: globalAnchors.addressColumnWidth + globalAnchors.xMargin
	width: text.width//globalAnchors.addressColumnWidth + globalAnchors.xMargin
	height: text.height

	Text {
		id: text
		text: address
		Component.onCompleted: {
			var acw = Math.max(globalAnchors.addressColumnWidth,text.width)

			if(acw != globalAnchors.addressColumnWidth)
				globalAnchors.addressColumnWidth = acw
		}
	}
}

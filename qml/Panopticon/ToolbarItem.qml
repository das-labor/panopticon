import QtQuick 2.4
import QtQuick.Controls 1.3 as Ctrl
import QtQuick.Layouts 1.1
import QtGraphicalEffects 1.0

Item {
	property string icon: ""
	property string title: ""

	signal activate()

	id: item
	width: row.width
	height: row.height

	Rectangle {
		anchors.fill: item
		anchors.margins: -5
		color: "red"
		radius: 3
		visible: mouseArea.containsMouse
	}

	Row {
		id: row
		spacing: 5

		Item {
			height: 18
			width: icon.width

			Image {
				id: icon
				source: item.icon
				fillMode: Image.PreserveAspectFit
				height: 18
				mipmap: true
			}
		}

		Label {
			height: 18
			text: item.title
			font {
				pointSize: 11
			}
			verticalAlignment: Text.AlignVCenter
		}
	}

	MouseArea {
		id: mouseArea
		anchors.fill: parent
		hoverEnabled: true

		onClicked: {
			item.activate()
		}
	}
}

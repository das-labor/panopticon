import QtQuick 2.0

Item {
	id: rootItem
	property string primaryTitle: ""
	property string secondaryTitle: ""
	property string primaryAction: "Next"

	Loader {
		id: nextPageLoader
	}

	Rectangle {
		id: titleItem
		x: parent.width - childrenRect.width

		Text {
			id: primaryTitleItem
			font {
				family: "Sans"
				pointSize: 22
			}
			text: primaryTitle
		}

		Text {
			id: secondaryTitleItem
			anchors.left: primaryTitleItem.right
			anchors.leftMargin: 10
			anchors.baseline: primaryTitleItem.baseline

			font {
				family: "Sans"
				pointSize: 14
			}
			color: "#444"
			text: secondaryTitle
		}
	}

	Item {
		id: contentItem
		anchors.top: titleItem.top
		width: rootItem.width
		height: primaryActionItem.top - titleItem.bottom

		Rectangle {
			anchors.fill: parent
			color: "red"
		}
	}

	Item {
		id: primaryActionItem
		height: childrenRect.height

		Rectangle {
			id: iconItem
			x: 8; y: 8
			height: 35
			width: 35
			color: "green"
		}

		Text {
			anchors.left: iconItem.right
			anchors.leftMargin: 10
			anchors.top: iconItem.top

			text: primaryAction
			font { pointSize: 14; family: "Sans" }
		}
	}
}

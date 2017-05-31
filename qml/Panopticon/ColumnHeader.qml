import QtQuick 2.4
import QtQuick.Controls 1.3 as Ctrl
import QtQuick.Layouts 1.1
import QtQuick.Controls.Styles 1.2

Item {
	property int columnWidth: 0
	property string columnTitle: ""
	property int columnOrdinal: 0

	Layout.alignment: Qt.AlignHCenter
	Layout.preferredWidth: columnWidth
	Layout.preferredHeight: childrenRect.height

	Item {
		anchors.centerIn: parent
		width: childrenRect.width
		height: childrenRect.height

		Label {
			id: title
			text: columnTitle
			color: "#8e8e8e"
			font.pointSize: 9
		}

		Image {
			height: 10
			anchors.left: title.right
			anchors.leftMargin: 10
			anchors.verticalCenter: title.verticalCenter
			antialiasing: true
			fillMode: Image.PreserveAspectFit
			source: "../icons/chevron-down.svg"
			mipmap: true
			visible: Panopticon.fileBrowserSortColumn === columnOrdinal || mouseArea.containsMouse
			rotation: (Panopticon.fileBrowserSortAscending ? 180 : 0)
		}
	}

	MouseArea {
		id: mouseArea
		anchors.fill: parent
		hoverEnabled: true
		onClicked: {
			if(columnOrdinal !== Panopticon.fileBrowserSortColumn) {
				Panopticon.sortBy(columnOrdinal,Panopticon.fileBrowserSortAscending)
			} else {
				Panopticon.sortBy(columnOrdinal,!Panopticon.fileBrowserSortAscending)
			}
		}
	}
}

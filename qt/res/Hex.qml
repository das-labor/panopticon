import QtQuick 2.0
import Panopticon 1.0

Item {
	id: root
	height: 100
	width: 100

	property int addressColumnWidth: 1
	property int dataColumnWidth: 1
	property int yMargin: 3
	property int xMargin: 3
	property int addressDataMargin: 10
	property int anchorRow: -1
	property int anchorCol: -1
	property int selectRow: -1
	property int selectCol: -1

	signal select(int firstRow, int firstCol, int lastRow, int lastCol)

	function modifySelection(row, col, extend)
	{
		if(extend)
		{
			selectRow = row
			selectCol = col
		}
		else
		{
			selectRow = anchorRow = row
			selectCol = anchorCol = col
		}

		root.select(anchorRow,anchorCol,selectRow,selectCol)
	}

	ListView {
		id: listView
		anchors.fill: parent
		model: linearModel
		delegate: Component {
			Item {
				height: childrenRect.height
				width: childrenRect.width

				function indexAt(x,y) { return loader.item.indexAt(x,y) }

				Loader {
					id: loader
					property var rowData: row
					property var globalAnchors: root
					source: delegate
				}
			}
		}
	}

	MouseArea {
		anchors.fill: root

		onPressed: {
			var item = listView.itemAt(mouse.x + listView.contentX,mouse.y + listView.contentY)
			root.modifySelection(listView.indexAt(mouse.x + listView.contentX,mouse.y + listView.contentY),item.indexAt(mouse.x,mouse.y),false)
		}

		onPositionChanged: {
			var item = listView.itemAt(mouse.x + listView.contentX,mouse.y + listView.contentY)
			root.modifySelection(listView.indexAt(mouse.x + listView.contentX,mouse.y + listView.contentY),item.indexAt(mouse.x,mouse.y),true)
		}
	}
}

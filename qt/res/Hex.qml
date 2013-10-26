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
	signal toggleVisibility(int blkid,bool vis)

	function modifySelection(row, col, extend)
	{
		if(extend)
		{
			if(selectRow != row || selectCol != col)
			{
				selectRow = row
				selectCol = col
				root.select(anchorRow,anchorCol,selectRow,selectCol)
			}
		}
		else
		{
			selectRow = anchorRow = row
			selectCol = anchorCol = col
			root.select(anchorRow,anchorCol,selectRow,selectCol)
		}
	}

	ListView {
		id: listView
		anchors.fill: parent
		model: linearModel
		delegate: Component {
			Item {
				height: childrenRect.height
				width: childrenRect.width

				function indexAt(x,y) {
					if(loader.item.indexAt != undefined)
						return loader.item.indexAt(x,y)
					else
						return -1
				}

				Loader {
					id: loader
					property var payload: model.payload
					property var globalAnchors: root
					property var address: model.offset
					source: model.delegate
				}
			}
		}
	}

	MouseArea {
		anchors.fill: root
		acceptedButtons: Qt.LeftButton | Qt.RightButton

		onPressed: {
			var item = listView.itemAt(mouse.x + listView.contentX,mouse.y + listView.contentY)
			if(item != null)
			{
				var index = item.indexAt(mouse.x,mouse.y)
				if(index >= 0)
				{
					root.modifySelection(listView.indexAt(mouse.x + listView.contentX,mouse.y + listView.contentY),index,false)
					return
				}
			}
			mouse.accepted = false
		}

		onPositionChanged: {
			var item = listView.itemAt(mouse.x + listView.contentX,mouse.y + listView.contentY)
			if(item != null)
			{
				var index = item.indexAt(mouse.x,mouse.y)
				if(index >= 0)
				{
					root.modifySelection(listView.indexAt(mouse.x + listView.contentX,mouse.y + listView.contentY),index,true)
					return
				}
			}
			mouse.accepted = false
		}
	}
}

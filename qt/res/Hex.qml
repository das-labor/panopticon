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

	Component {
		id: comp

		Item {
			height: childrenRect.height
			width: childrenRect.width

			function indexAt(x,y)
			{
				var i = 0
				while(i < column.count)
				{
					var item = column.itemAt(i)
					if(item.x <= x && item.x + item.width >= x)
						return i
					else
						++i
				}
				return null
			}

			Loader {
				source: modelData.source
				onLoaded: {
					item.x = Qt.binding(function() {root.addressColumnWidth + root.addressDataMargin + (2 * root.xMargin + root.dataColumnWidth) * index})
					item.hexData = Qt.binding(function() { modelData.data })
					item.selected = Qt.binding(function() { modelData.selected })
					item.globalAnchors = Qt.binding(function() { root })
				}
			}
/*
			Repeater {
				id: column
				model: meta
				delegate: Element {
					id: elem
					x: root.addressColumnWidth + root.addressDataMargin + (2 * root.xMargin + root.dataColumnWidth) * index
					hexData: modelData.data
					selected: modelData.selected
					globalAnchors: root
				}
			}*/

			Text {
				id: address
				anchors.left: parent.left
				anchors.leftMargin: root.xMargin
				anchors.top: parent.top
				anchors.topMargin: root.yMargin
				text: index

				Component.onCompleted: { root.addressColumnWidth = Math.max(root.addressColumnWidth,address.width) }
			}
		}
	}

	ListView {
		id: listView
		anchors.fill: parent
		model: linearModel
		delegate: comp
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

import QtQuick 2.1

Item {
	id: root

	property var session: null
	property var rowsByIndex: []

	anchors.fill: parent

	Component {
		id: row

		Row {
			id: rowContents

			function columnAt(x) {
				var i = 0
				while(i < 5) {
					var itm = rep.itemAt(i)

					if(itm.x <= x && itm.x + itm.width >= x)
						return i
					++i
				}

				return -1
			}

			Repeater {
				id: rep
				model: 5
				delegate: Rectangle {
					height: 33
					width: 33

					Text { anchors.centerIn: parent; text: modelData }
				}
			}

			Component.onCompleted: { root.rowsByIndex[index] = rowContents }
			Component.onDestruction: { delete root.rowsByIndex[index] }
		}
	}

	Component {
		id: selection

		Item {
			id: item
			property rect firstRect: Qt.rect(0,0,0,0)
			property rect lastRect: Qt.rect(0,0,0,0)
			property color color: "#11223355"
			property int rightEdge: 33 * 5
			property int borderWidth: 1

			Rectangle {
				id: single

				x: item.firstRect.x
				y: item.firstRect.y
				width: item.lastRect.x - item.firstRect.x + item.lastRect.width
				height: item.firstRect.height
				visible: item.firstRect.y == item.lastRect.y

				color: item.color

				RightBorder { borderWidth: item.borderWidth }
				LeftBorder { borderWidth: item.borderWidth }
				TopBorder { borderWidth: item.borderWidth }
				BottomBorder { borderWidth: item.borderWidth }
			}

			Rectangle {
				id: upper

				x: item.firstRect.x
				y: item.firstRect.y
				width: item.rightEdge - x
				height: item.firstRect.height
				visible: item.firstRect.y != item.lastRect.y

				color: item.color

				LeftBorder { borderWidth: item.borderWidth }
				RightBorder { borderWidth: item.borderWidth }
				TopBorder { borderWidth: item.borderWidth }
			}

			Rectangle {
				id: lower

				x: 0
				y: item.lastRect.y
				width: item.lastRect.x + item.lastRect.width
				height: item.lastRect.height
				visible: item.firstRect.y != item.lastRect.y

				color: item.color

				LeftBorder { borderWidth: item.borderWidth }
				RightBorder { borderWidth: item.borderWidth }
				BottomBorder { borderWidth: item.borderWidth }
			}

			Rectangle {
				id: mid

				x: 0
				y: item.firstRect.y + item.firstRect.height
				width: item.rightEdge
				height: item.lastRect.y - item.firstRect.y - item.firstRect.height
				visible: item.firstRect.y != item.lastRect.y

				color: item.color

				LeftBorder { borderWidth: item.borderWidth }
				RightBorder { borderWidth: item.borderWidth }
				TopBorder { borderWidth: item.borderWidth; width: upper.x }
				BottomBorder { borderWidth: item.borderWidth; x: lower.x + lower.width; color: "green" }
			}
		}
	}

	ListView {
		Item {
			id: overlays
			anchors.fill: parent

			function update(firstItm, firstIdx, lastItm, lastIdx) {
				for(var i in overlays.children) {
					var itm = overlays.children[i]
					if(firstIdx > itm.lower || lastIdx < itm.upper) {
						if(itm.component != null) {
							itm.component.visible = false
							itm.component = null
						}
					} else {
						if(itm.component == null) {
							itm.component = selection.createObject(itm)
							itm.component.width = lst.width
							itm.component.height = lst.height

						}

						if(firstIdx <= itm.upper) {
							var anc = root.rowsByIndex[itm.upper]
							itm.component.y = anc.y - lst.contentY
							itm.component.firstRect = Qt.rect(33 * itm.first,0,33,33)
						} else {
							itm.component.y = 0
							itm.component.firstRect = Qt.rect(0,0,33,33)
						}

						if(lastIdx >= itm.lower) {
							var anc = root.rowsByIndex[itm.lower]
							itm.component.height = anc.y - itm.component.y + anc.height - lst.contentY
							itm.component.lastRect = Qt.rect(33 * itm.last,itm.component.height - 33,33,33)
						} else {
							itm.component.height = lst.height
							itm.component.lastRect = Qt.rect(4 * 33,itm.component.height - 33,33,33)
						}

						itm.component.visible = true
					}
				}
			}
		}

		id: lst
		anchors.fill: parent
		model: root.session.linear
		delegate: row

		function scheduleUpdate() {
			var firstItm = itemAt(contentX,contentY)
			var firstIdx = indexAt(contentX,contentY)
			var lastItm = itemAt(contentX,contentY + height - 1)
			var lastIdx = indexAt(contentX,contentY + height - 1)

			if(firstItm != null && lastItm != null)
				overlays.update(firstItm,firstIdx,lastItm,lastIdx)
		}

		onContentYChanged: scheduleUpdate()
		Component.onCompleted: scheduleUpdate()
	}

	MouseArea {
		property var cursor: null
		property int anchorRow: -1
		property int anchorCol: -1

		anchors.fill: lst

		onPressed: {
			if(state == "") {
				var row = lst.indexAt(mouse.x + lst.contentX,mouse.y + lst.contentY)
				var rowItm = lst.itemAt(mouse.x + lst.contentX,mouse.y + lst.contentY)

				if(row >= 0 && rowItm.columnAt != undefined) {
					var col = rowItm.columnAt(mouse.x + lst.contentX)
					anchorRow = row
					anchorCol = col
					state = "selecting"
					updateCursor()
				}
			} else if(state == "selected") {
				state = ""
				if(cursor != null)
					cursor.visible = false
			}
			mouse.accepted = true
		}

		onReleased: {
			if(state == "selecting") {
				mouse.accepted = true
				anchorRow = -1
				anchorCol = -1
				state = "selected"
			}
		}

		onPositionChanged: {
			if(state == "selecting") {
				updateCursor()
				mouse.accepted = true
			}
		}

		function updateCursor() {
			if(state == "selecting") {
				var row = lst.indexAt(mouseX + lst.contentX,mouseY + lst.contentY)
				var rowItm = lst.itemAt(mouseX + lst.contentX,mouseY + lst.contentY)

				if(row >= 0 && rowItm.columnAt != undefined) {
					var col = rowItm.columnAt(mouseX + lst.contentX)
					var u, d, f, l

					if(row < anchorRow) {
						u = row
						f = col
						d = anchorRow
						l = anchorCol
					} else if(row > anchorRow) {
						d = row
						l = col
						u = anchorRow
						f = anchorCol
					} else if(row == anchorRow) {
						u = row
						d = row
						f = Math.min(col,anchorCol)
						l = Math.max(col,anchorCol)
					}

					if(cursor == null) {
						cursor = Qt.createQmlObject("import QtQuick 2.1; Item { property int upper: "+u+"; property int lower: "+d+"; property int first: "+f+"; property int last: "+l+"; property var component: null }",overlays);
					} else {
						cursor.upper = u
						cursor.lower = d
						cursor.first = f
						cursor.last = l
						cursor.visible = true
					}

					lst.scheduleUpdate()
				}
			}
		}
	}
}

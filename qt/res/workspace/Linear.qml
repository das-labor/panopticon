import QtQuick 2.1
import QtQuick.Controls 1.2

Item {
	id: root

	property var session: null
	property int cursorUpperCol: -1
	property int cursorLowerCol: -1
	property int cursorUpperRow: -1
	property int cursorLowerRow: -1
	property int addressColumnWidth: 0

	readonly property string fontFamily: "Monospace"
	readonly property real fontPointSize: 7
	readonly property real cellSize: 23
	readonly property real halfCellSize: 12
	readonly property int maxMnemonicHexdump: 4

	focus: true

	Component {
		id: row

		Row {
			id: rowContents
			spacing: 25

			property var contents: eval(model.display)
			property int rowIndex: index

			function columnAt(x) {
				var i = 0
				var cols = [hexCol, textCol]
				var rep = 0

				x -= cellSize + 25 + addressColumnWidth + 25

				while(i < cols.length)
				{
					var c = cols[i++]
					if(c.x <= x && c.x + c.width > x)
					{
						rep = c
						break;
					}
				}

				i = 0
				while(rep != 0 && i < rep.children.length) {
					var itm = rep.children[i]

					if(itm.x + rep.x <= x && itm.x + rep.x + itm.width > x && contents.payload.hex[i] != '')
						return i
					++i
				}

				return -1
			}

			Canvas {
				readonly property int lineWidth: 1
				readonly property int trackPadding: 5
				readonly property int radius: 3

				height: cellSize
				width: cellSize
				contextType: "2d"

				onPaint: {
					var ctx = getContext("2d")
					var ends = contents.arrows.end
					var begins = contents.arrows.begin
					var ud = contents.arrows.pass

					ctx.imageSmoothingEnabled = true

					var y_step = Math.ceil(cellSize / (ends.concat(begins).length + 1))
					var idx = 0
					while(idx < begins.length)
					{
						var line = begins[idx].track
						var y_pos = y_step * (idx + 1)
						var x_pos = (line + 1) * trackPadding

						ctx.strokeStyle = "rgb(200,0,0)"
						ctx.lineWidth = lineWidth

						ctx.beginPath()
						ctx.moveTo(cellSize, y_pos);
						ctx.lineTo(x_pos + radius, y_pos)
						ctx.arc(x_pos + radius, y_pos + radius, radius, Math.PI * 1.5, Math.PI, true);
						ctx.lineTo(x_pos, cellSize);
						ctx.stroke()

						if(begins[idx].tip) {
							ctx.beginPath()
							ctx.arc(cellSize - 4,y_pos,4,0,Math.PI * 2, true)
							ctx.fill()
						}

						idx += 1
					}

					idx = 0
					while(idx < ends.length)
					{
						var line = ends[idx].track

						var y_pos = y_step * (idx + 1)
						var x_pos = (line + 1) * trackPadding

						ctx.strokeStyle = "rgb(200,0,0)"
						ctx.lineWidth = lineWidth

						ctx.beginPath()
						ctx.moveTo(cellSize, y_pos);
						ctx.lineTo(x_pos + radius, y_pos)
						ctx.arc(x_pos + radius, y_pos - radius, radius, Math.PI * 0.5, 1 * Math.PI, false);
						ctx.moveTo(x_pos, y_pos - radius);
						ctx.lineTo(x_pos, 0);
						ctx.stroke()

						if(ends[idx].tip) {
							ctx.beginPath()
							ctx.arc(cellSize - 4,y_pos,4,0,Math.PI * 2, true)
							ctx.fill()
						}
						idx += 1
					}

					idx = 0
					while(idx < ud.length)
					{
						var line = ud[idx]
						var x_pos = (line + 1) * trackPadding

						ctx.strokeStyle = "rgb(200,0,0)"
						ctx.lineWidth = lineWidth

						ctx.beginPath()
						ctx.moveTo(x_pos, 0);
						ctx.lineTo(x_pos , cellSize)
						ctx.stroke()

						idx += 1
					}

				}
			}

			Item {
				id: addrCol
				height: root.cellSize
				width: root.addressColumnWidth

				Text {
					height: parent.height
					text: contents.address
					verticalAlignment: Text.AlignVCenter
					font { family: root.fontFamily; pointSize: root.fontPointSize }

					onWidthChanged: { root.addressColumnWidth = Math.max(root.addressColumnWidth,width) }
				}
			}

			Item {
				height: cellSize
				width: childrenRect.width

				Row {
					id: hexdump
					visible: contents.payload.type == 'raw'
					spacing: 25

					property int activeColumn: -1

					Row {
						id: hexCol

						Repeater {
							model: contents.payload.hex
							delegate: Rectangle {
								property int colIndex: index
								height: root.cellSize
								width: root.cellSize
								color: {
									var c = ((cursorUpperRow < rowIndex && cursorLowerRow > rowIndex) ||
													 (cursorUpperRow == rowIndex && cursorLowerRow != rowIndex && cursorUpperCol <= colIndex) ||
													 (cursorLowerRow == rowIndex && cursorUpperRow != rowIndex && cursorLowerCol >= colIndex) ||
													 (cursorLowerRow == rowIndex && cursorUpperRow == rowIndex && cursorUpperCol <= colIndex && cursorLowerCol >= colIndex))
									return c ? "red" : "white"
								}

								Text {
									anchors.centerIn: parent
									text: modelData
									color: hexdump.activeColumn == colIndex ? "red" : "black"
									font { family: root.fontFamily; pointSize: root.fontPointSize }
								}

								MouseArea {
									id: mouseArea
									anchors.fill: parent
									hoverEnabled: true
									acceptedButtons: Qt.RightButton

									onEntered: hexdump.activeColumn = colIndex
									onExited: hexdump.activeColumn = -1
									onPressed: session.disassemble(rowIndex,colIndex)
								}
							}
						}
					}

					Row {
						id: textCol

						Repeater {
							model: contents.payload.text
							delegate: Rectangle {
								property int colIndex: index
								height: root.cellSize
								width: root.halfCellSize
								color: {
									var c = ((cursorUpperRow < rowIndex && cursorLowerRow > rowIndex) ||
													 (cursorUpperRow == rowIndex && cursorLowerRow != rowIndex && cursorUpperCol <= colIndex) ||
													 (cursorLowerRow == rowIndex && cursorUpperRow != rowIndex && cursorLowerCol >= colIndex) ||
													 (cursorLowerRow == rowIndex && cursorUpperRow == rowIndex && cursorUpperCol <= colIndex && cursorLowerCol >= colIndex))
									return c ? "red" : "white"
								}

								Text {
									anchors.centerIn: parent
									text: modelData
									color: hexdump.activeColumn == colIndex ? "red" : "black"
									font { family: root.fontFamily; pointSize: root.fontPointSize }
								}

								MouseArea {
									id: mouseArea
									anchors.fill: parent
									hoverEnabled: true
									acceptedButtons: Qt.RightButton

									onEntered: hexdump.activeColumn = colIndex
									onExited: hexdump.activeColumn = -1
									onPressed: session.disassemble(rowIndex,colIndex)
								}
							}
						}
					}
				}

				Row {
					id: struct
					visible: contents.payload.type == 'mne'
					height: childrenRect.height
					width: childrenRect.width

					Repeater {
						model: contents.payload.hex.slice(0,maxMnemonicHexdump)
						delegate: Rectangle {
							property int colIndex: index
							height: root.cellSize
							width: root.cellSize
							color: {
								var c = ((cursorUpperRow < rowIndex && cursorLowerRow > rowIndex) ||
												 (cursorUpperRow == rowIndex && cursorLowerRow != rowIndex && cursorUpperCol <= colIndex) ||
												 (cursorLowerRow == rowIndex && cursorUpperRow != rowIndex && cursorLowerCol >= colIndex) ||
												 (cursorLowerRow == rowIndex && cursorUpperRow == rowIndex && cursorUpperCol <= colIndex && cursorLowerCol >= colIndex))
								return c ? "red" : "white"
							}

							Text {
								anchors.centerIn: parent
								text: modelData
								color: hexdump.activeColumn == colIndex ? "red" : "black"
								font { family: root.fontFamily; pointSize: root.fontPointSize }
							}

							MouseArea {
								id: mouseArea
								anchors.fill: parent
								hoverEnabled: true
								acceptedButtons: Qt.RightButton

								onEntered: hexdump.activeColumn = colIndex
								onExited: hexdump.activeColumn = -1
								onPressed: session.disassemble(rowIndex,colIndex)
							}
						}
					}

					Repeater {
						model: maxMnemonicHexdump - contents.payload.hex.length
						delegate: Rectangle {
							height: cellSize
							width: cellSize
						}
					}

					Text {
						width: cellSize + halfCellSize
						height: cellSize
						text: (contents.payload.hex.length > maxMnemonicHexdump ? "…" : "")
					}

					Text {
						width: paintedWidth
						height: cellSize
						verticalAlignment: Text.AlignBottom
						font { family: root.fontFamily; pointSize: root.fontPointSize }
						text: (contents.payload.type == 'mne' ? contents.payload.op + "  " : "") + (contents.payload.type == 'mne' ? contents.payload.args.join(", ") : "")
					}
				}
			}

			Item {
				width: childrenRect.width; height: childrenRect.height
				x:childrenRect.x; y: childrenRect.y

				Rectangle {
					anchors.fill: commentCol
					color: commentCol.activeFocus ? "red" : "#00000000"
				}

				TextEdit {
					id: commentCol

					readOnly: false
					text: contents.comment
					width: 300
					height: root.cellSize * Math.max(1,lineCount)

					Keys.enabled: true
					Keys.priority: Keys.BeforeItem
					Keys.onPressed: {
						if((event.key == Qt.Key_Enter && (event.modifiers & Qt.ShiftModifier) == 0) ||
							 (event.key == Qt.Key_Return && (event.modifiers & Qt.ShiftModifier) == 0)) {
							root.session.postComment(rowIndex,text)
							text = contents.comment
							focus = false
							event.accepted = true
						}
						else if(event.key == Qt.Key_Escape) {
							text = contents.comment
							focus = false
							event.accepted = true
						}
					}
				}
			}
		}
	}

	ScrollView {
		anchors.fill: parent
		id: scroll

			ListView {
			id: lst
			model: root.session.linear
			delegate: row
			cacheBuffer: 1
			interactive: false
		}
	}

	MouseArea {
		property var cursor: null
		property int anchorRow: -1
		property int anchorCol: -1

		anchors.fill: scroll

		onPressed: {
			if(state == "") {
				var row = lst.indexAt(mouse.x + lst.contentX,mouse.y + lst.contentY)
				var rowItm = lst.itemAt(mouse.x + lst.contentX,mouse.y + lst.contentY)

				if(row >= 0 && rowItm.columnAt != undefined) {
					var col = rowItm.columnAt(mouse.x + lst.contentX)
					if(col >= 0) {
						anchorRow = row
						anchorCol = col
						state = "selecting"
						updateCursor()
						mouse.accepted = true
					} else {
						mouse.accepted = false
					}
				}
			} else if(state == "selected") {
				state = ""
				cursorUpperRow = -1
				cursorLowerRow = -1
				cursorUpperCol = -1
				cursorLowerCol = -1
				mouse.accepted = true
			}
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
					var u = -1, d = -1, f = -1, l = -1

					if(col >= 0) {
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

						root.cursorUpperRow = u
						root.cursorLowerRow = d
						root.cursorUpperCol = f
						root.cursorLowerCol = l
					}
				}
			}
		}
	}
}

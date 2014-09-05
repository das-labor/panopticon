import QtQuick 2.0
import QtQuick.Controls 1.0

Item {
	id: root

	property var session: null
	property int cursorUpperCol: -1
	property int cursorLowerCol: -1
	property int cursorUpperRow: -1
	property int cursorLowerRow: -1
	property int addressColumnWidth: 0
	property int payloadColumnWidth: 0
	property int arrowColumnWidth: 100

	readonly property string fontFamily: "Monospace"
	readonly property real fontPixelSize: 14
	readonly property real cellSize: 23
	readonly property real halfCellSize: 12
	readonly property int maxMnemonicHexdump: 4
	readonly property bool mnemonicHexdump: false

	property color selectionColor: "red"
	property color arrowBodyColor: "blue"
	property color arrowHeadColor: "blue"
	property color hoverColor: "blue"

	Component {
		id: row

		Row {
			id: rowContents
			spacing: 25
			height: childrenRect.height
			width: childrenRect.width

			property var contents: eval(model.display)
			property int rowIndex: index

			function columnAt(x) {
				var i = 0
				var cols = [hexCol, textCol]
				var rep = 0

				x -= arrowCanvas.width + 25 + addressColumnWidth + 25

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
				readonly property int trackPadding: 10
				readonly property int radius: 4

				id: arrowCanvas
				height: cellSize
				width: root.arrowColumnWidth

				onPaint: {
					var ctx = arrowCanvas.getContext("2d")

					ctx.fillStyle = "#dddddd"
					ctx.fillRect(arrowCanvas.x,arrowCanvas.y,arrowCanvas.width,arrowCanvas.height)

					if(ctx != null) {
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
							var x_pos = arrowCanvas.width - (line + 2) * trackPadding

							ctx.strokeStyle = root.arrowBodyColor
							ctx.lineWidth = lineWidth

							ctx.beginPath()
							ctx.moveTo(arrowCanvas.width, y_pos);
							ctx.lineTo(x_pos + radius, y_pos)
							ctx.arc(x_pos + radius, y_pos + radius, radius, Math.PI * 1.5, Math.PI, true);
							ctx.lineTo(x_pos, cellSize);
							ctx.stroke()

							if(begins[idx].tip) {
								ctx.fillStyle = root.arrowHeadColor
								ctx.beginPath()
								ctx.arc(arrowCanvas.width - 4,y_pos,4,0,Math.PI * 2, true)
								ctx.fill()
							}

							idx += 1
						}

						idx = 0
						while(idx < ends.length)
						{
							var line = ends[idx].track

							var y_pos = y_step * (idx + 1)
							var x_pos = arrowCanvas.width - (line + 2) * trackPadding

							ctx.strokeStyle = root.arrowBodyColor
							ctx.lineWidth = lineWidth

							ctx.beginPath()
							ctx.moveTo(arrowCanvas.width, y_pos);
							ctx.lineTo(x_pos + radius, y_pos)
							ctx.arc(x_pos + radius, y_pos - radius, radius, Math.PI * 0.5, 1 * Math.PI, false);
							ctx.moveTo(x_pos, y_pos - radius);
							ctx.lineTo(x_pos, 0);
							ctx.stroke()

							if(ends[idx].tip) {
								ctx.fillStyle = root.arrowHeadColor
								ctx.beginPath()
								ctx.arc(arrowCanvas.width - 4,y_pos,4,0,Math.PI * 2, true)
								ctx.fill()
							}
							idx += 1
						}

						idx = 0
						while(idx < ud.length)
						{
							var line = ud[idx]
							var x_pos = arrowCanvas.width - (line + 2) * trackPadding

							ctx.strokeStyle = root.arrowBodyColor
							ctx.lineWidth = lineWidth

							ctx.beginPath()
							ctx.moveTo(x_pos, 0);
							ctx.lineTo(x_pos , cellSize)
							ctx.stroke()

							idx += 1
						}
					}
				}
			}

			Item {
				id: addrCol
				height: root.cellSize
				width: root.addressColumnWidth

				Rectangle {
					id: background
					x: -25
					height: parent.height
					width: -1 * x + parent.width + 10
					color: "#dddddd"
				}

				Rectangle {
					anchors.left: background.right
					width: 1
					height: parent.height
					color: "#aeaeae"
				}

				Text {
					height: parent.height
					text: contents.address
					verticalAlignment: Text.AlignVCenter
					font { family: root.fontFamily; pixelSize: root.fontPixelSize }

					onWidthChanged: { root.addressColumnWidth = Math.max(root.addressColumnWidth,width) }
				}
			}

			Item {
				id: payloadCol
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
									return c && modelData != "" ? root.selectionColor : "#00000000"
								}

								Text {
									anchors.centerIn: parent
									text: modelData
									color: hexdump.activeColumn == colIndex ? root.hoverColor : "black"
									font { family: root.fontFamily; pixelSize: root.fontPixelSize }
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
									return c ? root.selectionColor : "#00000000"
								}

								Text {
									anchors.centerIn: parent
									text: modelData
									color: hexdump.activeColumn == colIndex ? root.hoverColor : "black"
									font { family: root.fontFamily; pixelSize: root.fontPixelSize }
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
						model: root.mnemonicHexdump ? contents.payload.hex.slice(0,maxMnemonicHexdump) : []
						delegate: Rectangle {
							property int colIndex: index
							height: root.cellSize
							width: root.cellSize
							color: {
								var c = ((cursorUpperRow < rowIndex && cursorLowerRow > rowIndex) ||
												 (cursorUpperRow == rowIndex && cursorLowerRow != rowIndex && cursorUpperCol <= colIndex) ||
												 (cursorLowerRow == rowIndex && cursorUpperRow != rowIndex && cursorLowerCol >= colIndex) ||
												 (cursorLowerRow == rowIndex && cursorUpperRow == rowIndex && cursorUpperCol <= colIndex && cursorLowerCol >= colIndex))
								return c ? root.selectionColor : "#00000000"
							}

							Text {
								anchors.centerIn: parent
								text: modelData
								color: hexdump.activeColumn == colIndex ? root.hoverColor : "black"
								font { family: root.fontFamily; pixelSize: root.fontPixelSize }
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
						model: root.mnemonicHexdump ? (root.maxMnemonicHexdump - contents.payload.hex.length) : 0
						delegate: Rectangle {
							height: cellSize
							width: cellSize
							color: "#00000000"
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
						verticalAlignment: Text.AlignVCenter
						font { family: root.fontFamily; pixelSize: root.fontPixelSize }
						color: "#1e1e1e"
						text: (contents.payload.type == 'mne' ? contents.payload.op + "  " : "") + (contents.payload.type == 'mne' ? contents.payload.args.join(", ") : "")
					}
				}

				onWidthChanged: { root.payloadColumnWidth = Math.max(root.payloadColumnWidth,width) }
			}

			Item {
				width: childrenRect.width + Math.max(0,root.payloadColumnWidth - payloadCol.width)
				height: childrenRect.height

				Rectangle {
					anchors.leftMargin: -25
					anchors.fill: commentCol
					color: commentCol.activeFocus ? root.selectionColor : "#dddddd"

					Rectangle {
						height: parent.height
						width: 1
						color: "#aeaeae"
					}
				}

				TextEdit {
					id: commentCol

					readOnly: false
					text: contents.comment
					width: root.width - root.payloadColumnWidth - root.addressColumnWidth - root.arrowColumnWidth
					height: root.cellSize * Math.max(1,lineCount)
					x: Math.max(0,root.payloadColumnWidth - payloadCol.width) + 25

					Keys.enabled: true
					Keys.priority: Keys.BeforeItem
					Keys.onPressed: {
						if((event.key == Qt.Key_Enter && (event.modifiers & Qt.ShiftModifier) == 0) ||
							 (event.key == Qt.Key_Return && (event.modifiers & Qt.ShiftModifier) == 0)) {
							root.session.postComment(rowIndex,text)
							text = contents.comment
							focus = false
							event.accepted = true
						} else if(event.key == Qt.Key_Escape) {
							text = contents.comment
							focus = false
							event.accepted = true
						}
					}
				}
			}
		}
	}

	ListView {
		id: lst
		model: root.session.linear
		delegate: row
		cacheBuffer: 1
		maximumFlickVelocity: 1500
		boundsBehavior: Flickable.StopAtBounds
		anchors.fill: parent
	}

	MouseArea {
		property var cursor: null
		property int anchorRow: -1
		property int anchorCol: -1

		anchors.fill: lst
		preventStealing: true

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

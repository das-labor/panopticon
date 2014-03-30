import QtQuick 2.0

Item {
	// geometry
	property var firstRow: null
	property var lastRow: null
	property var cursor: null

	// visual
	property int outlineWidth: 2
	property var color: "#be9700"
	property var background: "rgba(255,255,0,0.5)"

	id: item
	x: linearViewContext.columnWidth + 5
	y: { firstRow && lastRow ? firstRow.y : 0 }
	z: 1
	width: { firstRow && lastRow ? Math.max(firstRow.width,lastRow.width) - linearViewContext.columnWidth - 5 : 0 }
	height: { firstRow && lastRow ? lastRow.y + lastRow.height - firstRow.y : 0 }
	onCursorChanged: { poly.requestPaint() }

	Canvas {
		id: poly
		anchors.fill: parent
		contextType: "2d"
		onAvailableChanged: { item.cursor = item.cursor }

		function continousSelection(start,end)
		{
			var ret = []
			var line_start = outlineWidth / 2
			var line_end = item.width - outlineWidth
			var single_line = start.y == end.y

			var p1 = Qt.point(start.x,start.y)
			var p2 = Qt.point(line_end,start.y)
			var p3 = Qt.point(line_end,end.y)
			var p4 = Qt.point(end.x + end.width,end.y)
			var p5 = Qt.point(end.x + end.width,end.y + end.height)
			var p6 = Qt.point(line_start,end.y + end.height)
			var p7 = Qt.point(line_start,start.y + start.height)
			var p8 = Qt.point(start.x,start.y + start.height)

			ret.push(p1);

			if(!single_line)
				ret = ret.concat(p2,p3)

			ret = ret.concat(p4,p5)

			if(!single_line)
				ret = ret.concat(p6,p7)

			ret = ret.concat(p8)

			return [ret]
		}

		function selectionPolysDisjointed(start,end)
		{
			var ret = []
			var line_start = outlineWidth / 2
			var line_end = item.width - outlineWidth

			return [[Qt.point(start.x,start.y),
							 Qt.point(line_end,start.y),
							 Qt.point(line_end,start.y + start.height),
							 Qt.point(start.x,start.y + start.height)],
							[Qt.point(line_start,end.y),
							 Qt.point(end.x + end.width,end.y),
							 Qt.point(end.x + end.width,end.y + end.height),
							 Qt.point(line_start,end.y + end.height)]]
		}

		onPaint: {
			context.clearRect(0,0,poly.width,poly.height)

			if(cursor && firstRow && lastRow)
			{
				var f,l;

				if(cursor.firstLine != cursor.lastLine)
				{
					f = (cursor.firstLine == cursor.anchorLine ? cursor.anchorColumn : cursor.cursorColumn) * firstRow.elementWidth
					l = (cursor.lastLine == cursor.anchorLine ? cursor.anchorColumn : cursor.cursorColumn) * lastRow.elementWidth
				}
				else
				{
					f = cursor.firstColumn * firstRow.elementWidth
					l = cursor.lastColumn * firstRow.elementWidth
				}

				var firstRect = Qt.rect(f + outlineWidth / 2,outlineWidth / 2,firstRow.elementWidth,firstRow.height - outlineWidth)
				var lastRect = Qt.rect(l,height - lastRow.height + outlineWidth / 2,lastRow.elementWidth - outlineWidth,lastRow.height - outlineWidth)
				var paths

				if(firstRow.row > cursor.firstLine)
					firstRect = Qt.rect(firstRect.x,firstRect.y - firstRect.height,firstRow.elementWidth,firstRow.height - outlineWidth)
				if(lastRow.row < cursor.lastLine)
					lastRect = Qt.rect(lastRect.x,lastRect.y + lastRect.height,lastRow.elementWidth,lastRow.height - outlineWidth)

				if((firstRow.y + firstRow.height) == lastRow.y && (lastRect.x + lastRect.width) <= firstRect.x)
					paths = selectionPolysDisjointed(firstRect,lastRect)
				else
					paths = continousSelection(firstRect,lastRect)

				context.fillStyle = item.background
				context.strokeStyle = item.color
				context.lineWidth = item.outlineWidth

				for(var j in paths)
				{
					var p = paths[j]
					context.beginPath()
					context.moveTo(p[0].x,p[0].y)

					for(var i in p)
					{
						context.lineTo(p[i].x,p[i].y)
						i += 1
					}
					context.closePath()
					context.stroke()
					context.fill()
				}
			}
		}
	}
}

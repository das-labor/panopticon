/*
 * Panopticon - A libre disassembler (https://panopticon.re/)
 * Copyright (C) 2017 Panopticon authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

import QtQuick 2.4
import QtQuick.Controls 1.3 as Ctrl
import QtQuick.Layouts 1.1
import QtQuick.Controls.Styles 1.2 as Style

MouseArea {
	id: overlay

	property rect boundingBox: "0,0,0x0"
	property string uuid: ""
	property string name: ""

	function open(bb,name,uuid) {
		overlay.boundingBox = bb;
		overlay.uuid = uuid;
		overlayField.focus = true
		overlayField.text = name
		overlayField.selectAll();
		overlay.visible = true
	}

	function close() {
		overlayField.focus = false
		overlay.visible = false
	}

	x: overlay.boundingBox.x
	y: overlay.boundingBox.y
	width: overlay.boundingBox.width
	height: overlay.boundingBox.height
	hoverEnabled: true
	visible: false

	onExited: { close(); }
	onWheel: { close(); }
	onClicked: { close(); }

	Rectangle {
		id: overlayBox

		anchors.left: overlayTip.right
		anchors.top: overlayTip.top
		anchors.topMargin: -5
		anchors.leftMargin: -2

		width: overlayField.width + 10
		height: overlayField.height + 10
		color: "#fafafa"
		border {
			//color: "#9a9a9a"
			color: "#d8dae4"
			width: .7
		}
		radius: 2

		Ctrl.TextField {
			id: overlayField

			x: 5
			y: 5

			style: Style.TextFieldStyle {
				 background: Rectangle { color: "#fafafa"; border { width: 0 } }
			}

			onAccepted: {
				var txt = overlayField.text;

				if(txt !== "") {
					Panopticon.rename_function(uuid,txt);
					overlay.close();
				}
			}

			onEditingFinished: { overlay.close() }
		}
	}

	Canvas {
		id: overlayTip
		x: parent.width - width / 2
		y: 0
		width: 6
		height: 20

		onPaint: {
			var ctx = overlayTip.getContext('2d');

			ctx.fillStyle = "#fafafa";
			ctx.strokeStyle = "#d8dae4";
			ctx.lineWidth = 1;

			ctx.beginPath();
			ctx.moveTo(overlayTip.width,0);
			ctx.lineTo(0,(overlayTip.height - 1)/ 2);
			ctx.lineTo(overlayTip.width,overlayTip.height - 1);
			ctx.closePath();
			ctx.fill();

			ctx.beginPath();
			ctx.moveTo(overlayTip.width - 1,0);
			ctx.lineTo(0,(overlayTip.height - 1)/ 2);
			ctx.lineTo(overlayTip.width -1,overlayTip.height - 1);
			ctx.stroke();
		}
	}
}

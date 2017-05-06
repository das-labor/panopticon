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

import QtQuick 2.3
import QtQuick.Controls 1.2 as Ctrl
import QtQuick.Controls.Styles 1.2 as Style
import QtQuick.Layouts 1.1
import QtGraphicalEffects 1.0

import Panopticon 1.0
import ".."

Item {
	id: editOverlay

	property string variable: ""

	function open(x,y,v) {
		editOverlay.x = x;
		editOverlay.y = y;
		editOverlay.visible = true
		editOverlayField.focus = true
		editOverlay.variable = v;
	}

	function close() {
		editOverlayField.focus = false
		editOverlay.visible = false;
		editOverlayField.text = "";
	}

	z: 2
	visible: false
/*
	Glow {
		anchors.fill: editOverlayBox
		radius: 6
		samples: 17
		color: "#555555"
		source: editOverlayBox
	}*/

	Rectangle {
		id: editOverlayBox
		anchors.topMargin: -1
		anchors.top: editOverlayTip.bottom
		anchors.horizontalCenter: parent.left
		width: editOverlayField.width + 10
		height: editOverlayField.height + 10
		//color: "white"
		color: "#fafafa"
		border {
			//color: "#9a9a9a"
			color: "#d8dae4"
			width: .7
		}
		radius: 2

		Ctrl.TextField {
			id: editOverlayField

			x: 5
			y: 5

			style: Style.TextFieldStyle {
				 background: Rectangle { color: "#fafafa"; border { width: 0 } }
			}

			onAccepted: {
				Panopticon.set_value_for(editOverlay.variable,editOverlayField.text);
			}

			onEditingFinished: { editOverlay.close() }
		}
	}

	Canvas {
		id: editOverlayTip
		x: parent.width - width / 2
		y: 0
		width: 20
		height: 6

		onPaint: {
			var ctx = editOverlayTip.getContext('2d');

			ctx.fillStyle = "#fafafa";
			ctx.strokeStyle = "#d8dae4";
			ctx.lineWidth = 1;

			ctx.beginPath();
			ctx.moveTo(0,editOverlayTip.height);
			ctx.lineTo(editOverlayTip.width / 2,0);
			ctx.lineTo(editOverlayTip.width,editOverlayTip.height);
			ctx.closePath();
			ctx.fill();

			ctx.beginPath();
			ctx.moveTo(0,editOverlayTip.height + 1);
			ctx.lineTo(editOverlayTip.width / 2,0);
			ctx.lineTo(editOverlayTip.width,editOverlayTip.height + 1);
			ctx.stroke();
		}
	}
}

/*
 * Panopticon - A libre disassembler (https://panopticon.re/)
 * Copyright (C) 2014-2015 Kai Michaelis
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

import QtQuick 2.0
import Panopticon 1.0
import QtQuick.Controls 1.3

Item {
	id: root

	signal activated(string uuid);

	property string selection: "";

	onSelectionChanged: {
		functionTable.selection.clear();

		for(var i = 0; i < functionModel.count; i++) {
			var node = functionModel.get(i);

			if(node.uuid == selection) {
				functionTable.selection.select(i);
				return;
			}
		}
	}

	Component.onCompleted: {
		Panopticon.startedFunction.connect(function(uu) {
			var obj = eval(Panopticon.functionInfo(uu));

			obj.name = "<b>Working</b>";
			for(var i = 0; i < functionModel.count; i++) {
				var node = functionModel.get(i);

				if(node.uuid == obj.uuid) {
					functionModel.set(i,obj);
					return;
				}
			}

			console.error("Error: got startedFunction() signal w/ unknown function " + uu);
		});

		Panopticon.discoveredFunction.connect(function(uu) {
			var obj = eval(Panopticon.functionInfo(uu));
			if(obj.type == "todo") {
				obj.name = "<i>Todo</i>";
			}
			functionModel.append(obj);
			functionModel.sort();
		});

		Panopticon.finishedFunction.connect(function(uu) {
			var obj = eval(Panopticon.functionInfo(uu));
			for(var i = 0; i < functionModel.count; i++) {
				var node = functionModel.get(i);

				if(node.uuid == obj.uuid) {
					functionModel.set(i,obj);
					return;
				}
			}

			functionModel.append(obj);
			functionModel.sort();
		});

		Panopticon.changedFunction.connect(function(uu) {
			var obj = eval(Panopticon.functionInfo(uu));
			for(var i = 0; i < functionModel.count; i++) {
				var node = functionModel.get(i);

				if(node.uuid == obj.uuid) {
					functionModel.set(i,obj);
					return;
				}
			}
			console.error("Error: got changedFunction() signal w/ unknown function " + uu);
		});

	}

	ListModel {
		function sortBy(a,b) {
			return parseInt(a.start,10) < parseInt(b.start,10);
		}

		id: functionModel

		function sort() {
			if (count < 2) {
				return;
			}

			var qsort = function(left, right) {
				if (left < right) {
					var pivot = JSON.parse(JSON.stringify(get(right)));
					var i = left - 1;
					var j = right + 1;

					while (true) {
						do {
							j -= 1;
						} while (sortBy(pivot,get(j)));

						do {
							i += 1;
						} while (sortBy(get(i),pivot));

						if (i < j) {
							var t = JSON.parse(JSON.stringify(get(i)));
							set(i,JSON.parse(JSON.stringify(get(j))));
							set(j,t);
						} else {
							break;
						}
					}

					qsort(left,j-1)
					qsort(j+1,right)
				}
			};

			qsort(0,count-1)
		}
	}

	TableView {
		property int renameRow: -1

		id: functionTable
		anchors.fill: parent

    TableViewColumn {
			role: "name"
			title: "Name"
			width: 100
    }
    TableViewColumn {
			role: "start"
			title: "Offset"
			width: 100
			delegate: Item {
				x: 12
			        Label {
				        text: "0x" + styleData.value.toString(16)
				}
			}
		}
		model: functionModel
		enabled: !edit.visible
		focus: !edit.visible

		onClicked: {
			root.selection = functionModel.get(row).uuid;
		}

		onActivated: {
			root.activated(functionModel.get(row).uuid);
		}

		onDoubleClicked: {
			functionTable.renameRow = row
		}

		itemDelegate: Item {
			x: 12

			Label {
				id: view
				text: styleData.value
			}

			Binding {
				when: (functionTable.renameRow == styleData.row && styleData.column == 0)
				value: parent
				target: edit
				property: "targetRow"
			}

			Binding {
				when: !(functionTable.renameRow == styleData.row && styleData.column == 0)
				value: null
				target: edit
				property: "targetRow"
			}

		}
	}

	Canvas {
		property var targetRow: null;

		readonly property int tipHeight: 12
		readonly property int bubblePadding: 8
		readonly property int bubbleRadius: 4

		id: edit
		x: (targetRow === null ? 0 : mapFromItem(targetRow,targetRow.x + 20,0).x)
		y: (targetRow === null ? 0 : mapFromItem(targetRow,0,targetRow.y + targetRow.height).y)
		width: editField.width + 2 * bubblePadding
		height: editField.height + 2 * bubblePadding + tipHeight
		visible: targetRow !== null

		onPaint: {
			var ctx = edit.getContext('2d');

			const corner_sz = edit.bubbleRadius;
			const tip_apex = 25;
			const tip_w = 20;
			const tip_h = edit.tipHeight;

			/*
			 *       tip_apex
			 *          /\
			 * .-------'  `-----. - top
			 * |       | tip_end|
			 * |    tip_start   |
			 * '----------------' - bottom
			 */

			const top = tip_h;
			const bottom = edit.height - 1;
			const tip_start = tip_apex - tip_w / 2;
			const tip_end = tip_start + tip_w;
			const end = edit.width - 1;


			ctx.fillStyle = "#efecca";
			ctx.strokeStyle = "black";
			ctx.lineWidth = 0.5;

			ctx.clearRect(0,0,width,height);
			ctx.beginPath();

			ctx.moveTo(1 + corner_sz,top);
			ctx.lineTo(tip_start,top);
			ctx.lineTo(tip_apex,0);
			ctx.lineTo(tip_end,top);
			ctx.lineTo(end - corner_sz,top);
			ctx.arc(end - corner_sz,top + corner_sz,corner_sz,1.5 * Math.PI,0,false);
			ctx.lineTo(end,bottom - corner_sz);
			ctx.arc(end - corner_sz,bottom - corner_sz,corner_sz,0,0.5 * Math.PI,false);
			ctx.lineTo(1 + corner_sz,bottom);
			ctx.arc(1 + corner_sz,bottom - corner_sz,corner_sz,0.5 * Math.PI,Math.PI,false);
			ctx.lineTo(1,top + corner_sz);
			ctx.arc(1 + corner_sz,top + corner_sz,corner_sz,Math.PI,1.5 * Math.PI,false);

			ctx.fill();
			ctx.stroke();
		}

		TextField {
			id: editField
			x: edit.bubblePadding
			y: edit.bubblePadding + edit.tipHeight
			focus: edit.visible
			validator: RegExpValidator {
				regExp: /[a-zA-Z0-9 .;:|<>,@{}\[\]!$%&*()-]+/
			}

			onVisibleChanged: {
				if (edit.targetRow !== null) {
					text = edit.targetRow.styleData.value
					selectAll()
				}
			}

			onEditingFinished: {
				functionTable.renameRow = -1;
				editField.text = ""
			}

			onAccepted: {
				var row = edit.targetRow.styleData.row;
				Panopticon.setName(functionModel.get(row).uuid,editField.text);
				functionTable.renameRow = -1;
				editField.text = ""
			}
		}
	}

}

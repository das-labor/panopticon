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

import QtQuick 2.3
import QtQuick.Controls 1.2
import QtQml.Models 2.1
import Panopticon 1.0
import ".."

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

	ListModel {
		function lessThan(i,j) {
			var a = get(i)
			var b = get(j)

			return a.entry_point < b.entry_point;
		}

		id: functionModel
		dynamicRoles: true

		function sort() {
			if (count < 2) {
				return;
			}

			var qsort = function(left, right) {
				if (left < right) {
					var pivot = right;
					var i = left - 1;
					var j = right + 1;

					while (true) {
						do {
							j -= 1;
						} while (lessThan(pivot,j));

						do {
							i += 1;
						} while (lessThan(i,pivot));

						if (i < j) {
							move(i,j,1);
							move(j-1,i,1);
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

		Component.onCompleted: {
			Functions.added.connect(function(row) {
				var obj = JSON.parse(JSON.stringify(Functions.get(row)));

				obj.row = row;

				if(obj.failed || obj.empty) {
					return;
				}

				functionModel.append(obj);
				sort()
			})
			Functions.changed.connect(function(row) {
				var obj = JSON.parse(JSON.stringify(Functions.get(row)));

				obj.row = row;

				for(var i = 0; i < count; i++) {
					if(get(i).row == row) {
						if(obj.failed || obj.empty) {
							functionModel.remove(i,1);
						} else {
							functionModel.set(i,obj);
							sort();
						}
						return;
					}
				}

				if(!(obj.failed || obj.empty)) {
					functionModel.append(obj);
					sort()
				}
			})
			Functions.removed.connect(function(row) {
				for(var i = 0; i < count; i++) {
					if(get(i).row == row) {
						functionModel.remove(i);
						return;
					}
				}
			})

		}
	}

	TableView {
		id: functionTable
		anchors.fill: parent

		TableViewColumn {
			role: "name"
			title: "Name"
			width: 100
			delegate: Item {
				x: 12
				Label {
					text: styleData.value != undefined ? styleData.value : ""
				}
			}
		}

		TableViewColumn {
			role: "entry_point"
			title: "Offset"
			width: 100
			delegate: Item {
				x: 12
				Label {
					text: styleData.value != undefined ? "0x" + styleData.value.toString(16) : "-"
				}
			}
		}
		model: functionModel

		onClicked: {
			root.selection = functionModel.get(row).uuid;
			console.log(JSON.stringify(functionModel.get(row)));
		}

		onActivated: {
			root.activated(functionModel.get(row).uuid);
			console.log(JSON.stringify(functionModel.get(row)));
		}

		onDoubleClicked: {
			root.activated(functionModel.get(row).uuid);
		}
	}
}

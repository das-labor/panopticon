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
import QtQuick.Controls 1.2 as Ctrl
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
		id: functionModel
		dynamicRoles: true

		Component.onCompleted: {
			Functions.added.connect(function(row) {
				var obj = JSON.parse(JSON.stringify(Functions.get(row)));

				obj.row = row;

				if(obj.failed || obj.empty || obj.kind == "todo") {
					return;
				}

				if(obj.entry_point !== undefined) {
					for(var j = 0; j < functionModel.count; j++) {
						var ent = get(j).entry_point;
						if(ent !== undefined && ent > obj.entry_point) {
							functionModel.insert(j, obj);
							return;
						}
					}
				}

				functionModel.append(obj);
			})

			Functions.changed.connect(function(row) {
				var obj = JSON.parse(JSON.stringify(Functions.get(row)));

				obj.row = row;

				for(var i = 0; i < functionModel.count; i++) {
					if(get(i).row == row) {
						functionModel.remove(i, 1);
						break;
					}
				}

				if(!obj.failed && !obj.empty) {
					for(var j = 0; j < functionModel.count; j++) {
						var ent = get(j).entry_point;
						if(ent !== undefined && ent > obj.entry_point) {
							functionModel.insert(j, obj);

							if(root.selection === "") {
								root.selection = obj.uuid;
								root.activated(obj.uuid);
							}

							return;
						}
					}
					functionModel.insert(functionModel.count, obj);
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

	Ctrl.TableView {
		id: functionTable
		anchors.fill: parent

		Ctrl.TableViewColumn {
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

		Ctrl.TableViewColumn {
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
		}

		onActivated: {
			root.activated(functionModel.get(row).uuid);
		}

		onDoubleClicked: {
			root.activated(functionModel.get(row).uuid);
		}
	}
}

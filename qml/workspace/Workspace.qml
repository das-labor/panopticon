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

import Panopticon 1.0
import ".."

Item {
	id: root

	property string selection: "";

	Component {
		id: errorPopup
		ErrorPopup {}
	}

	function displayError(msg) {
		window.enabled = false;
		try {
			errorPopup.createObject(window).displayMessage(msg);
		} catch(e) {
			window.enabled = true;
			throw e;
		}
		window.enabled = true;
	}

	FunctionTable {
		id: functionTable
		height: root.height
		width: 300

		onSelectionChanged: {
			/*if(callgraph.item !== null) {
				callgraph.item.selection = selection;
			}*/
			if(cflow_graph.item !== null) {
				cflow_graph.item.selection = selection;
			}
			root.selection = selection;
		}
	}

	Ctrl.TabView {
		id: tabs
		height: root.height
		width: root.width - 300
		x: 300

		/*Ctrl.Tab {
			id: callgraph
			title: "Call Graph"

			Callgraph {
				onSelectionChanged: {
					functionTable.selection = selection;
				}
			}
		}*/

		Ctrl.Tab {
			id: cflow_graph
			title: "Control Flow"
			state: ""

			onLoaded: item.selection = root.selection

			ControlFlowGraph {
				id: cfg
				anchors.fill: parent
			}
		}
	}
}

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
import "."

Item {
	id: root

	property string selection: "";

	Component.onCompleted: {
		Panopticon.changedFunction.connect(function(uu) {
			if (uu == selection) {
				var cfg = JSON.parse(Panopticon.functionCfg(selection));

				if(cflow_graph.item !== null && cflow_graph.item.bblockList !== null) {
					for (var i in cflow_graph.item.bblockList) {
						if(cflow_graph.item.bblockList.hasOwnProperty(i) && cfg.contents[i] !== undefined) {
							cflow_graph.item.bblockList[i].contents = cfg.contents[i];
						}
					}
				}
			}
		});

		Panopticon.start()
	}

	FunctionTable {
		id: functionTable
		height: root.height
		width: 300

		onSelectionChanged: {
			if(callgraph.item !== null) {
				callgraph.item.selection = selection;
			}
			if(cflow_graph.item !== null) {
				cflow_graph.item.selection = selection;
			}
			root.selection = selection;
		}
	}

	TabView {
		id: tabs
		height: root.height
		width: root.width - 300
		x: 300

		Tab {
			id: callgraph
			title: "Call Graph"

			Callgraph {
				onSelectionChanged: {
					functionTable.selection = selection;
				}
			}
		}

		Tab {
			id: cflow_graph
			title: "Control Flow"

			onLoaded: item.selection = root.selection

			Item {
				clip: true

				Rectangle {
					anchors.fill: parent
					color: "#efefef"
				}

				Canvas {
					id: graph

					x: bblockRoot.x + bblockRoot.childrenRect.x
					y: bblockRoot.y + bblockRoot.childrenRect.y
					width: bblockRoot.childrenRect.width
					height: bblockRoot.childrenRect.height

					property var edges: null;
					property var boxes: null;

					onPaint: {
						var ctx = graph.getContext('2d');
						var func = JSON.parse(Panopticon.functionInfo(selection));
						var cfg = JSON.parse(Panopticon.functionCfg(selection));

						ctx.clearRect(0,0,width,height);

						for(var i = 0; i < cfg.edges.length; i++) {
							var from = cfg.edges[i].from;
							var to = cfg.edges[i].to;
						}

						if(edges !== null) {
							for(var i = 0; i < graph.edges.length; i++) {
								var e = graph.edges[i];

								ctx.beginPath();
								ctx.moveTo(e.x1 - bblockRoot.childrenRect.x,e.y1 - bblockRoot.childrenRect.y);
								ctx.lineTo(e.x2 - bblockRoot.childrenRect.x,e.y2 - bblockRoot.childrenRect.y);
								ctx.stroke();
							}
						}
					}

					onEdgesChanged: requestPaint();
					onBoxesChanged: requestPaint();

					MouseArea {
						x: -cflow_graph.width
						y: -cflow_graph.height
						width: parent.width + 2*cflow_graph.width
						height: parent.height + 2*cflow_graph.height
						drag.target: bblockRoot
						drag.axis: Drag.XAndYAxis
						drag.minimumX: -bblockRoot.childrenRect.width - bblockRoot.childrenRect.x + 10
						drag.minimumY: -bblockRoot.childrenRect.height - bblockRoot.childrenRect.y + 10
						drag.maximumX: cflow_graph.width - bblockRoot.childrenRect.x - 10
						drag.maximumY: cflow_graph.height - bblockRoot.childrenRect.y - 10
						cursorShape: containsMouse && pressed ? Qt.ClosedHandCursor : Qt.OpenHandCursor
					}
				}

				Item {
					id: bblockRoot
				}

				property string selection: "";
				property var bblockList: null;

				Component.onCompleted: {
					Panopticon.layoutedFunction.connect(function(_res) {
						var cfg = JSON.parse(Panopticon.functionCfg(selection));
						var res = JSON.parse(_res);
						var pos = res[0];

						for (var k in pos) {
							if(pos.hasOwnProperty(k)) {
								var obj = bblockList[k];

								obj.visible = true;
								obj.x = pos[k].x - obj.width / 2;
								obj.y = pos[k].y - obj.height / 2;
							}
						}

						for (var i = 0; i < cfg.edges.length; i++) {
							var from = bblockList[cfg.edges[i].from];
							var to = bblockList[cfg.edges[i].to];
						}

						graph.edges = res[1];
					});
				}

				onSelectionChanged: {
					var cfg = JSON.parse(Panopticon.functionCfg(selection));
					var func = JSON.parse(Panopticon.functionInfo(selection));
					var dims = {};

					if(cflow_graph.item.bblockList != null) {
						for (var i in bblockList) {
							if(bblockList.hasOwnProperty(i)) {
								bblockList[i].visible = false;
								bblockList[i].destroy();
							}
						}
					}

					bblockList = {};
					var bblock = Qt.createComponent("BasicBlock.qml");
					for(var i = 0; i < cfg.nodes.length; i++) {
						var node = cfg.nodes[i];
						var c = {
							"contents":cfg.contents[node] ,
							"color":(node == cfg.head ? "red" : "steelblue"),
						};

						while (bblock.status != Component.Ready && bblock.status != Component.Error) {
							sleep(1);
						}
						if (bblock.status == Component.Error) {
							console.error(bblock.errorString())
						} else {
							var obj = bblock.createObject(bblockRoot,c);

							obj.visible = false;
							bblockList[node] = obj;

							dims[node] = {"width":obj.width,"height":obj.height};
						}
					}

					if(cfg.nodes.length > 1) {
						Panopticon.sugiyamaLayout(selection,JSON.stringify(dims),100,30);
					} else {
						for (var i in bblockList) {
							if(bblockList.hasOwnProperty(i)) {
								bblockList[i].visible = true;
							}
						}
					}

					graph.edges = [];
				}
			}
		}
	}
}

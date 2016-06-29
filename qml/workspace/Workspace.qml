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

import Panopticon 1.0
import ".."

Item {
	id: root

	property string selection: "";

	Component {
		id: basicBlock
		BasicBlock {}
	}

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

	Component.onCompleted: {
		Panopticon.changedFunction.connect(function(uu) {
			if (uu == selection) {
				var res = JSON.parse(Panopticon.functionCfg(selection));

				if(res.status == "ok") {
					var cfg = res.payload;

					if(cflow_graph.item !== null && cflow_graph.item.bblockList !== null) {
						for (var i in cflow_graph.item.bblockList) {
							if(cflow_graph.item.bblockList.hasOwnProperty(i) && cfg.contents[i] !== undefined) {
								cflow_graph.item.bblockList[i].contents = cfg.contents[i];
							}
						}
					}
				} else {
					displayError(res.error);
				}
			}
		});
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
			state: ""

			onLoaded: item.selection = root.selection

			property string errorMessage: ""

			Item {
				clip: true

				Rectangle {
					anchors.fill: parent
					color: "#efefef"
				}

				Canvas {
					id: graph

					x: bblockRoot.x + bblockRoot.childrenRect.x - 500
					y: bblockRoot.y + bblockRoot.childrenRect.y - 500
					width: bblockRoot.childrenRect.width + 1000
					height: bblockRoot.childrenRect.height + 1000

					property var edges: null;
					property var boxes: null;

					function draw_arrow_head(x,y,ctx) {
						var dim = 10;

						ctx.save();
						ctx.translate(x,y);
						ctx.rotate(Math.PI);
						ctx.beginPath();
						ctx.moveTo(0,dim / -2);
						ctx.lineTo(dim / 2,dim / 2);
						ctx.lineTo(0,dim / 3);
						ctx.lineTo(dim / -2,dim / 2);
						ctx.lineTo(0,dim / -2);
						ctx.stroke();
						ctx.fill();
						ctx.restore();
					}

					onPaint: {
						var ctx = graph.getContext('2d');
						var func_res = JSON.parse(Panopticon.functionInfo(selection));
						var cfg_res = JSON.parse(Panopticon.functionCfg(selection));

						if (func_res.status != "ok") {
							cflow_graph.errorMessage = func_res.error
							cflow_graph.state = "ERROR"
							return;
						} else {
							cflow_graph.state = ""
						}

						if (cfg_res.status != "ok") {
							cflow_graph.errorMessage = cfg_res.error
							cflow_graph.state = "ERROR"
							return;
						} else {
							cflow_graph.state = ""
						}

						var func = func_res.payload;
						var cfg = cfg_res.payload;

						ctx.clearRect(0,0,width,height);

						for(var i = 0; i < cfg.edges.length; i++) {
							var from = cfg.edges[i].from;
							var to = cfg.edges[i].to;
						}

						if(edges !== null) {
							for (var conn in graph.edges) {
								if(graph.edges.hasOwnProperty(conn)) {
									var segs = graph.edges[conn].segments;

									for(var i = 0; i < segs.length; i++) {
										var e = segs[i];

										ctx.beginPath();
										ctx.moveTo(e.x1 - bblockRoot.childrenRect.x + 500,e.y1 - bblockRoot.childrenRect.y + 500);
										ctx.lineTo(e.x2 - bblockRoot.childrenRect.x + 500,e.y2 - bblockRoot.childrenRect.y + 500);
										ctx.stroke();
									}

									draw_arrow_head(
										graph.edges[conn].head_offset.x - bblockRoot.childrenRect.x + 500,
										graph.edges[conn].head_offset.y - bblockRoot.childrenRect.y - 5 + 500,ctx);
								}
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

						onWheel: {
							bblockRoot.y = Math.max(drag.minimumY,Math.min(drag.maximumY,bblockRoot.y + wheel.angleDelta.y / 10));
							bblockRoot.x = Math.max(drag.minimumX,Math.min(drag.maximumX,bblockRoot.x + wheel.angleDelta.x / 10));
						}
					}
				}

				Item {
					id: bblockRoot
				}

				Rectangle {
					anchors.fill: parent
					width: childrenRect.width
					height: childrenRect.height
					visible: cflow_graph.state == "ERROR"
					color: "#efefef"

					Label {
						anchors.fill: parent
						horizontalAlignment: Text.AlignHCenter
						verticalAlignment: Text.AlignVCenter
						wrapMode: Text.WordWrap
						font.pixelSize: 21
						color: "#333"
						text: errorMessage
					}
				}

				property string selection: "";
				property var bblockList: null;

				Component.onCompleted: {
					Panopticon.layoutedFunction.connect(function(_layout) {
						console.log("layouted!")
						var cfg_res = JSON.parse(Panopticon.functionCfg(selection));
						var layout = JSON.parse(_layout);

						if (cfg_res.status != "ok") {
							cflow_graph.errorMessage = cfg_res.error
							cflow_graph.state = "ERROR"
							return;
						} else {
							cflow_graph.state = ""
						}

						var cfg = cfg_res.payload;
						var pos = layout[0];
						var entry = undefined;
						var num_blocks = 0;

						for (var k in pos) {
							if(pos.hasOwnProperty(k)) {
								var obj = bblockList[k];

								obj.visible = true;
								obj.x = pos[k].x - obj.width / 2;
								obj.y = pos[k].y - obj.height / 2;

								if (k == cfg.entry) {
									entry = obj;
								}

								num_blocks += 1;
							}
						}

						console.error(num_blocks.toString() + " blocks!");

						for (var i = 0; i < cfg.edges.length; i++) {
							var from = bblockList[cfg.edges[i].from];
							var to = bblockList[cfg.edges[i].to];
						}

						graph.edges = layout[1];

						if (entry !== undefined) {
							var ent_pos = mapFromItem(bblockRoot,entry.x,entry.y);
							var tab_pos = mapFromItem(cflow_graph,cflow_graph.x,cflow_graph.y);
							var tab_cent_x = tab_pos.x + cflow_graph.width / 2;

							bblockRoot.x = tab_cent_x - (bblockRoot.childrenRect.width / 2) - bblockRoot.childrenRect.x;
							bblockRoot.y = tab_pos.y + 100 - bblockRoot.childrenRect.y;
						}
					});
				}

				onSelectionChanged: {
					var cfg_str = Panopticon.functionCfg(selection);
					var cfg_res = JSON.parse(cfg_str);
					var func_res = JSON.parse(Panopticon.functionInfo(selection));
					var dims = {};

					if (cfg_res.status != "ok") {
						cflow_graph.errorMessage = cfg_res.error
						cflow_graph.state = "ERROR"
						return;
					} else {
						cflow_graph.state = ""
					}

					var cfg = cfg_res.payload;

					if (func_res.status != "ok") {
						cflow_graph.errorMessage = func_res.error
						cflow_graph.state = "ERROR"
						return;
					} else {
							cflow_graph.state = ""
					}

					var func = func_res.payload;

					if(cflow_graph.item.bblockList != null) {
						for (var i in bblockList) {
							if(bblockList.hasOwnProperty(i)) {
								bblockList[i].visible = false;
								bblockList[i].destroy();
							}
						}
					}

					var res = JSON.parse(Panopticon.functionApproximate(selection));
					if(res.status == "ok") {
						var approx = res.payload;
					} else {
						console.error(res.error);
						var approx = [];
					}

					bblockList = {};
					for(var i = 0; i < cfg.nodes.length; i++) {
						var node = cfg.nodes[i];

						if(cfg.code[node] != undefined) {
							var c = {
								"code":cfg.code[node],
								"mode":"RESOLVED",
								"approx": approx,
							};
						} else if(cfg.targets[node] != undefined) {
							var c = {
								"target":cfg.targets[node],
								"mode":"UNRESOLVED",
								"approx": approx,
							};
						} else if(cfg.errors[node] != undefined) {
							var c = {
								"target":cfg.errors[node],
								"mode":"UNRESOLVED",
								"approx": approx,
							};
						} else {
							console.error("Node '" + node.toString() + "' has neither code nor target");
						}

						var obj = basicBlock.createObject(bblockRoot,c);

						obj.visible = false;
						bblockList[node] = obj;

						dims[node] = {"width":obj.width,"height":obj.height};
					}
					console.log(JSON.stringify(cfg.nodes));

					if(cfg.nodes.length > 1) {
						var res = JSON.parse(Panopticon.sugiyamaLayout(selection,JSON.stringify(dims),100,30,8));
						console.log(JSON.stringify(res));
						if(res.status != "ok") {
							cflow_graph.errorMessage = res.error
							cflow_graph.state = "ERROR"
							console.error(res.error);
						}
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

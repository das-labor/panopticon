/*
 * Panopticon - A libre disassembler (https://panopticon.re/)
 * Copyright (C) 2014-2016 Kai Michaelis
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

Rectangle {
	id: root
	color: "#efefef"
	clip: true

	// EMPTY, LOADING, ERROR, LOADED
	state: "EMPTY"

	property string selection: "";
	property string errorMessage: ""

	Component {
		id: basicBlock
		BasicBlock {}
	}

	Ctrl.BusyIndicator {
		anchors.centerIn: parent
		running: root.state === "LOADING"
	}

	MouseArea {
		anchors.fill: parent
		hoverEnabled: true
		cursorShape: {
			if(pressed && containsMouse) {
				return Qt.ClosedHandCursor;
			} else {
				return Qt.OpenHandCursor;
			}
		}
		onWheel: {
			if(wheel.modifiers & Qt.ControlModifier) {
				if(wheel.angleDelta.y < 0) {
					nodeScale.xScale *= 0.95
					nodeScale.yScale *= 0.95
				} else {
					nodeScale.xScale *= 1.05
					nodeScale.yScale *= 1.05
				}

				nodeScale.xScale = Math.min(nodeScale.xScale, 1);
				nodeScale.yScale = Math.min(nodeScale.yScale, 1);
				nodeScale.xScale = Math.max(nodeScale.xScale, 0.000001);
				nodeScale.yScale = Math.max(nodeScale.yScale, 0.000001);
			} else if(wheel.modifiers === 0) {
				nodeRoot.y += wheel.angleDelta.y * 1 / nodeScale.yScale;
				nodeRoot.x += wheel.angleDelta.x * 1 / nodeScale.xScale;
			} else {
				wheel.accepted = false
			}
		}

		property int fixX: 0
		property int fixY: 0

		onPositionChanged: {
			if(mouse.buttons & Qt.LeftButton != 0) {
				nodeRoot.x += (mouse.x - fixX) * 1 / nodeScale.xScale;
				nodeRoot.y += (mouse.y - fixY) * 1 / nodeScale.yScale;
			} else {
				wheel.accepted = false
			}
			fixX = mouse.x;
			fixY = mouse.y;
		}
	}

	Component.onCompleted: {
		/*
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
		 });*/

		 Panopticon.layoutedFunction.connect(function(_layout) {
			 var cfg_res = JSON.parse(Panopticon.functionCfg(selection));
			 var layout = JSON.parse(_layout);

			 if (cfg_res.status != "ok") {
				 root.errorMessage = cfg_res.error
				 root.state = "ERROR"
				 console.exception(cfg_res.error);
				 return;
			 }

			 var cfg = cfg_res.payload;
			 var pos = layout[0];
			 var entry = undefined;
			 var num_blocks = 0;

			 for (var i in nodeRoot.children) {
				 var obj = nodeRoot.children[i];
				 var p = pos[obj.name];

				 obj.visible = true;
				 obj.x = p.x - obj.width / 2;
				 obj.y = p.y - obj.height / 2;

				 if (obj.name == cfg.entry_point) {
					 entry = obj;
				 }

				 num_blocks += 1;
			 }

			 if(nodeRoot.childrenRect.x !== 0 || nodeRoot.childrenRect.y !== 0) {
				 var off_x = nodeRoot.childrenRect.x;
				 var off_y = nodeRoot.childrenRect.y;

				 for (var conn in layout[1]) {
					 if(layout[1].hasOwnProperty(conn)) {
						 var segs = layout[1][conn].segments;

						 for(var i = 0; i < segs.length; i++) {
							 var e = segs[i];

							 off_x = Math.min(off_x, Math.min(e.x1, e.x2));
							 off_y = Math.min(off_y, Math.min(e.y1, e.y2));
						 }
					 }
				 }

				 for (var i in nodeRoot.children) {
					 var obj = nodeRoot.children[i];

					 obj.x -= off_x;
					 obj.y -= off_y;
					 obj.visible = true;
				 }
			 }

			 root.state = "LOADED"

			 edgeCanvas.edges = layout[1];
			 edgeCanvas.xCorrection = off_x;
			 edgeCanvas.yCorrection = off_y;
			 nodeRoot.x = 0;
			 nodeRoot.y = 0;

			 if (entry !== undefined) {
				 var mid = mapFromItem(nodeRoot, entry.x + entry.width / 2, entry.y + entry.height / 2);
				 nodeRoot.x = (root.width / 2) - mid.x;
				 nodeRoot.y = (root.height / 3) - mid.y;
			 }
		 });
	 }

	 // Initiate layouting
	 onSelectionChanged: {
		 var cfg_res = JSON.parse(Panopticon.functionCfg(selection));
		 var func_res = JSON.parse(Panopticon.functionInfo(selection));
		 var dims = {};

		 root.state = "LOADING"

		 for (var i in nodeRoot.children) {
			 nodeRoot.children[i].visible = false;
			 nodeRoot.children[i].destroy();
		 }

		 if (cfg_res.status != "ok") {
			 root.errorMessage = cfg_res.error
			 root.state = "ERROR"
			 console.exception(cfg_res.error);
			 return;
		 }

		 var cfg = cfg_res.payload;

		 if (cfg.nodes.length == 0) {
			 root.errorMessage = "Function is empty"
			 root.state = "ERROR"
			 console.exception("Function is empty");
			 return;
		 }

		 if (cfg.nodes.length == 1 && cfg.nodes[0].substr(0, 3) == "err") {
			 root.errorMessage = "Disassembly failed: " + cfg.errors[cfg.nodes[0]];
			 root.state = "ERROR"
			 console.exception("Disassembly failed: " + cfg.errors[cfg.nodes[0]]);
			 return;
		 }

		 if (func_res.status != "ok") {
			 root.errorMessage = func_res.error
			 root.state = "ERROR"
			 console.exception(func_res.error);
			 return;
		 }

		 var func = func_res.payload;
		 var approx = [];

		 /*var res = JSON.parse(Panopticon.functionApproximate(selection));
			if(res.status == "ok") {
				var approx = res.payload;
			} else {
				console.exception(res.error);
				var approx = [];
			}*/

			for(var i = 0; i < cfg.nodes.length; i++) {
				var node = cfg.nodes[i];

				if(cfg.code[node] != undefined) {
					var c = {
						"code":cfg.code[node],
						"name":node,
						"mode":"RESOLVED",
						"approx": approx,
					};
				} else if(cfg.targets[node] != undefined) {
					var c = {
						"name":node,
						"target":cfg.targets[node],
						"mode":"UNRESOLVED",
						"approx": approx,
					};
				} else if(cfg.errors[node] != undefined) {
					var c = {
						"name":node,
						"target":cfg.errors[node],
						"mode":"UNRESOLVED",
						"approx": approx,
					};
				} else {
					console.exception("Node '" + node.toString() + "' has neither code nor target");
				}

				var obj = basicBlock.createObject(nodeRoot, c);

				obj.visible = false;
				dims[node] = {"width":obj.width, "height":obj.height};
			}

			var res = JSON.parse(Panopticon.sugiyamaLayout(selection, JSON.stringify(dims), 100, 30, 8));
			if(res.status != "ok") {
				root.errorMessage = res.error
				root.state = "ERROR"
				console.exception(res.error);
			}
		}

		Rectangle {
			anchors.fill: parent
			width: childrenRect.width
			height: childrenRect.height
			visible: root.state === "ERROR"
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

		Canvas {
			property var edges: null;
			property int xCorrection: 0;
			property int yCorrection: 0;
			readonly property int margin: 5000;

			id: edgeCanvas
			antialiasing: true
			x: nodeRoot.x - margin
			y: nodeRoot.y - margin
			width: nodeRoot.width + 2 * margin
			height: nodeRoot.height + 2 * margin
			visible: root.state === "LOADED"

			onEdgesChanged: requestPaint();

			transform: Scale {
				xScale: nodeScale.xScale
				yScale: nodeScale.yScale
				origin {
					x: nodeScale.origin.x + edgeCanvas.margin
					y: nodeScale.origin.y + edgeCanvas.margin
				}
			}

			onPaint: {
				var ctx = edgeCanvas.getContext('2d');
				var func_res = JSON.parse(Panopticon.functionInfo(selection));
				var cfg_res = JSON.parse(Panopticon.functionCfg(selection));
				var x_corr = xCorrection - margin
				var y_corr = yCorrection - margin

				if (func_res.status != "ok") {
					root.errorMessage = func_res.error
					root.state = "ERROR"
					return;
				}

				if (cfg_res.status != "ok") {
					root.errorMessage = cfg_res.error
					root.state = "ERROR"
					return;
				}

				var func = func_res.payload;
				var cfg = cfg_res.payload;

				ctx.clearRect(0, 0, width, height);

				if(edges !== null) {
					for (var conn in edgeCanvas.edges) {
						if(edgeCanvas.edges.hasOwnProperty(conn)) {
							var segs = edgeCanvas.edges[conn].segments;

							for(var i = 0; i < segs.length; i++) {
								var e = segs[i];

								ctx.beginPath();
								ctx.moveTo(e.x1 - x_corr, e.y1 - y_corr);
								ctx.lineTo(e.x2 - x_corr, e.y2 - y_corr);
								ctx.stroke();
							}

							draw_arrow_head(
								edgeCanvas.edges[conn].head_offset.x - x_corr,
								edgeCanvas.edges[conn].head_offset.y - y_corr - 5, ctx);
							}
						}
					}
				}

				function draw_arrow_head(x, y, ctx) {
					var dim = 10;

					ctx.save();
					ctx.translate(x, y);
					ctx.rotate(Math.PI);
					ctx.beginPath();
					ctx.moveTo(0, dim / -2);
					ctx.lineTo(dim / 2, dim / 2);
					ctx.lineTo(0, dim / 3);
					ctx.lineTo(dim / -2, dim / 2);
					ctx.lineTo(0, dim / -2);
					ctx.stroke();
					ctx.fill();
					ctx.restore();
				}
			}

			Item {
				id: nodeRoot
				width: childrenRect.width
				height: childrenRect.height
				visible: root.state == "LOADED"
				transform: Scale {
					id: nodeScale
					xScale: 1
					yScale: 1
					origin {
						x: parent.width / 2 - nodeRoot.x
						y: parent.height / 2 - nodeRoot.y
					}
				}
			}
		}

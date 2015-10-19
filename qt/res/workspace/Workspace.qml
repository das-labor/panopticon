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
				clip: true;

				Rectangle {
					anchors.fill: parent
					color: "#efefef"
				}

				Canvas {
					id: graph

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
								var from = graph.edges[i].from;
								var to = graph.edges[i].to;

								ctx.beginPath();
								ctx.moveTo(Math.max(1,from.x),Math.max(1,from.y));
								ctx.lineTo(Math.max(1,to.x),Math.max(1,to.y));
								ctx.stroke();
							}
						}
					}

					onEdgesChanged: requestPaint();
					onBoxesChanged: requestPaint();

					MouseArea {
						anchors.fill: parent;
						drag.target: parent
						drag.axis: Drag.XAndYAxis
						cursorShape: containsMouse && pressed ? Qt.ClosedHandCursor : Qt.OpenHandCursor
					}
				}

				anchors.fill: parent

				property string selection: "";
				property var bblockList: null;

				Component.onCompleted: {
					Panopticon.layoutedFunction.connect(function(_pos) {
						var cfg = JSON.parse(Panopticon.functionCfg(selection));
						var pos = JSON.parse(_pos);
						var right = 0;
						var bottom = 0;
						var obstacles = [];
						var waypoints = [];

						for (var k in pos) {
							if(pos.hasOwnProperty(k)) {
								var obj = bblockList[k];

								obj.visible = true;
								obj.x = pos[k].x - obj.width / 2 + 100;
								obj.y = pos[k].y - obj.height / 2 + 100;


								right = Math.max(right,obj.x + obj.width);
								bottom = Math.max(bottom,obj.y + obj.height);

								obstacles.push({"x":obj.x,"y":obj.y,"width":obj.width,"height":obj.height});
								waypoints.push({"x":obj.x - 3,"y":obj.y - 3});
								waypoints.push({"x":obj.x - 3,"y":obj.y + obj.height + 3 });
								waypoints.push({"x":obj.x + obj.width + 3,"y":obj.y - 3});
								waypoints.push({"x":obj.x + obj.width + 3,"y":obj.y + obj.height + 3});
							}
						}

						var in_degree = {};
						var out_degree = {};

						for(var i = 0; i < cfg.nodes.length; i++) {
							in_degree[cfg.nodes[i]] = 0;
							out_degree[cfg.nodes[i]] = 0;
						}

						for(var l = 0; l < cfg.edges.length; l++) {
							if(cfg.edges[l].from != cfg.edges[l].to) {
								in_degree[cfg.edges[l].to] += 1;
								out_degree[cfg.edges[l].from] += 1;
							}
						}

						var edges = [];
						for (var i = 0; i < cfg.edges.length; i++) {
							var from = bblockList[cfg.edges[i].from];
							var to = bblockList[cfg.edges[i].to];

							waypoints.push({"x": to.x + to.width / 2,"y": to.y - 5});
							waypoints.push({"x": from.x + from.width / 2,"y": from.y + from.height + 5});
							edges.push({"from": waypoints.length - 1, "to": waypoints.length - 2});
						}


						graph.width = right + 200;
						graph.height = bottom + 200;

						graph.y = (cflow_graph.item.height - graph.height) / 2;
						graph.x = (cflow_graph.item.width - graph.width) / 2;

						Panopticon.dijkstraRoute(JSON.stringify({"obstacles":obstacles,"waypoints":waypoints,"edges":edges}));
					});

					Panopticon.routedFunction.connect(function(_paths) {
						var paths = JSON.parse(_paths);
						var canvas_edges = [];

						for (var j = 0; j < paths.length; j++) {
							var p = paths[j];

							if (p.length >= 2) {
								for (var l = 1; l < p.length; l++) {
									canvas_edges.push({"from":p[l-1],"to":p[l]});
								}
							}
						}

						graph.edges = canvas_edges;
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
							var obj = bblock.createObject(graph,c);

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

						graph.y = (cflow_graph.item.height - graph.height) / 2;
						graph.x = (cflow_graph.item.width - graph.width) / 2;
					}

					graph.edges = [];
				}
			}
		}
	}
}

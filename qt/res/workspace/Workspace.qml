import QtQuick 2.0
import Panopticon 1.0
import QtQuick.Controls 1.3
import "."

Item {
	id: root

	property string selection: "";

	Component.onCompleted: {
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
				Component {
					id: bblock

					Rectangle {
						width: childrenRect.width + 10;
						height: childrenRect.height + 10;
						color: "steelblue";
						border.width: 1;
						border.color: "black";

						property string contents: "";

						Text {
							x: 5
							y: 5
							height: contentHeight
							width: contentWidth
							text: contents
						}
					}
				}

				Canvas {
					width: childrenRect.width;
					height: childrenRect.height;
					id: graph

					property var edges: null;
					property var boxes: null;

					onPaint: {
						var ctx = graph.getContext('2d');
						var func = eval(Panopticon.functionInfo(selection));
						var cfg = eval(Panopticon.functionCfg(selection));

						ctx.clearRect(0,0,width,height);

						for(var i = 0; i < cfg.edges.length; i++) {
							var from = cfg.edges[i].from;
							var to = cfg.edges[i].to;
						}

						if(edges !== null && boxes !== null) {
							for(var i = 0; i < graph.edges.length; i++) {
								var from = graph.boxes[graph.edges[i].from];
								var to = graph.boxes[graph.edges[i].to];

								ctx.beginPath();
								ctx.moveTo(Math.max(1,from.x + from.width / 2),Math.max(1,from.y + from.height / 2));
								ctx.lineTo(Math.max(1,to.x + to.width / 2),Math.max(1,to.y + to.height / 2));
								ctx.stroke();
							}
						}
					}

					onEdgesChanged: requestPaint();

					MouseArea {
						anchors.fill: parent;
						drag.target: parent
						drag.axis: Drag.XAndYAxis
					}
				}

				anchors.fill: parent
				//clip: true;

				property string selection: "";
				property var bblockList: null;

				onSelectionChanged: {
					var cfg = eval(Panopticon.functionCfg(selection));
					var func = eval(Panopticon.functionInfo(selection));
					cfg.type = "rankingSimplex";
					cfg.widths = {};
					cfg.heights = {};

					cfg.head = "bb" + func.start.toString();

					if(cflow_graph.item.bblockList != null) {
						for (var i in bblockList) {
							if(bblockList.hasOwnProperty(i)) {
								bblockList[i].visible = false;
								bblockList[i].destroy();
							}
						}
					}

					bblockList = {};
					for(var i = 0; i < cfg.nodes.length; i++) {
						var node = cfg.nodes[i];
						var c = {"contents":cfg.contents[node].join("\n"),"color":(node == cfg.head ? "red" : "steelblue")};
						//var c = {"contents":node,"color":(node == cfg.head ? "red" : "steelblue")};
						var obj = bblock.createObject(graph,c);

						obj.visible = false;
						bblockList[node] = obj;
						cfg.widths[node] = obj.width;
						cfg.heights[node] = obj.height;
					}

					if(cfg.nodes.length > 1) {
						layoutTask.sendMessage(cfg);
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

				WorkerScript {
					id: layoutTask
					source: "../sugiyama.js"
					onMessage: {
						//console.log("MS: " + JSON.stringify(messageObject));

						switch(messageObject.type) {
							case "rankingSimplex": {
								simplexTask.sendMessage(messageObject);
								break;
							}
							case "order": {
								simplexTask.sendMessage(messageObject);
								break;
							}
							case "finalize": {
								var boxes = {};
								var nodes = [];

								for(var i = 0; i < messageObject.nodes.length; i++) {
									var node = messageObject.nodes[i];
									var l = messageObject.layout[node];

									nodes.push(node);
									boxes[node] = {"x":l.x,"y":l.y,"width":l.width,"height":l.height};

									if(cflow_graph.item.bblockList[node] !== undefined) {
										cflow_graph.item.bblockList[node].x = l.x;
										cflow_graph.item.bblockList[node].y = l.y;
										cflow_graph.item.bblockList[node].visible = true;
									}
								}

								graph.y = (cflow_graph.item.height - graph.height) / 2;
								graph.x = (cflow_graph.item.width - graph.width) / 2;
								graph.boxes = boxes;
								graph.edges = messageObject.edges;
								//graph.requestPaint();

								//var cfg = eval(Panopticon.functionCfg(cflow_graph.item.selection));
								//routeTask.sendMessage({"boxes":boxes,"nodes":nodes,"edges":messageObject.edges});
								//routeTask.sendMessage({"boxes":boxes,"nodes":nodes,"edges":cfg.edges});

								break;
							}

							default: {
								break;
							}
						}
					}
				}

				WorkerScript {
					id: simplexTask
					source: "../simplex.js"
					onMessage: {
						//console.log("SP: " + JSON.stringify(messageObject));

						switch(messageObject.type) {
							case "rankingSimplex": {
								messageObject.type = "order";
								layoutTask.sendMessage(messageObject);
								break;
							}
							case "order": {
								messageObject.type = "finalize";
								layoutTask.sendMessage(messageObject);
								break;
							}
							default: {
								break;
							}
						}
					}
				}

				WorkerScript {
					id: routeTask
					source: "../route.js"
					onMessage: {
						graph.edges = messageObject;
						graph.requestPaint();
					}
				}
			}
		}
	}
}

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
						height: txt.contentHeight + 10
						width: txt.contentWidth + 10
						color: "steelblue";
						border.width: 1;
						border.color: "black";

						property string contents: "";

						Text {
							id: txt
							x: 5
							y: 5
							text: contents
							font.family: "Monospace"
						}
					}
				}

				clip: true;

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

						for (var k in pos) {
							if(pos.hasOwnProperty(k)) {
								var obj = bblockList[k];

								obj.visible = true;
								obj.x = pos[k].x - obj.width / 2 + 100;
								obj.y = pos[k].y - obj.height / 2 + 100;


								right = Math.max(right,obj.x + obj.width);
								bottom = Math.max(bottom,obj.y + obj.height);
							}
						}

						graph.width = right + 200;
						graph.height = bottom + 200;

						graph.y = (cflow_graph.item.height - graph.height) / 2;
						graph.x = (cflow_graph.item.width - graph.width) / 2;
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
					for(var i = 0; i < cfg.nodes.length; i++) {
						var node = cfg.nodes[i];
						var contents = " ";

						if(cfg.contents[node]) {
							contents = cfg.contents[node].map(function(x) {
								return x.opcode + " " + x.args.join(", ");
							}).join("\n");
						}

						var c = {"contents":contents ,"color":(node == cfg.head ? "red" : "steelblue")};
						//var c = {"contents":node,"color":(node == cfg.head ? "red" : "steelblue")};
						var obj = bblock.createObject(graph,c);

						obj.visible = false;
						bblockList[node] = obj;

						dims[node] = {"width":obj.width,"height":obj.height};
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

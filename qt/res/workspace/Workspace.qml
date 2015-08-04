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
						width: childrenRect.width;
						height: childrenRect.height;
						color: "steelblue";

						property string contents: "";

						Text {
							height: contentHeight
							width: contentWidth
							text: contents
						}
					}
				}

				Item {
					width: childrenRect.width;
					height: childrenRect.height;
					id: graph

					MouseArea {
						anchors.fill: parent;
						drag.target: parent
						drag.axis: Drag.XAndYAxis
					}
				}


				anchors.fill: parent
				clip: true;

				property string selection: "";
				property var bblockList: null;

				onSelectionChanged: {
					var cfg = eval(Panopticon.functionCfg(selection));
					var func = eval(Panopticon.functionInfo(selection));
					cfg.type = "rankingSimplex";

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
						var c = {"contents":cfg.contents[cfg.nodes[i]].join("\n")};
						var obj = bblock.createObject(graph,c);

						obj.visible = false;
						bblockList[cfg.nodes[i]] = obj;
					}

					if(cfg.nodes.length > 1) {
						cfg.head = "bb" + func.start.toString();
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
				}
/*
				onPaint: {
					var ctx = cflow_graph.item.getContext('2d');
					var func = eval(Panopticon.functionInfo(selection));

					if(func !== undefined) {
						ctx.fillText(func.name,cflow_graph.width / 2,cflow_graph.height / 2);
					}
				}*/

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
								for(var i = 0; i < messageObject.nodes.length; i++) {
									var node = messageObject.nodes[i];

									if(cflow_graph.item.bblockList[node] !== undefined) {
										cflow_graph.item.bblockList[node].x = messageObject.lp.x[i];
										cflow_graph.item.bblockList[node].y = messageObject.layout[node].rank * 100;
										cflow_graph.item.bblockList[node].visible = true;
									}
								}

								graph.y = (cflow_graph.item.height - graph.height) / 2;
								graph.x = (cflow_graph.item.width - graph.width) / 2;

								//cflow_graph.item.requestPaint()
								break;
							}
							default: {
								break;
							}
						}
					}
				}
			}
		}
	}
}

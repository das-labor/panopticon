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

			Canvas {
				anchors.fill: parent

				property string selection: "";

				onSelectionChanged: layoutTask.sendMessage(Panopticon.functionCfg(selection));
				onPaint: {
					var ctx = cflow_graph.item.getContext('2d');
					var func = eval(Panopticon.functionInfo(selection));

					ctx.fillText(func.name,cflow_graph.width / 2,cflow_graph.height / 2);
				}

				WorkerScript {
					id: layoutTask
					source: "../sugiyama.js"
					onMessage: {
						console.log("MS: " + JSON.stringify(messageObject));
						cflow_graph.item.requestPaint()
					}
				}
			}
		}
	}
}

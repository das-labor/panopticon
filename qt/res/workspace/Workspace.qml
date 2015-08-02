import QtQuick 2.0
import Panopticon 1.0
import QtQuick.Controls 1.3

Item {
	id: root

	Component.onCompleted: {
		layoutTask.sendMessage({"type":"resize","width":callgraph.width,"height":callgraph.height});
		timer.running = true;

		Panopticon.startedFunction.connect(function(uu) {
			var obj = eval(Panopticon.functionInfo(uu));

			obj.name = "<b>Working</b>";
			for(var i = 0; i < functionModel.count; i++) {
				var node = functionModel.get(i);

				if(node.uuid == obj.uuid) {
					functionModel.set(i,obj);
					return;
				}
			}

			console.error("Error: got startedFunction() signal w/ unknown function " + uu);
		});

		Panopticon.discoveredFunction.connect(function(uu) {
			var obj = eval(Panopticon.functionInfo(uu));
			if(obj.type == "todo") {
				obj.name = "<i>Todo</i>";
			}
			console.log(JSON.stringify(obj));
			functionModel.append(obj);
		});

		Panopticon.finishedFunction.connect(function(uu) {
			var obj = eval(Panopticon.functionInfo(uu));
			for(var i = 0; i < functionModel.count; i++) {
				var node = functionModel.get(i);

				if(node.uuid == obj.uuid) {
					functionModel.set(i,obj);
					layoutTask.sendMessage({"type":"add","item":obj});
					timer.running = true;

					return;
				}
			}
			console.error("Error: got finishedFunction() signal w/ unknown function " + uu);
		});

		Panopticon.start()
	}

	ListModel {
		id: functionModel
	}

	Timer {
		id: timer;
		interval: 0
		running: false;
		onTriggered: layoutTask.sendMessage({"type":"tick"});
	}

	TableView {
		height: root.height
		width: 300

    TableViewColumn {
			role: "name"
			title: "Name"
			width: 100
    }
    TableViewColumn {
			role: "start"
			title: "Offset"
			width: 100
		}/*
itemDelegate: Item {
    Text {
        anchors.verticalCenter: parent.verticalCenter
        color: styleData.textColor
        elide: styleData.elideMode
        text: styleData.value === undefined ? "---" : styleData.value
			}
		}*/
    model: functionModel
	}

	Canvas {
		id: callgraph
		height: root.height
		width: root.width - 300
		x: 300

		onPaint: {
			var ctx = callgraph.getContext('2d');

			ctx.clearRect(0,0,width,height);
			ctx.beginPath();

			for(var i = 0; i < functionModel.count; ++i) {
				var func = functionModel.get(i);

				ctx.moveTo(func.x,func.y);
				ctx.arc(func.x,func.y,10,0,Math.PI * 2,true);
			}

			ctx.stroke();
			ctx.fill();

			ctx.beginPath();

			for(var i = 0; i < functionModel.count; ++i) {
				var from = functionModel.get(i);

				for(var e in from.calls) {
					var edge = from.calls[e];

					for(var j = 0; j < functionModel.count; ++j) {
						var to = functionModel.get(j);

						if(to.uuid == edge) {
							ctx.moveTo(from.x,from.y);
							ctx.lineTo(to.x,to.y);
						}
					}
				}
			}

			ctx.stroke();
		}
	}

	WorkerScript {
			id: layoutTask
			source: "../layout.js"
			onMessage: {
				//console.log("MS: " + JSON.stringify(messageObject));

				if(messageObject.type == "tock") {
					for(var i = 0; i < functionModel.count; i++) {
						var node = functionModel.get(i);

						if(messageObject.nodes[node.uuid] !== undefined) {
							functionModel.setProperty(i,"x",messageObject.nodes[node.uuid].x);
							functionModel.setProperty(i,"y",messageObject.nodes[node.uuid].y);
						}
					}
				}

				timer.running = messageObject.type !== "stop";
				callgraph.requestPaint();
			}
	}
}

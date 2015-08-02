import QtQuick 2.0
import Panopticon 1.0
import QtQuick.Controls 1.3

/*
 * +-------+---------------------+-------+
 * |   S   |                     |   S   |
 * |   i   |                     |   i   |
 * |   d   |                     |   d   |
 * |   e   |      Workspace      |   e   |
 * |       |                     |       |
 * |   B   |                     |   B   |
 * |   a   |                     |   a   |
 * |   r   |                     |   r   |
 * +-------+---------------------+-------+
 */
Item {
	id: root

	Component.onCompleted: {
		console.log(Panopticon.state)
		layoutTask.sendMessage({"type":"update","model":functionModel,"width":callgraph.width,"height":callgraph.height});
		timer.running = true;

		Panopticon.startedFunction.connect(function(pos) {
			console.log("started " + pos);
		});

		Panopticon.discoveredFunction.connect(function(pos) {
			console.log("discovered " + pos);
		});

		Panopticon.finishedFunction.connect(function(id) {
			var obj = Panopticon.functionInfo(id);
			console.log(obj);
			obj = eval(obj);
			layoutTask.sendMessage({"type":"add","item":obj});
		});

		Panopticon.start()
		console.log(Panopticon.state)
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
		}

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
				timer.running = messageObject.type !== "stop";
				callgraph.requestPaint();
			}
	}
}

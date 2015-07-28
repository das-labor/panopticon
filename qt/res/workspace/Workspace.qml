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

		Panopticon.startedFunction.connect(function(pos) {
			console.log("started " + pos);
		});

		Panopticon.discoveredFunction.connect(function(pos) {
			console.log("discovered " + pos);
		});

		Panopticon.finishedFunction.connect(function(pos) {
			console.log("finishid " + pos);
			functionModel.append({"name":"func_" + pos, "start":pos});
		});

		Panopticon.start()
		console.log(Panopticon.state)
	}

	ListModel {
		id: functionModel
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
        width: 200
    }
    model: functionModel
	}
}

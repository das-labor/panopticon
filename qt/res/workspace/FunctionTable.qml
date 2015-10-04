import QtQuick 2.0
import Panopticon 1.0
import QtQuick.Controls 1.3

Item {
	id: root

	signal activated(string uuid);

	property string selection: "";

	onSelectionChanged: {
		functionTable.selection.clear();

		for(var i = 0; i < functionModel.count; i++) {
			var node = functionModel.get(i);

			if(node.uuid == selection) {
				functionTable.selection.select(i);
				return;
			}
		}
	}

	Component.onCompleted: {
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
			functionModel.append(obj);
			functionModel.sort();
		});

		Panopticon.finishedFunction.connect(function(uu) {
			var obj = eval(Panopticon.functionInfo(uu));
			for(var i = 0; i < functionModel.count; i++) {
				var node = functionModel.get(i);

				if(node.uuid == obj.uuid) {
					functionModel.set(i,obj);
					return;
				}
			}
			console.error("Error: got finishedFunction() signal w/ unknown function " + uu);
		});
	}

	ListModel {
		function sortBy(a,b) {
			return parseInt(a.start,10) < parseInt(b.start,10);
		}

		id: functionModel

		function sort() {
			if (count < 2) {
				return;
			}

			var qsort = function(left, right) {
				if (left < right) {
					var pivot = JSON.parse(JSON.stringify(get(right)));
					var i = left - 1;
					var j = right + 1;

					while (true) {
						do {
							j -= 1;
						} while (sortBy(pivot,get(j)));

						do {
							i += 1;
						} while (sortBy(get(i),pivot));

						if (i < j) {
							var t = JSON.parse(JSON.stringify(get(i)));
							set(i,JSON.parse(JSON.stringify(get(j))));
							set(j,t);
						} else {
							break;
						}
					}

					qsort(left,j-1)
					qsort(j+1,right)
				}
			};

			qsort(0,count-1)
		}
	}

	TableView {
		id: functionTable
		anchors.fill: parent

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

		onClicked: {
			root.selection = functionModel.get(row).uuid;
		}

		onActivated: {
			root.activated(functionModel.get(row).uuid);
		}
	}
}

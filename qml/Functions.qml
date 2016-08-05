pragma Singleton

import QtQuick 2.0
import QtQml.Models 2.1
import Panopticon 1.0

ListModel {
	id: model
	dynamicRoles: true

	signal added(int row)
	signal changed(int row)
	signal removed(int row)

	function upsert(uu,ev) {
		var _info = Panopticon.functionInfo(uu);
		var info = JSON.parse(_info);

		if(info.status == "ok") {
			var obj = info.payload;

			obj.failed = false;
			obj.empty = false;

			if(info.payload.kind == "function") {
				var _cfg = Panopticon.functionCfg(uu);
				var cfg = JSON.parse(_cfg);

				if(cfg.status == "ok") {
					obj.cfg = cfg.payload;
					obj.empty = obj.cfg.nodes.length == 0
					obj.failed = obj.cfg.nodes.reduce(function(acc,x,i,a) {
									 return acc && x.substr(0,3) === "err";
								 },true);
				} else {
					obj.failed = true;
					console.exception(info.error);
				}
			}

			obj.working = (ev === "started");

			for(var i = 0; i < model.count; i++) {
				var node = model.get(i);

				if(node.uuid == obj.uuid) {
					model.set(i,obj);
					model.changed(i);
					return i;
				}
			}

			model.append(obj);
			model.added(model.count - 1);
			return model.count - 1;
		} else {
			console.exception(info.error);
			return -1;
		}
	}

	Component.onCompleted: {
		Panopticon.onStateChanged.connect(function() {
			switch(Panopticon.state) {
				case "NEW":{
					for(var i = 0; i < model.count; i++) {
						model.removed(i);
					}
					model.clear();
				}
			}
		});

		Panopticon.startedFunction.connect(function(uu) {
			upsert(uu,"started");
		});

		Panopticon.discoveredFunction.connect(function(uu) {
			upsert(uu,"discovered");
		});

		Panopticon.finishedFunction.connect(function(uu) {
			upsert(uu,"finished");
		});

		Panopticon.changedFunction.connect(function(uu) {
			upsert(uu,"changed");
		});
	}
}

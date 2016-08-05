/*
 * Panopticon - A libre disassembler (https://panopticon.re/)
 * Copyright (C) 2016 Kai Michaelis
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

import QtQuick 2.3
import QtQuick.Controls 1.2

import Panopticon 1.0

Action {
	property var window: null
	property var fileBrowser: null;
	property var errorPopup: null;
	property var targetPopup: null;

	function displayError(msg) {
		window.enabled = false;
		try {
			errorPopup.createObject(window).displayMessage(msg);
		} catch(e) {
			window.enabled = true;
			throw e;
		}
		window.enabled = true;
	}

	id: openAction
	text: "&Open"
	shortcut: StandardKey.Open
	iconName: "document-open"
	tooltip: "Disassemble a new file"
	enabled: window.enabled && fileBrowser.status == Component.Ready && errorPopup.status == Component.Ready
	onTriggered: {
		window.enabled = false;

		try {
			if(Panopticon.state == "DIRTY") {
				var res = JSON.parse(Panopticon.snapshotProject(Panopticon.savePath));
				if(res.status == "err") {
					displayError(res.error);
				}
			}

			var fb = fileBrowser.createObject(window);
			var code = fb.readFile();

			if(code == 0) {
				var res = JSON.parse(Panopticon.fileDetails(fb.selectedFile));

				if(res.status == "ok") {
					var req = {
						"kind": res.payload.format,
						"path": fb.selectedFile
					};

					console.log(Panopticon.request());
					var res = JSON.parse(Panopticon.request());
					if(res.status == "ok" && res.payload == null) {
						Panopticon.setRequest(JSON.stringify(req));
					} else {
						window.serveRequest(req);
					}
				} else {
					displayError(res.error);
				}
			}
			window.enabled = true;
		} catch(e) {
			window.enabled = true;
			throw e;
		}
	}
}

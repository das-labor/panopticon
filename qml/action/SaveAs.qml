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

	id: saveAction
	text: "&Save"
	shortcut: StandardKey.Save
	iconName: "document-save"
	enabled: Panopticon.state != "NEW"
	onTriggered: {
		window.enabled = false;

		try {
			var fb = fileBrowser.createObject(window);
			var code = fb.writeFile();

			if(code == 0) {
				console.log("call snapshot func");
				var _res = Panopticon.snapshotProject(fb.selectedFile);
				console.log(_res);
				var res = JSON.parse(_res);

				if(res.status == "err") {
					displayError(res.error);
				}
			}
			window.enabled = true;
		} catch(e) {
			window.enabled = true;
			throw e
		}
	}
}

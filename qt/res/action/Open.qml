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

import QtQuick.Controls 1.1
import QtQuick 2.5
import Panopticon 1.0

Action {
	property var window: null
	property var fileBrowser: Qt.createComponent("../popup/FileBrowser.qml");
	property var errorPopup: Qt.createComponent("../popup/ErrorPopup.qml");

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

			console.log("code: " + JSON.stringify(code));
			console.log("open: " + fb.selectedFile);
			if(code == 0) {
				var _res = Panopticon.fileDetails(fb.selectedFile);
				console.log(_res);
				var res = JSON.parse(_res);

				if(res.status == "ok") {
					switch(res.payload.format) {
						case "raw": {
							if(targetSelection.show() == 1) {
								var res = Panopticon.createRawProject(
									fb.selectedFile,
									targetSelection.target,
									targetSelection.base,
									targetSelection.entry_point);

									if(res.status == "err") {
										displayError(res.error);
									}
								}
								break;
							}
							case "elf": {
								var res = Panopticon.createElfProject(fb.selectedFile)
								if(res.status == "err") {
									displayError(res.error);
								}
								break;
							}
							case "pe": {
								var res = Panopticon.createPeProject(fb.selectedFile)
								if(res.status == "err") {
									displayError(res.error);
								}
								break;
							}
							case "panop": {
								var res = Panopticon.openProject(fb.selectedFile)
								if(res.status == "err") {
									displayError(res.error);
								}
								break;
							}
							default: {
								displayError("Internal error: Unknown format");
								break;
							}
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

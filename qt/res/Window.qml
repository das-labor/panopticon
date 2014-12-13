/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Kai Michaelis
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

import QtQuick.Controls 1.0
import QtQuick.Dialogs 1.0
import QtQuick 2.0
import Panopticon 1.0

ApplicationWindow {
	id: mainWindow
	title: "Panopticon"
	height: 1000
	width: 1000

	Loader {
		focus: true
		id: loader
		anchors.fill: parent
	}

	Component.onCompleted: {
		if(Panopticon.session) {
			loader.setSource("workspace/Workspace.qml",{ "session": Panopticon.session })
		} else {
			loader.setSource("wizard/Main.qml")
		}
	}
}

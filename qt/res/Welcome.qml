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

import QtQuick 2.0

Item {
	id: root
	anchors.fill: parent

	Rectangle {
		border.color: "red"
		anchors.fill: parent
	}

	Column {
		spacing: 50
		anchors.top: parent.top

		Option {
			icon: "blue"
			title: "New session"
			description: "Pick a file and analyze it in a new session."

			MouseArea {
				anchors.fill: parent

				onClicked: {
					info.x = 100
				}
			}
		}

		Option {
			icon: "green"
			title: "Continue session"
			description: "Open a session saved previously and continue analysis."
		}

		Option {
			icon: "red"
			title: "Quit Panopticon"
			description: "Close all open sessions and exit the application."
		}
	}

	Rectangle {
		x: { parent.width - 400 }
		y: { parent.height - 400 }
		height: 400
		width: 400
		z: -1

		color: "gray"
	}

	Rectangle {
		id: info
		x: { root.x + root.width }
		y: 0

		height: 200; width: 200
		color: "gray"

		Behavior on x { NumberAnimation { duration: 1000; easing.type: Easing.OutQuad } }
	}
}

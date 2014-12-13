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
	property var icon: null
	property string title: "to title"
	property string description: "no description"

	signal activated()

	height: childrenRect.height
	width: childrenRect.width

	MouseArea {
		anchors.fill: rootItem
		hoverEnabled: true

		onEntered: rootItem.state = "HOVER"
		onExited: rootItem.state = "DEFAULT"
		onPressed: { activated() }
	}

	Rectangle {
		id: rootItem
		height: childrenRect.height + 20
		width: childrenRect.width + 20
		border.width: 2
		state: "DEFAULT"

		states: [
			State {
				name: "DEFAULT"
        PropertyChanges { target: rootItem.border; color: "#ffffff" }
			},
			State {
				name: "HOVER"
        PropertyChanges { target: rootItem.border; color: "#ff0000"}
			}
		]

		transitions: [
			Transition {
				from: "DEFAULT"
				to: "HOVER"

				ColorAnimation {
					target: rootItem.border
					duration: 300
					easing.type: Easing.OutQuad
				}
			},
			Transition {
				from: "HOVER"
				to: "DEFAULT"

				ColorAnimation {
					target: rootItem.border
					duration: 300
					easing.type: Easing.OutQuad
				}
			}
		]

		Rectangle {
			id: iconItem
			x: 10; y: 10
			width: 90; height: 90
			color: icon
		}

		Text {
			id: titleItem
			anchors.left: iconItem.right
			anchors.leftMargin: 20
			y: 20

			text: title
			font { pointSize: 12; family: "Sans"; weight: Font.Bold }
		}

		Text {
			anchors.left: iconItem.right
			anchors.leftMargin: 20
			anchors.top: titleItem.bottom
			anchors.topMargin: 20

			text: description
			font { pointSize: 8; family: "Sans" }
		}
	}
}

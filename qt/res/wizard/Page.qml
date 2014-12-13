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
	id: rootItem
	property string primaryTitle: ""
	property string secondaryTitle: ""
	property string primaryAction: "Next"

	signal primary

	Rectangle {
		id: titleItem
		x: parent.width - childrenRect.width

		Text {
			id: primaryTitleItem
			font {
				family: "Sans"
				pointSize: 22
			}
			text: primaryTitle
		}

		Text {
			id: secondaryTitleItem
			anchors.left: primaryTitleItem.right
			anchors.leftMargin: 10
			anchors.baseline: primaryTitleItem.baseline

			font {
				family: "Sans"
				pointSize: 14
			}
			color: "#444"
			text: secondaryTitle
		}
	}

	Item {
		id: contentItem
		anchors.top: titleItem.top
		width: rootItem.width
		height: primaryActionItem.top - titleItem.bottom

		Rectangle {
			anchors.fill: parent
			color: "red"
		}
	}

	Item {
		id: primaryActionItem
		width: childrenRect.width
		height: childrenRect.height

		MouseArea {
			anchors.fill: parent

			onClicked: {
				primary()
			}
		}

		Rectangle {
			id: iconItem
			x: 8; y: 8
			height: 35
			width: 35
			color: "green"
		}

		Text {
			anchors.left: iconItem.right
			anchors.leftMargin: 10
			anchors.top: iconItem.top

			text: primaryAction
			font { pointSize: 14; family: "Sans" }
		}
	}
}

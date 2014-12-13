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
import Panopticon 1.0
import "../workspace"
import "../"
import Qt.labs.settings 1.0

Item {
	signal back()

	Loader {
		property variant session: null

		id: loader
		height: parent.height
		width: parent.width
		anchors.left: root.right
	}

	Page {
		id: root
		anchors.fill: parent
		primaryTitle: "Continue Session"
		secondaryTitle: "Recent sessions"
		primaryAction: "Back"

		Behavior on x {
			NumberAnimation { duration: 300 }
		}

		Settings {
			id: settings
			property string recent: ""
		}

		onPrimary: {
			back()
		}

		Item {
			height: childrenRect.height
			width: childrenRect.width
			anchors.centerIn: parent

			Column {
				spacing: 30

				Repeater {
					model: settings.recent.split(",").filter(function(a) { return a.length > 0 })
					delegate: Item {
						height: 80
						width: 600
						clip: true

						Rectangle {
							color: "green"
							anchors.fill: parent
						}

						Text {
							width: 580
							anchors.centerIn: parent
							text: modelData
						}

						MouseArea {
							property variant sess: null

							id: mouseArea
							anchors.fill: parent

							Component {
								id: comp
								Workspace { session: mouseArea.sess }
							}

							onPressed: {
								root.anchors.fill = undefined
								root.x = -1 * root.width

								console.log(modelData)
								mouseArea.sess = Panopticon.openSession(modelData)
								loader.sourceComponent = comp
							}
						}
					}
				}
			}
		}
	}
}

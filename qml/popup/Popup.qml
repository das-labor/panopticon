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
import QtQuick.Layouts 1.1
import QtTest 1.1
import QtGraphicalEffects 1.0

import Panopticon 1.0

Item {
	property var component: null
	property string title: "(unset)"
	property var buttons: [{"title":"Ok","enabled":true}]

	signal done(int ret)

	function show() {
		root.opacity = 1
		spy.wait(9999999999999999999999999)
		root.opacity = 0

		return spy.signalArguments[0][0]
	}

	id: root
	anchors.centerIn: parent
	z: 1
	visible: opacity > 0
	opacity: 0

	Behavior on opacity {
		SmoothedAnimation { velocity: 6 }
	}

	RectangularGlow {
		anchors.fill: border
		glowRadius: 4
		spread: 0.01
		color: "gray"
		cornerRadius: 3
	}

	Rectangle {
		id: border
		anchors.centerIn: parent
		width: layout.width + 10
		height: layout.height + 10
		color: "white"
		border.width: 0
		border.color: "gray"
		radius: 2
	}

	SignalSpy {
		id: spy
		signalName: "done"
		target: root
	}

	GridLayout {
		id: layout
		anchors.centerIn: parent
		Label {
			id: titleLabel
			font.bold: true
			Layout.column: 0
			Layout.row: 0
			Layout.fillWidth: true
			text: root.title
			horizontalAlignment: Text.AlignHCenter
		}

		Text {
			Layout.column: 1
			Layout.row: 0
			width: titleLabel.height
			height: titleLabel.height
			verticalAlignment: Text.AlignVCenter;
			horizontalAlignment: Text.AlignHCenter;
			font.family: "FontAwesome"
			font.pixelSize: 16
			text: "\uf00d"

			MouseArea {
				anchors.fill: parent
				onClicked: root.done(-1)
			}
		}

		Loader {
			Layout.row: 1
			Layout.column: 0
			Layout.columnSpan: 2
			sourceComponent: root.component
		}

		Row {
			Layout.row: 2
			Layout.column: 0
			Layout.columnSpan: 2
			Layout.alignment: Qt.AlignRight
			spacing: 10

			Repeater {
				model: root.buttons.length

				Button {
					text: root.buttons[index].title
					enabled: root.buttons[index].enabled
					menu: {
							if(root.buttons[index].confirm !== undefined) {
								confirmMenu.createObject(parent,{});
							} else {
								null
							}
						}

					Component {
						id: confirmMenu
						Menu {
							MenuItem {
								text: (root.buttons[index].confirm !== undefined ? root.buttons[index].confirm : "")
								onTriggered: {
									root.done(index)
								}
							}
						}
					}

					onClicked: {
						root.done(index)
					}
				}
			}
		}
	}
}

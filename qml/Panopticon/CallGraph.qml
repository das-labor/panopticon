/*
 * Panopticon - A libre disassembler (https://panopticon.re/)
 * Copyright (C) 2014-2016 Kai Michaelis
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
import QtQuick.Controls 1.2 as Ctrl

Item {
	property string programUuid: ""

	onProgramUuidChanged: {
		Panopticon.display_call_graph_for(programUuid);
	}

	Rectangle {
		anchors.fill: parent
		color: "#fafafa"
	}

	Canvas {
		id: callgraphEdges
		anchors.centerIn: parent

		ListView {
			id: callGraphEdgeList
			model: Panopticon.callGraphEdges
			delegate: Item {
				property real from_x: Math.cos(model.from_phi) * model.from_r
				property real from_y: Math.sin(model.from_phi) * model.from_r
				property real from_phi: model.from_phi
				property real from_r: model.from_r

				property real to_x: Math.cos(model.to_phi) * model.to_r
				property real to_y: Math.sin(model.to_phi) * model.to_r
				property real to_phi: model.to_phi
				property real to_r: model.to_r
			}
		}

		onPaint: {
			var ctx = callgraphEdges.getContext('2d');

			ctx.clearRect(0,0,callgraphEdges.width,callgraphEdges.height);

			if(callGraphEdgeList.count > 0) {
				callGraphEdgeList.currentIndex = 0

				var offset = callgraphEdges.width / 2;
				while (true) {
					var edge = callGraphEdgeList.currentItem;
					var phi = edge.from_phi;
					var r = edge.from_r;

					ctx.lineWidth = (edge.marked ? 4.0 : 1.5);
					ctx.beginPath();

					if( r === 0 ) {
						// draw edge from the center as straight lines
						ctx.moveTo(Math.cos(phi) * r + offset,Math.sin(phi) * r + offset);
						ctx.lineTo(Math.cos(edge.to_phi) * edge.to_r + offset,
						Math.sin(edge.to_phi) * edge.to_r + offset);
					} else {
						// otherwise draw a spiral
						var dist_phi = edge.to_phi - edge.from_phi;
						var dist_r = edge.to_r - edge.from_r;
						var step_r = dist_r / 100;

						// we take the shortest path
						if(dist_phi > Math.PI) {
							dist_phi -= 2*Math.PI;
							var step_phi = dist_phi / 100;
						} else if (dist_phi < -Math.PI) {
							dist_phi += 2*Math.PI;
							var step_phi = dist_phi / 100;
						} else {
							var step_phi = dist_phi / 100;
						}

						ctx.moveTo(Math.cos(phi) * r + offset,Math.sin(phi) * r + offset);
						for (var i = 0; i < 100; i++) {
							r += step_r;
							phi += step_phi;
							ctx.lineTo(Math.cos(phi) * r + offset,Math.sin(phi) * r + offset);
						}
					}
					ctx.stroke();

					if (callGraphEdgeList.currentIndex + 1 < callGraphEdgeList.count) {
						callGraphEdgeList.incrementCurrentIndex();
					} else {
						break;
					}
				}
			}
		}

		Component.onCompleted: {
			Panopticon.call_graph_edges_changed.connect(function() {
				var max_radius = 0;

				if(callGraphEdgeList.count > 0) {
					callGraphEdgeList.currentIndex = 0
					while (true) {
						var edge = callGraphEdgeList.currentItem;

						max_radius = Math.max(max_radius,edge.from_r);
						max_radius = Math.max(max_radius,edge.to_r);

						if (callGraphEdgeList.currentIndex + 1 < callGraphEdgeList.count) {
							callGraphEdgeList.incrementCurrentIndex();
						} else {
							break;
						}
					}

					callgraphEdges.height = max_radius * 2 + 100
					callgraphEdges.width = max_radius * 2 + 100
					callgraphEdges.requestPaint();
				}
			});
		}
	}

	Item {
		id: callgraphNodes
		anchors.centerIn: parent
		Repeater {
			model: Panopticon.callGraphNodes
			delegate: Item {
				property real largeRadius: 13
				property real smallRadius: 8

				x: Math.cos(model.phi) * model.r - width / 2
				y: Math.sin(model.phi) * model.r - height / 2
				width: (model.marked ? 2*largeRadius : 2*smallRadius)
				height: (model.marked ? 2*largeRadius : 2*smallRadius)

				Rectangle {
					anchors.fill: parent
					z: 1
					color: mouseArea.containsMouse ? "#f0f0f0" : "#ffffff"
					radius: (model.marked ? largeRadius : smallRadius)
					border {
						width: (model.marked ? 4.0 : 1.5)
						color: "black"
					}
				}

				Text {
					anchors.centerIn: parent
					z: 1
					text: model.name
				}

				Canvas {
					id: nameBubble
					opacity: .8
					x: (model.marked ? largeRadius - 2 : smallRadius - 1)
					y: -45 + (model.marked ? largeRadius + 2 : smallRadius + 1)
					width: 100; height: 100
					onPaint: {
						var ctx = nameBubble.getContext('2d');

						ctx.clearRect(0,0,nameBubble.width,nameBubble.height);

						ctx.strokeStyle = "black";

						if(model.marked) {
							ctx.fillStyle = "black";
						} else {
							ctx.fillStyle = "white";
						}

						ctx.beginPath();
						roundedRect(ctx, 23, 0, 50, 45, 6, 0, 45);
						ctx.fill();
						ctx.stroke();
					}

					function roundedRect(ctx, x, y, width, height, radius, tip_x, tip_y) {
						ctx.moveTo(x, y + radius);

						// tip
						ctx.lineTo(x, y + height / 2);
						ctx.lineTo(tip_x, tip_y);
						ctx.lineTo(x, y + height - radius - 3);

						ctx.lineTo(x, y + height - radius);
						ctx.arcTo(x, y + height, x + radius, y + height, radius);
						ctx.lineTo(x + width - radius, y + height);
						ctx.arcTo(x + width, y + height, x + width, y + height-radius, radius);
						ctx.lineTo(x + width, y + radius);
						ctx.arcTo(x + width, y, x + width - radius, y, radius);
						ctx.lineTo(x + radius, y);
						ctx.arcTo(x, y, x, y + radius, radius);
					}
				}

				MouseArea {
					id: mouseArea
					anchors.fill: parent
					hoverEnabled: true

					onClicked: {
						Panopticon.toggle_call_graph_item(model.name);
					}
				}
			}
		}
	}

	Text {
		visible: callgraphNodes.children.length === 0
		anchors.centerIn: parent
		text: "Call Graph"
	}
}

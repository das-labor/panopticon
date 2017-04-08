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
import QtGraphicalEffects 1.0
import QtQuick.Layouts 1.1

MouseArea {
	property string functionUuid: ""

	signal showControlFlowGraph(string uuid)

	onFunctionUuidChanged: {
		Panopticon.display_control_flow_for(functionUuid);
	}

	id: controlflow
	clip: true
	hoverEnabled: true
	anchors.left: bar.right
	anchors.right: parent.right
	anchors.top: parent.top
	anchors.bottom: parent.bottom

	cursorShape: {
		if(pressed && containsMouse) {
			return Qt.ClosedHandCursor;
		} else {
			return Qt.OpenHandCursor;
		}
	}
	onWheel: {
		if(wheel.modifiers & Qt.ControlModifier) {
			if(wheel.angleDelta.y < 0) {
				controlFlowScale.xScale *= 0.95
				controlFlowScale.yScale *= 0.95
			} else {
				controlFlowScale.xScale *= 1.05
				controlFlowScale.yScale *= 1.05
			}

			controlFlowScale.xScale = Math.min(controlFlowScale.xScale,1);
			controlFlowScale.yScale = Math.min(controlFlowScale.yScale,1);
			controlFlowScale.xScale = Math.max(controlFlowScale.xScale,0.000001);
			controlFlowScale.yScale = Math.max(controlFlowScale.yScale,0.000001);
			updateFollower();
		} else if(wheel.modifiers === 0) {
			controlFlowRoot.x += Math.max(Math.min(wheel.angleDelta.x,200),-200) * 1 / controlFlowScale.xScale;
			controlFlowRoot.y += Math.max(Math.min(wheel.angleDelta.y,200),-200) * 1 / controlFlowScale.yScale;
			updateFollower();
		} else {
			wheel.accepted = false
		}
	}

	property int fixX: 0
	property int fixY: 0
	signal updateFollower;

	onPositionChanged: {
		if(mouse.buttons & Qt.LeftButton != 0) {
			controlFlowRoot.x += (mouse.x - fixX) * 1 / controlFlowScale.xScale;
			controlFlowRoot.y += (mouse.y - fixY) * 1 / controlFlowScale.yScale;
			updateFollower();
		} else {
			wheel.accepted = false
		}
		fixX = mouse.x;
		fixY = mouse.y;
	}

	Rectangle {
		anchors.fill: parent
		color: "#fafafa"
	}

	Item {
		id: controlFlowNodeFollower
		x: 0; y: 0; z: 1

		Repeater {
			model: Panopticon.controlFlowNodes
			delegate: Rectangle {
				id: follower

				x: controlflow.width - 25 - follower.width
				y: 25
				width: 36
				height: 36
				radius: height / 2
				color: "#ffffff"
				visible: model.is_entry
				opacity: 0.
				transform: Rotation {
					id: rotation

					origin {
						x: follower.width / 2
						y: follower.height / 2
					}
					angle: 0
				}

				Behavior on opacity { NumberAnimation { duration: 300 } }

				Image {
					anchors.centerIn: parent
					source: "../icons/home.svg"
					fillMode: Image.PreserveAspectFit
					width: 22
					mipmap: true
				}

				MouseArea {
					visible: follower.opacity > .5;
					anchors.fill: parent
					onClicked: {
						controlFlowRoot.centerEntryPoint()
						updateFollower()
					}
				}

				Component.onCompleted: {
					function updatePos() {
						// XXX
						if(model !== undefined) {
							var global_x = ((model.x - controlFlowScale.origin.x) * controlFlowScale.xScale)
													 + controlFlowScale.origin.x + controlFlowRoot.x;
							var global_y = ((model.y - controlFlowScale.origin.y) * controlFlowScale.yScale)
													 + controlFlowScale.origin.y + controlFlowRoot.y;

							var local_x = Math.max(0,Math.min(controlflow.width,global_x))
							var local_y = Math.max(0,Math.min(controlflow.height,global_y))

							var dx = global_x - local_x;
							var dy = global_y - local_y;
							var rad = Math.atan2(dy,dx);

							//x = local_x
							//y = local_y
							//rotation.angle = rad * (180 / Math.PI);
							if(Math.abs(dx) >= 1 || Math.abs(dy) >= 1) {
								follower.opacity = 1;
							} else {
								follower.opacity = 0.;
							}
						}
					}
					if(model.is_entry) {
						controlflow.onUpdateFollower.connect(updatePos);
						updatePos();
					}
				}
			}
		}
	}

	Item {
		id: controlFlowRoot
		transform: Scale {
			id: controlFlowScale
			xScale: 1
			yScale: 1
			origin {
				x: controlFlowRoot.width / 2 - controlFlowRoot.x
				y: controlFlowRoot.height / 2 - controlFlowRoot.y
			}
		}

		property real entryNodeCenterX: 0
		property real entryNodeCenterY: 0

		function centerEntryPoint() {
			controlFlowScale.xScale = 1;
			controlFlowScale.yScale = 1;

			var unscaled_x = -1*entryNodeCenterX + controlflow.width / 2
			var unscaled_y = -1*entryNodeCenterY + controlflow.height / 3

			controlFlowRoot.x = unscaled_x;
			controlFlowRoot.y = unscaled_y;
		}

		Canvas {
			id: controlFlowEdges

			ListView {
				id: controlFlowEdgeList
				model: Panopticon.controlFlowEdges
				delegate: Item {
					property var path_x: JSON.parse(model.path_x)
					property var path_y: JSON.parse(model.path_y)
					property var start_x: model.start_x
					property var start_y: model.start_y
					property var end_x: model.end_x
					property var end_y: model.end_y
					property string kind: model.kind
					property string label: model.label
				}
			}

			onPaint: {
				var ctx = controlFlowEdges.getContext('2d');

				ctx.lineWidth = 2.0;
				ctx.clearRect(0,0,controlFlowEdges.width,controlFlowEdges.height);

				if(controlFlowEdgeList.count > 0) {
					controlFlowEdgeList.currentIndex = 0

					while (true) {
						try {
							var edge = controlFlowEdgeList.currentItem;
							var first_x = edge.path_x[0];
							var first_y = edge.path_y[0];
							var prev_x = first_x;
							var prev_y = first_y;

							switch(edge.kind) {
								case "branch":
								ctx.strokeStyle = "green";
								ctx.fillStyle = "green";
								break;

								case "branch-backedge":
								ctx.strokeStyle = "green";
								ctx.fillStyle = "green";
								break;

								case "fallthru":
								ctx.strokeStyle = "red";
								ctx.fillStyle = "red";
								break;

								case "jump":
								default:
								ctx.strokeStyle = "black";
								ctx.fillStyle = "black";
								break;

								case "jump-backedge":
								ctx.strokeStyle = "black";
								ctx.fillStyle = "black";
								break;
							}

							ctx.beginPath();
							ctx.moveTo(first_x,first_y);

							for(var i = 1; i < edge.path_x.length-1; i += 1) {
								var cur_x = edge.path_x[i];
								var cur_y = edge.path_y[i];
								var next_x = edge.path_x[i+1];
								var next_y = edge.path_y[i+1];

								function lerp(a,b,p) {
									return a + p * (b - a);
								}

								var a_x = lerp(cur_x,prev_x,.2);
								var a_y = lerp(cur_y,prev_y,.2);
								var b_x = lerp(cur_x,next_x,.2);
								var b_y = lerp(cur_y,next_y,.2);

								ctx.lineTo(a_x,a_y);
								ctx.bezierCurveTo(cur_x,cur_y,cur_x,cur_y,b_x,b_y);

								prev_x = cur_x;
								prev_y = cur_y;
							}

							var last_x = edge.path_x[edge.path_x.length-1];
							var last_y = edge.path_y[edge.path_y.length-1];

							ctx.lineTo(last_x,last_y);
							ctx.stroke();

							draw_arrow_head(edge.end_x,edge.end_y,ctx);

							if (controlFlowEdgeList.currentIndex + 1 < controlFlowEdgeList.count) {
								controlFlowEdgeList.incrementCurrentIndex();
							} else {
								break;
							}
						} catch(e) {
							console.exception(e);
						}
					}
				}
			}

			function draw_arrow_head(x,y,ctx) {
				var dim = 10;

				ctx.save();
				ctx.lineWidth = 1.0;
				ctx.translate(x,y);
				ctx.rotate(Math.PI);
				ctx.beginPath();
				ctx.moveTo(0,dim / -2);
				ctx.lineTo(dim / 2,dim / 2);
				ctx.lineTo(0,dim / 3);
				ctx.lineTo(dim / -2,dim / 2);
				ctx.lineTo(0,dim / -2);
				ctx.stroke();
				ctx.fill();
				ctx.restore();
			}

			Component.onCompleted: {
				Panopticon.control_flow_edges_changed.connect(function() {
					var min_x = Infinity;
					var min_y = Infinity;
					var max_x = -Infinity;
					var max_y = -Infinity;

					if(controlFlowEdgeList.count > 0) {
						controlFlowEdgeList.currentIndex = 0
						while (true) {
							var edge = controlFlowEdgeList.currentItem;

							for(var i = 0; i < edge.path_x.length; i += 1) {
								min_x = Math.min(min_x,edge.path_x[i]);
								min_y = Math.min(min_y,edge.path_y[i]);
								max_x = Math.max(max_x,edge.path_x[i]);
								max_y = Math.max(max_y,edge.path_y[i]);
							}

							if (controlFlowEdgeList.currentIndex + 1 < controlFlowEdgeList.count) {
								controlFlowEdgeList.incrementCurrentIndex();
							} else {
								break;
							}
						}

						controlFlowEdges.height = max_y - min_y + 100;
						controlFlowEdges.width = max_x - min_x + 100
						controlFlowEdges.requestPaint();
					}
				});
			}
		}

		Item {
			id: controlFlowNodes
			x: 0; y: 0
			Repeater {
				model: Panopticon.controlFlowNodes
				delegate: BasicBlock {
					id: basicBlock
					x: model.x - width / 2
					y: model.y - height / 2
					code: JSON.parse(model.contents)
					nodeId: model.id

					onStartComment: {
						var pnt = mapToItem(controlflow,x,y);
						overlay.open(pnt.x,pnt.y,address,comment);
					}

					onDisplayPreview: {
						var mapped = mapToItem(controlflow,bb.x,bb.y,bb.width,bb.height);
						preview.open(mapped);
					}

					onShowControlFlowGraph: {
						controlflow.showControlFlowGraph(uuid)
					}

					Component.onCompleted: {
						Panopticon.controlFlowNodes.dataChanged.connect(function() {
							basicBlock.code = JSON.parse(model.contents)
						});

						if(model.is_entry) {
							controlFlowRoot.entryNodeCenterX = x + width / 2;
							controlFlowRoot.entryNodeCenterY = y + height / 2;
							controlFlowRoot.centerEntryPoint()
						}
					}
				}
			}
		}

		Item {
			id: controlFlowEdgeLabels
			anchors.fill: controlFlowEdges

			Repeater {
				model: Panopticon.controlFlowEdges
				delegate: Rectangle {
					x: model.start_x - width / 2
					y: model.start_y - height / 2
					visible: {
						model.kind == "fallthru" ||
						model.kind == "fallthru-backedge" ||
						model.kind == "branch" ||
						model.kind == "branch-backedge"
					}
					width: label.width + 0
					height: label.height + 0
					color: "#fafafa"

					Ctrl.Label {
						id: label
						anchors.centerIn: parent
						width: contentWidth
						height: contentHeight
						text: model.label
						font {
							family: "Source Code Pro"; pointSize: 13;
						}
						color: {
							if(model.kind == "fallthru" || model.kind == "fallthru-backedge") {
								return "red";
							} else if(model.kind == "branch" || model.kind == "branch-backedge") {
								return "green";
							} else {
								return "black";
							}
						}
					}
				}
			}
		}
	}

	PreviewOverlay {
		id: preview
		visible: false

		onShowControlFlowGraph: {
			controlflow.showControlFlowGraph(uuid)
		}
	}

	CommentOverlay {
		id: overlay
		anchors.fill: parent
		visible: false
	}

	Item {
		x: 20
		anchors.top: parent.top
		anchors.topMargin: 20
		anchors.bottom: parent.bottom
		width: 100

		Column {
			spacing: 20

			Repeater {
				model: Panopticon.tasks
				delegate: Item {
					height: childrenRect.height
					width: childrenRect.width

					Component.onCompleted: {
						Panopticon.tasks.dataChanged.connect(function() {
							cancelLabel.text = model.state == "preparing" || model.state == "running" ? "Cancel" : "Reset";
							titleLabel.text = model.title
						})
					}

					Rectangle {
						id: task

						height: cancelLabel.y + cancelLabel.height + 12
						width: childrenRect.width + 16
						color: "white"
						radius: 2

						Label {
							id: titleLabel

							x: 8
							y: 16
							width: contentWidth
							height: contentHeight
							text: model.title
							font {
								weight: Font.DemiBold
								pointSize: 12
							}
						}

						Label {
							id: descLabel

							anchors.top: titleLabel.bottom
							anchors.topMargin: model.description !== "" ? 8 : 0
							visible: model.description !== ""
							width: model.description !== "" ? contentWidth : 0
							height: model.description !== "" ? contentHeight : 0
							text: model.description
							font {
								pointSize: 12
							}
						}

						Label {
							id: cancelLabel
							anchors.top: descLabel.bottom
							anchors.topMargin: 12
							anchors.right: parent.right
							anchors.rightMargin: 8

							text: model.state == "preparing" || model.state == "running" ? "Cancel" : "Reset";

							font {
								family: "Source Sans Pro"
								pointSize: 12
								underline: mousearea.containsMouse
							}
							color: "#4a95e2"

							MouseArea {
								id: mousearea
								anchors.fill: parent
								hoverEnabled: true
								cursorShape: (containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor)
								onClicked: {
									Panopticon.cancel_task(model.uuid);
								}
							}
						}
					}

					DropShadow {
						anchors.fill: parent
						horizontalOffset: 3
						verticalOffset: 3
						radius: 6.0
						samples: 17
						color: "#80000000"
						source: task
					}
				}
			}
		}
	}
}

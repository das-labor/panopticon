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
import Panopticon 1.0

MouseArea {
	property string functionUuid: ""

	function showControlFlowGraph(uuid) {
    console.log("showControlFlowGraph(): " + uuid);

    controlflow.fixX = 0
    controlflow.fixY = 0
    controlFlowRoot.entryNodeCenterX = 0
    controlFlowRoot.entryNodeCenterY = 0
    controlFlowRoot.x = 0
    controlFlowRoot.y = 0
		controlflow.functionUuid = uuid
		controlFlowRoot.centerEntryPoint()
  }

	function centerEntryPoint() {
		controlFlowRoot.centerEntryPoint()
	}

	id: controlflow
	clip: true
	hoverEnabled: true
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

	Rectangle {
		id: follower

		x: controlflow.width - 25 - follower.width
		y: 25
		width: 36
		height: 36
		radius: height / 2
		color: "#ffffff"
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
				var global_x = ((controlFlowRoot.entryNodeCenterX - controlFlowScale.origin.x) * controlFlowScale.xScale)
				+ controlFlowScale.origin.x + controlFlowRoot.x;
				var global_y = ((controlFlowRoot.entryNodeCenterY - controlFlowScale.origin.y) * controlFlowScale.yScale)
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
			controlflow.onUpdateFollower.connect(updatePos);
			updatePos();
		}
	}

	ControlFlowGraph {
		id: controlFlowRoot
		transform: Scale {
			id: controlFlowScale
			xScale: 1
			yScale: 1
			origin {
				x: controlflow.width / 2 - controlFlowRoot.x
				y: controlflow.height / 2 - controlFlowRoot.y
			}
		}
		uuid: controlflow.functionUuid

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

    delegate: Component {
      Loader {
        sourceComponent: blockIsBlock ? basicBlockComponent : messageBlockComponent

        Component {
          id: messageBlockComponent

          MessageBlock {
            nodeX: blockX
            nodeY: blockY
            nodeValue: blockContents[0]
            z: 0
           }
         }

        Component {
          id: basicBlockComponent

          BasicBlock {
            property bool isEntry: blockIsEntry

            id: basicBlock
            nodeX: blockX
            nodeY: blockY
            z: 0
            code: blockContents
            nodeId: blockId
            uuid: functionUuid

            onVisibleChanged: {
              if(visible && isEntry) {
                controlFlowRoot.entryNodeCenterX = blockX
                controlFlowRoot.entryNodeCenterY = blockY
                controlflow.centerEntryPoint()
              }
            }

            onStartComment: {
              var pnt = basicBlock.mapToItem(controlflow,x,y);
              overlay.open(pnt.x,pnt.y,address,comment);
            }

            onDisplayPreview: {
              var mapped = mapToItem(controlflow,bb.x,bb.y,bb.width,bb.height);
              controlFlowRoot.requestPreview(uuid);
              preview.open(mapped,uuid);
            }

            onShowControlFlowGraph: {
              controlflow.showControlFlowGraph(newUuid)
            }
          }
        }
      }
    }
		edgeDelegate: Component {
			Rectangle {
				id: controlFlowEdgeLabel
				x: edgeHead.x - width / 2
				y: edgeHead.y - height / 2
				visible: {
					edgeKind == "fallthru" ||
					edgeKind == "fallthru-backedge" ||
					edgeKind == "branch" ||
					edgeKind == "branch-backedge"
				}
				width: label.width
				height: label.height
				color: "#fafafa"

				Ctrl.Label {
					id: label
					anchors.centerIn: parent
					width: contentWidth
					height: contentHeight
					text: edgeLabel
					font {
						family: "Source Code Pro"; pointSize: 13;
					}
					color: {
						if(edgeKind == "fallthru" || edgeKind == "fallthru-backedge") {
							return "red";
						} else if(edgeKind == "branch" || edgeKind == "branch-backedge") {
							return "green";
						} else {
							return "black";
						}
					}
				}
			}
		}
	}

	Ctrl.Label {
		anchors.centerIn: parent
		width: 140
		font {
			family: "Source Sans Pro"; pointSize: 20;
		}
		visible: controlFlowRoot.isEmpty && Panopticon.layoutTask === controlflow.functionUuid
		text: "Function is layouting"
		color: "#a2a2a2"
		horizontalAlignment: Text.AlignHCenter
		wrapMode: Text.WordWrap
  }

	Ctrl.Label {
		anchors.centerIn: parent
		width: 140
		font {
			family: "Source Sans Pro"; pointSize: 20;
		}
		visible: controlFlowRoot.isEmpty && Panopticon.layoutTask !== controlflow.functionUuid
		text: "Function is empty"
		color: "#a2a2a2"
		horizontalAlignment: Text.AlignHCenter
		wrapMode: Text.WordWrap
	}

	PreviewOverlay {
		id: preview
		visible: false
		code: controlFlowRoot.preview

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

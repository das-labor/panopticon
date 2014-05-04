import QtQuick 2.1
import Panopticon 1.0

Item {
	property var procedure: null

	Component {
		id: nodeComponent

		Rectangle {
			color: "red"

			ListView {
				model: mnemonics
				delegate: Text {
					text: mnemonic
				}
			}
		}
	}

	Component {
		id: edgeComponent

		// points
	}

	Sugiyama {
		node: nodeComponent
		edge: edgeComponent

		graph: procedure

		onLayoutStart: {}
		onLayoutDone: {}
		onRoutingStart: {}
		onRoutingDone: {}
	}
}

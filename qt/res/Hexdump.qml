import QtQuick 1.0
import Panopticon 1.0

LinearScene {
	Column {
		Repeater {
			model: nodes
			Section { rows: model.modelData.rows; name: model.modelData.name }
		}
	}
}

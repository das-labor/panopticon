import QtQuick 2.0
import QtQuick.Controls 1.0
import Panopticon 1.0

Item {
	id: root

	property variant session: null

	anchors.fill: parent

	Row {
		anchors.fill: parent

		Linear {
			id: lst1
			session: root.session
		}
	}
}

import QtQuick 2.0

Item {
	property string title: "(no name)"
	signal collapse()

	id: root
	height: childrenRect.height
	width: childrenRect.width

	function mouseMoved() {}
	function mousePressed() {}

	Text {
		text: root.title
	}
}

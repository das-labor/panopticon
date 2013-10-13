import QtQuick 1.0
import Panopticon 1.0

/*
 more than 2GB data
 hexdump + ascii
 structures
 mnemonics
 referece arrows
 marker
 selections
 addresses
 collapseble parts
*/

Item {
	ListModel {
		id: testModel
		ListElement { rows: 10; name: "aa" }
		ListElement { rows: 20; name: "aa" }
		ListElement { rows: 14; name: "aa" }
		ListElement { rows: 12; name: "aa" }
		ListElement { rows: 66; name: "aa" }
	}

	Column {

	Repeater {
		model: testModel
		LinearScene {
			width: 100
			height: { 10 * rows }
			model: testModel.get(index)
		}
	}
}
}
/*
LinearScene {
	height: 300
	width: 300

	/*height: childrenRect.height
	width: childrenRect.width

	ListModel {
		id: testModel
		ListElement { y: 0; text: "a" }
		ListElement { y: 10; text: "b" }
		ListElement { y: 20; text: "c" }
		ListElement { y: 30; text: "d" }
		ListElement { y: 40; text: "e" }
		ListElement { y: 50; text: "f" }
		ListElement { y: 60; text: "g" }
		ListElement { y: 70; text: "h" }
		ListElement { y: 80; text: "i" }
		ListElement { y: 90; text: "j" }
		ListElement { y: 100; text: "k" }
	}

	Rectangle {
		anchors.fill: parent
		color: "green"
	}
/*
	MouseArea {
		anchors.fill: parent
		onClicked: {
			var i = 0

			while(i < testModel.count)
			{
				var obj = testModel.get(i)
				testModel.set(i++,{ "y": obj.y + 10, "text": obj.text})
			}
		}
	}

	ListView {
		model: 200000
	//Column {
		//Repeater {
			//model: nodes

			Text {
				text: model.modelData.name
			}
//			Section { rows: model.modelData.rows; name: model.modelData.name }
		}
	//}
}*/

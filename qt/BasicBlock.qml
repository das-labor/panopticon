import QtQuick 1.0

Rectangle
{
	width: 120; height: 120
	color: "#00000000"

	Rectangle
	{
		id: rect
		width: 120; height: 120;
		color: "green"
	}

	states:
	[
		State
		{
			name: ""
			PropertyChanges { target: rect; color: "green" }
		},
		State
		{
			name: "grabbed"
			PropertyChanges { target: rect; color: "red" }
		},
		State
		{
			name: "hidden"
			PropertyChanges { target: rect; visible: false }
		}
	]
	transitions: [ Transition { to: "*"; ColorAnimation { target: rect; duration: 500 } } ]
}

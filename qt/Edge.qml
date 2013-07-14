import QtQuick 1.0
import Panopticon 1.0

Path
{
	id: path
	direct: false
	pen { width: 2; joinStyle: Pen.RoundJoin }
	state: ""
	z: -1

	Image
	{
		source: "head.svg"
		width: 20; height: 20
		id: h
	}
	
	Image
	{
		z: 3
		source: "tail.svg"
		width: 20; height: 20
		id: t
	}

	head: h
	tail: t

	states:
	[
		State
		{
			name: ""
			PropertyChanges
			{
				target: path
				direct: false
				visible: true
				pen { style: Pen.SolidLine; color: "#11111" }
			}
			PropertyChanges { target: h; visible: true }
			PropertyChanges { target: t; visible: true }
		},
		State
		{
			name: "simple"
			PropertyChanges
			{
				target: path
				direct: true
				visible: true
				pen { style: Pen.DashLine; color: "gray" }
			}
			PropertyChanges { target: h; visible: false }
			PropertyChanges { target: t; visible: false }
		},
		State
		{
			name: "hidden"
			PropertyChanges { target: path; visible: false }
		}
	]
}

import QtQuick 1.0
import Panopticon 1.0

Graph
{
	BasicBlock { id: bb0; }
	BasicBlock { id: bb1; }
	/*BasicBlock { id: bb2; }
	BasicBlock { id: bb3; }
	BasicBlock { id: bb4; }
	BasicBlock { id: bb5; }
	BasicBlock { id: bb6; }
	BasicBlock { id: bb7; }
	BasicBlock { id: bb8; }
	BasicBlock { id: bb9; }
	BasicBlock { id: bb10; }
	BasicBlock { id: bb11; }
	BasicBlock { id: bb12; }*/

	Edge { id:e01; from: bb0; to: bb1 }
/*	Edge { id:e02; from: bb0; to: bb2 }
	Edge { id:e13; from: bb1; to: bb3 }
	Edge { id:e34; from: bb3; to: bb4 }
	Edge { id:e35; from: bb3; to: bb5 }
	Edge { id:e56; from: bb5; to: bb6 }
	Edge { id:e67; from: bb6; to: bb7 }
	Edge { id:e57; from: bb5; to: bb7 }
	Edge { id:e78; from: bb7; to: bb8 }
	Edge { id:e81; from: bb8; to: bb1 }
	Edge { id:e29; from: bb2; to: bb9 }
	Edge { id:e910; from: bb9; to: bb10 }
	Edge { id:e110; from: bb1; to: bb10 }
	Edge { id:e1011; from: bb10; to: bb11 }
	Edge { id:e911; from: bb9; to: bb11 }
	Edge { id:e511; from: bb5; to: bb11 }
	Edge { id:e1112; from: bb11; to: bb12 }*/

	nodes: [ bb0, bb1/*, bb2, bb3, bb4, bb5, bb6, bb7, bb8, bb9, bb10, bb11, bb12*/ ]
	paths: [ e01/*, e02, e13, e34, e35, e56, e67, e57, e78, e81, e29, e910, e110, e1011, e911, e511, e1112*/ ]
}

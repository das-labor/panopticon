import QtQuick 2.4
import QtQuick.Controls 1.3 as Ctrl
import QtQuick.Layouts 1.1
import QtQuick.Controls.Styles 1.4 as Style
import Panopticon 1.0

Rectangle {
  property string functionUuid: "";
  signal showControlFlowGraph(string uuid)

  id: root
  color: "white"

  Accessible.name: "Sidebar"
  Accessible.role: Accessible.Pane

  Component.onCompleted: {
    Panopticon.sidebarSortRole = 0
    Panopticon.sidebarSortAscending = true
  }

  onFunctionUuidChanged: {
    for(var row = 0; row < Panopticon.sortedSidebar.rowCount(); ++row) {
      var idx = Panopticon.sortedSidebar.index(row,0);
      var uuid = Panopticon.sortedSidebar.data(idx,0x102);

      if(uuid === functionUuid && !listView.selection.contains(row)) {
        listView.selection.clear();
        listView.selection.select(row);
        listView.positionViewAtRow(row,ListView.Contain);
        return;
      }
    }
  }

  Ctrl.TableView {
    id: listView
    anchors.left: parent.left
    anchors.right: parent.right
    anchors.top: parent.top
    anchors.bottom: parent.bottom

    backgroundVisible: false
    alternatingRowColors: false
    model: Panopticon.sortedSidebar
    frameVisible: false
    horizontalScrollBarPolicy: Qt.ScrollBarAlwaysOff
    sortIndicatorVisible: true
    visible: listView.rowCount > 0

    style: Style.TableViewStyle {
      transientScrollBars: true
      handle: Item {
        implicitWidth: 14
        implicitHeight: 26
        Rectangle {
          color: "#a2a2a2"
          radius: 3
          anchors.fill: parent
          anchors.topMargin: 6
          anchors.leftMargin: 4
          anchors.rightMargin: 4
          anchors.bottomMargin: 6
        }
      }
      scrollBarBackground: Item {
        implicitWidth: 1
        implicitHeight: 26
      }
      incrementControl: Item {}
      decrementControl: Item {}
    }
    rowDelegate: Rectangle {
      height: 24
      color: styleData.selected ? "#a2a2a2" : "transparent"

      Rectangle {
        width: parent.width
        height: 0
        anchors.bottom: parent.bottom
        color: "#ededed"
      }
    }

    itemDelegate: Item {
      onParentChanged: {
        if(parent) {
          anchors.verticalCenter = parent.verticalCenter
        }
      }

      MouseArea {
        anchors.fill: parent
        onDoubleClicked: {
          if(styleData.column === 0) {
            loaderEditor.active = true
          }
        }
        onClicked: {
          listView.selection.clear();
          listView.selection.select(styleData.row);
          mouse.accepted = false
        }

        propagateComposedEvents: true
      }

      Ctrl.Label {
        anchors.fill: parent
        anchors.leftMargin: 5
        anchors.rightMargin: 5

        color: styleData.textColor
        elide: styleData.elideMode
        text: styleData.value
        visible: !loaderEditor.active
        verticalAlignment: Text.AlignVCenter
        font { pointSize: 11; family: "Source Sans Pro" }
      }

      Loader {
        id: loaderEditor
        anchors.fill: parent
        anchors.leftMargin: 5
        anchors.rightMargin: 5

        Connections {
          target: loaderEditor.item
          onAccepted: {
            var idx = Panopticon.sortedSidebar.index(styleData.row,0);
            var uuid = Panopticon.sortedSidebar.data(idx,0x102);
            var title = Panopticon.sortedSidebar.data(idx,0x100);
            var txt = loaderEditor.item.text;

            if(txt !== "" && txt !== title) {
              Panopticon.renameFunction(uuid,txt);
            }
          }
          onEditingFinished: {
            loaderEditor.active = false
          }

        }
        active: false
        sourceComponent: Component {
          id: editor
          Ctrl.TextField {
            id: textinput
            text: styleData.value
            verticalAlignment: Text.AlignVCenter
            font { pointSize: 11; family: "Source Sans Pro" }
            style: Style.TextFieldStyle {
              background: Rectangle {
                anchors.fill: parent
                border {
                  width: 1
                  color: "#157fcc"
                }
                color: "white"
              }
            }

            Component.onCompleted: {
              if(visible) {
                textinput.forceActiveFocus()
                textinput.selectAll()
              }
            }
          }
        }
      }
    }
    headerDelegate: Rectangle {
      implicitHeight: 30
      color: "#f5f5f5"

      Rectangle {
        anchors.right: parent.right
        width: 1
        height: parent.height
        color: "#d8dae4"
      }

      Rectangle {
        anchors.bottom: parent.bottom
        width: parent.width
        height: 1
        color: "#d8dae4"
      }

      Ctrl.Label {
        anchors.fill: parent
        anchors.leftMargin: 5
        anchors.rightMargin: 5

        text: styleData.value
        verticalAlignment: Text.AlignVCenter
        color: "#666"
        font {
          pointSize: 10
          family: "Source Sans Pro"
          weight: Font.Bold
        }
      }

      Image {
        height: 10
        anchors.right: parent.right
        anchors.rightMargin: 10
        anchors.verticalCenter: parent.verticalCenter
        antialiasing: true
        fillMode: Image.PreserveAspectFit
        source: "../icons/chevron-down.svg"
        mipmap: true
        visible: (Panopticon.sidebarSortRole === 0 && styleData.column === 0) || (Panopticon.sidebarSortRole === 0x101 && styleData.column === 1)
        rotation: (!Panopticon.sidebarSortAscending ? 180 : 0)
      }
    }

    Ctrl.TableViewColumn {
      role: "title"
      title: "Name"
      width: 150
    }
    Ctrl.TableViewColumn {
      role: "subtitle"
      title: "Start"
      width: 100
    }

    selection {
      onSelectionChanged: {
        selection.forEach(function(row) {
          var idx = Panopticon.sortedSidebar.index(row,0);
          var uuid = Panopticon.sortedSidebar.data(idx,0x102);

          console.log("display cfg for " + uuid);
          root.showControlFlowGraph(uuid);
        })
      }
    }

    onSortIndicatorColumnChanged: {
      switch(listView.sortIndicatorColumn) {
        case 0:
        default:
        Panopticon.sidebarSortRole = 0;
        break;

        case 1:
        Panopticon.sidebarSortRole = 0x101;
        break;
      }
    }

    onSortIndicatorOrderChanged: {
      Panopticon.sidebarSortAscending = listView.sortIndicatorOrder == Qt.AscendingOrder
    }
  }

  Ctrl.Label {
    anchors.centerIn: parent
    width: 140
    font {
      family: "Source Sans Pro"; pointSize: 20;
    }
    visible: listView.rowCount == 0
    text: "Please open a program"
    color: "#a2a2a2"
    horizontalAlignment: Text.AlignHCenter
    wrapMode: Text.WordWrap
  }

  Rectangle {
    id: divider

    anchors.right: parent.right
    anchors.top: parent.top
    anchors.bottom: parent.bottom
    width: 1
    color: "#d8dae4"
    Accessible.role: Accessible.Separator
  }
}

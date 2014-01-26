#include "selection.hh"

ElementSelection::ElementSelection(quint64 anc_line, quint64 anc_col, quint64 cur_line, quint64 cur_col)
: Selection<ElementPoint>(ElementPoint(anc_line,anc_col),ElementPoint(cur_line,cur_col)) {}

quint64 ElementSelection::anchorLine(void) const { return anchor().first; }
quint64 ElementSelection::anchorColumn(void) const { return anchor().second; }
quint64 ElementSelection::cursorLine(void) const { return cursor().first; }
quint64 ElementSelection::cursorColumn(void) const { return cursor().second; }

quint64 ElementSelection::firstLine(void) const { return minimum().first; }
quint64 ElementSelection::firstColumn(void) const { return minimum().second; }
quint64 ElementSelection::lastLine(void) const { return maximum().first; }
quint64 ElementSelection::lastColumn(void) const { return maximum().second; }

ElementSelectionObject::ElementSelectionObject(QObject *parent) : QObject(parent), ElementSelection(0,0,0,0) {}
ElementSelectionObject::ElementSelectionObject(const ElementSelection &s, QObject *parent) : QObject(parent), ElementSelection(s) {}

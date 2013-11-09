#include <elementselection.hh>

ElementSelection::ElementSelection(quint64 anc_line, quint64 anc_col, quint64 cur_line, quint64 cur_col)
: m_anchorLine(anc_line), m_anchorColumn(anc_col),
	m_cursorLine(cur_line), m_cursorColumn(cur_col)
{
	return;
}

bool ElementSelection::includes(const ElementSelection &s) const
{
	return (s.firstLine() > firstLine() || (s.firstLine() == firstLine() && s.firstColumn() >= firstColumn())) &&
				 (s.lastLine() < lastLine() || (s.lastLine() == lastLine() && s.lastColumn() <= lastColumn()));
}

bool ElementSelection::includes(quint64 column, quint64 line) const
{
	return ((line == firstLine() || line == lastLine()) && column >= firstColumn() && column <= lastColumn()) ||
				 (line > firstLine() && line < lastLine());
}

void ElementSelection::setCursor(quint64 line,quint64 col)
{
	m_cursorLine = line;
	m_cursorColumn = col;
}

quint64 ElementSelection::anchorLine(void) const			{ return m_anchorLine; }
quint64 ElementSelection::anchorColumn(void) const		{ return m_anchorColumn; }

quint64 ElementSelection::cursorLine(void) const			{ return m_cursorLine; }
quint64 ElementSelection::cursorColumn(void) const		{ return m_cursorColumn; }

quint64 ElementSelection::firstLine(void)	 const
{
	if(m_cursorLine < m_anchorLine)
		return m_cursorLine;
	else
		return m_anchorLine;
}

quint64 ElementSelection::firstColumn(void) const
{
	if(m_cursorLine < m_anchorLine ||
		(m_cursorLine == m_anchorLine && m_cursorColumn < m_anchorColumn))
			return m_cursorColumn;
	else
		return m_anchorColumn;
}

quint64 ElementSelection::lastLine(void) const
{
	if(m_cursorLine > m_anchorLine)
		return m_cursorLine;
	else
		return m_anchorLine;
}

quint64 ElementSelection::lastColumn(void) const
{
	if(m_cursorLine > m_anchorLine ||
		(m_cursorLine == m_anchorLine && m_cursorColumn > m_anchorColumn))
			return m_cursorColumn;
	else
		return m_anchorColumn;
}

bool ElementSelection::disjoint(const ElementSelection &sel) const
{
	return (lastLine() < sel.firstLine() || (lastLine() == sel.firstLine() && lastColumn() < sel.firstColumn())) &&
				 (firstLine() < sel.lastLine() || (firstLine() == sel.lastLine() && firstColumn() < sel.lastColumn()));
}

uint qHash(const ElementSelection &s)
{
	return qHash(QString("%1%2%3%4").arg(s.cursorColumn()).arg(s.cursorLine()).arg(s.anchorColumn()).arg(s.anchorLine()));
}

QDebug operator<<(QDebug dbg, const ElementSelection &c)
{
	dbg.nospace() << "[L:" << c.anchorLine() << " C:" << c.anchorColumn() << "] -> [L:" << c.cursorLine() << " C:" << c.cursorColumn() << "]";
	return dbg.space();
}

bool operator==(const ElementSelection &s1,const ElementSelection &s2)
{
	return 	s1.firstColumn() == s2.firstColumn() &&
					s1.lastColumn() == s2.lastColumn() &&
					s1.firstLine() == s2.firstLine() &&
					s1.lastLine() == s2.lastLine();
}

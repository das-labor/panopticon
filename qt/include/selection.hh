#include <QDebug>
#include <panopticon/hash.hh>
#include <panopticon/region.hh>

#pragma once

struct DelegatePoint
{
	DelegatePoint(size_t f, po::offset s) : first(f), second(s) {}
	size_t first;
	po::offset second;
};

struct ElementPoint
{
	ElementPoint(quint64 f, quint64 s) : first(f), second(s) {}
	quint64 first;
	quint64 second;
};

/*!
 * \brief Continuous selection of characters in a Column.
 *
 * All lines have the same column (character) count and every character
 * represents the same amount of bytes. Used by the view part of the
 * application to encapsulate its selections.
 *
 * \ingroup gui
 */
template<typename Point>
class Selection
{
public:
	/**
	 * New ElementSelection spanning from line \e anc_line, column \e anc_col
	 * to line \e cur_line, column \e cur_col.
	 */
	Selection(const Point &anchor, const Point &cursor) : _anchor(anchor), _cursor(cursor) {}

	bool operator==(const Selection &s2) const
	{
		return minimum().second == s2.minimum().second &&
					 maximum().second == s2.maximum().second &&
					 minimum().first == s2.minimum().first &&
					 maximum().first == s2.maximum().first;
	}

	bool operator!=(const Selection &s2) const { return !(*this == s2); }

	const Point &anchor(void) const { return _anchor; }

	const Point &cursor(void) const { return _cursor; }

	Point minimum(void) const { return Point(std::min(_anchor.first,_cursor.first),std::min(_anchor.second,_cursor.second)); }
	Point maximum(void) const { return Point(std::max(_anchor.first,_cursor.first),std::max(_anchor.second,_cursor.second)); }

	/**
	 * Moves the cursor to line \e line, column \e col.
	 */
	void setCursor(const Point &sel) { _cursor = sel; }

	/**
	 * \returns True if s ⊄ this
	 */
	bool includes(const Selection &s) const
	{
		return (s.minimum().first > minimum().first || (s.minimum().first == minimum().first && s.minimum().second >= minimum().second)) &&
					 (s.maximum().first < maximum().first || (s.maximum().first == maximum().first && s.maximum().second <= maximum().second));
	}

	bool includes(const Point &p) const
	{
		return ((p.first == minimum().first || p.first == maximum().first) && p.second >= minimum().second && p.second <= maximum().second) ||
					 (p.first > minimum().first && p.first < maximum().first);
	}

	/**
	 * \returns True if disjoint from sel
	 */
	bool disjoint(const Selection &sel) const
	{
		return (maximum().first < sel.minimum().first || (maximum().first == sel.minimum().first && maximum().second < sel.minimum().second)) &&
					 (minimum().first < sel.maximum().first || (minimum().first == sel.maximum().first && minimum().second < sel.maximum().second));
	}

private:
	Point _anchor;
	Point _cursor;
};

template<typename Point>
QDebug operator<<(QDebug dbg, const Selection<Point> &c)
{
	dbg.nospace() << "[" << c.anchor().first << ", " << c.anchor().second << "] -> [" << c.cursor().first << ", " << c.cursor().second << "]";
	return dbg.space();
}

namespace std
{
	template<typename Point>
	struct hash<Selection<Point>>
	{
		size_t operator()(const Selection<Point> &s) const
		{
			return po::hash_struct(s.anchor().first,s.anchor().second,s.cursor().first,s.cursor().second);
		}
	};
}

template<typename Point>
uint qHash(const Selection<Point> &s) { return std::hash<Selection<Point>>()(s); }

struct ElementSelection : public Selection<ElementPoint>
{
	ElementSelection(quint64 anc_line, quint64 anc_col, quint64 cur_line, quint64 cur_col);

	quint64 anchorLine(void) const;
	quint64 anchorColumn(void) const;
	quint64 cursorLine(void) const;
	quint64 cursorColumn(void) const;

	quint64 firstLine(void) const;
	quint64 firstColumn(void) const;
	quint64 lastLine(void) const;
	quint64 lastColumn(void) const;
};

namespace std
{
	template<>
	struct hash<ElementSelection>
	{
		size_t operator()(const ElementSelection &s) const
		{
			return hash<Selection<ElementPoint>>()(s);
		}
	};
}

using DelegateSelection = Selection<DelegatePoint>;

class ElementSelectionObject : public QObject, ElementSelection
{
	Q_OBJECT
	Q_PROPERTY(int firstLine READ firstLine NOTIFY firstLineChanged)
	Q_PROPERTY(int lastLine READ lastLine NOTIFY lastLineChanged)
	Q_PROPERTY(int firstColumn READ firstColumn NOTIFY firstColumnChanged)
	Q_PROPERTY(int lastColumn READ lastColumn NOTIFY lastColumnChanged)
	Q_PROPERTY(int cursorLine READ cursorLine NOTIFY cursorLineChanged)
	Q_PROPERTY(int anchorLine READ anchorLine NOTIFY anchorLineChanged)
	Q_PROPERTY(int cursorColumn READ cursorColumn NOTIFY cursorColumnChanged)
	Q_PROPERTY(int anchorColumn READ anchorColumn NOTIFY anchorColumnChanged)

public:
	ElementSelectionObject(QObject *parent = 0);
	ElementSelectionObject(const ElementSelection &s, QObject *parent = 0);

signals:
	void firstLineChanged(void);
	void lastLineChanged(void);
	void firstColumnChanged(void);
	void lastColumnChanged(void);
	void cursorLineChanged(void);
	void anchorLineChanged(void);
	void cursorColumnChanged(void);
	void anchorColumnChanged(void);
};

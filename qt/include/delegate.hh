#include <QList>
#include <QString>
#include <QStringList>
#include <QSharedPointer>
#include <QtQuick>

#include <boost/optional.hpp>
#include <panopticon/region.hh>
#include <panopticon/hash.hh>
#include "selection.hh"

#pragma once

/**
 * \brief Delegates connect Columns to a database.
 *
 * Delegate and its subclasses create strings for the columns
 * to render and translate cursor/mouse movements from byte- to
 * elementoriented objects.
 * \ingroup gui
 */
class Delegate : public QObject
{
	Q_OBJECT

public:
	/**
	 * New delegate for \e sec of database \e db. Dimensions will be \e lines height,
	 *
	 * The parent object is used if the database \e db is private. In this case the parent object
	 * will recvieve the MouseSelection and CursorSelection events.
	 */
	Delegate(po::region_wloc r, QObject *parent = 0);

	virtual ~Delegate(void);

	/**
	 * Strings for a supplied line.
	 * Returns a list of strings representing the columns
	 * of a line. The length of the list can be less than \ref columns()!
	 */
	virtual QQuickItem *createRow(unsigned int i) = 0;
	virtual void deleteRow(QQuickItem*) = 0;

	/*!
	 * Number of lines this delegate spans.
	 */
	virtual unsigned int rowCount(void) const = 0;

	po::region_wloc region(void) const;

public slots:
	virtual void select(boost::optional<std::pair<po::offset,po::offset>>) = 0;

signals:
	void modified(void);
	void selected(boost::optional<po::offset>,bool);

private:
	po::region_wloc _region;
};

class BinaryDelegate : public Delegate
{
	Q_OBJECT

public:
	BinaryDelegate(po::region_wloc r, unsigned int width, QQmlEngine *, QQuickItem *p = 0);
	virtual ~BinaryDelegate(void);

	virtual QQuickItem *createRow(unsigned int i);
	virtual void deleteRow(QQuickItem*);

	virtual unsigned int rowCount(void) const;

	QQuickItem *createOverlay(const ElementSelection &sel);
	void deleteOverlay(QQuickItem *);

	void setCursor(const boost::optional<ElementSelection> &sel);

public slots:
	void elementClicked(int,int);
	void elementEntered(int,int);
	void collapseRows(void);
	virtual void select(boost::optional<std::pair<po::offset,po::offset>>);

private:
	unsigned int _width;

	QQmlEngine *_engine;
	QQmlComponent _rowComponent;
	QQmlComponent _titleComponent;
	QQmlComponent _cursorComponent;

	std::unordered_set<std::pair<ElementSelection,QQuickItem*>> _overlays;
	std::map<int,QQuickItem*> _visibleRows;

	boost::optional<ElementSelection> _cursor;
	QQuickItem *_cursorOverlay;
	bool _collapsed;
	boost::optional<po::slab> _cache;

	boost::optional<std::pair<QQuickItem*,QQuickItem*>> attachableRows(const ElementSelection &sel);
	void updateOverlays(const ElementSelection &sel);
};

#include <QList>
#include <QString>
#include <QStringList>
#include <QSharedPointer>
#include <QtQuick>

class Delegate;
class Line;

typedef QSharedPointer<Line> LineRef;

#include <source.hh>
#include <elementselection.hh>
#include <boost/optional.hpp>

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
	Delegate(const po::address_space &as, const po::rrange &r, QObject *parent = 0);

	virtual ~Delegate(void);

	/**
	 * Strings for a supplied line.
	 * Returns a list of strings representing the columns
	 * of a line. The length of the list can be less than \ref columns()!
	 */
	virtual QQuickItem *createRow(unsigned int i) = 0;
	virtual void deleteRow(QQuickItem*) = 0;

	virtual QQuickItem *createHead(void) = 0;
	virtual void deleteHead(QQuickItem *) = 0;

	/*!
	 * Number of lines this delegate spans.
	 */
	virtual unsigned int rowCount(void) const = 0;

	/*!
	 * Current cursor selection or NULL if nothing is selected. Used to initialize Columns
	 * after this have been created.
	 * Caller take ownership otf the returned object.
	 *
	virtual const boost::optional<ElementSelection>& cursor(void) const = 0;*/

	/*!
	 * Current mouse selection or NULL. See cursor().
	 * Caller take ownership otf the returned object.
	 *
	virtual const boost::optional<ElementSelection>& mouse(void) const = 0;*/

	const po::address_space &space(void) const;
	const po::rrange &range(void) const;

	//virtual boost::optional<ElementSelection> elementSelection(const po::rrange &sel);
	//virtual po::rrange byteSelection(const boost::optional<ElementSelection> &sel);

//public slots:
	//virtual void setMouse(const boost::optional<ElementSelection> &pos) = 0;
	//virtual void setCursor(const boost::optional<ElementSelection> &sel) = 0;

signals:
	void modified(void);

private:
	po::address_space m_space;
	po::rrange m_range;
};

class TestDelegateContext : public QObject
{
	Q_OBJECT
	Q_PROPERTY(QString address READ address NOTIFY addressChanged)
	Q_PROPERTY(QVariantList data READ data NOTIFY dataChanged)
	Q_PROPERTY(int row READ row NOTIFY rowChanged)

public:
	TestDelegateContext(QObject *parent = nullptr);
	TestDelegateContext(const QString &a, const QVariantList &d, int row, QObject *parent = nullptr);

	QString address(void) const;
	QVariantList data(void) const;
	int row(void) const;

signals:
	void addressChanged(void);
	void dataChanged(void);
	void rowChanged(void);

private:
	QString m_address;
	QVariantList m_data;
	int m_row;
};

class TestDelegate : public Delegate
{
	Q_OBJECT

public:
	TestDelegate(const po::address_space &as, const po::rrange &r, unsigned int width, QQmlEngine *, QQuickItem *p = 0);
	virtual ~TestDelegate(void);

	virtual QQuickItem *createRow(unsigned int i);
	virtual void deleteRow(QQuickItem*);

	virtual QQuickItem *createHead(void);
	virtual void deleteHead(QQuickItem *);

	virtual unsigned int rowCount(void) const;

	QQuickItem *createOverlay(const ElementSelection &sel);
	void deleteOverlay(QQuickItem *);

	void setCursor(const boost::optional<ElementSelection> &sel);

public slots:
	void elementClicked(int,int);
	void elementEntered(int,int);
	void collapseRows(void);

private:
	unsigned int m_width;

	QQmlEngine *m_engine;
	QQmlComponent m_rowComponent;
	QQmlComponent m_headComponent;
	QQmlComponent m_cursorComponent;

	std::unordered_set<std::pair<ElementSelection,QQuickItem*>> m_overlays;
	std::map<int,QQuickItem*> m_visibleRows;

	boost::optional<ElementSelection> m_cursor;
	QQuickItem *m_cursorOverlay;
	bool m_collapsed;

	void updateOverlays(const ElementSelection &sel);
};

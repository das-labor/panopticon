#include <map>
#include <QtQuick>
#include "session.hh"
#include "delegate.hh"

#pragma once

class Header : public QObject
{
	Q_OBJECT
	Q_PROPERTY(QString name READ name NOTIFY nameChanged)
	Q_PROPERTY(bool collapsed READ collapsed NOTIFY collapsedChanged)
	Q_PROPERTY(int id READ id NOTIFY idChanged)

public:
	Header(void);
	Header(const QString &n, bool col, int id);
	virtual ~Header(void);

	QString name(void) const;
	bool collapsed(void) const;
	int id(void) const;

signals:
	void nameChanged(void);
	void collapsedChanged(void);
	void idChanged(void);

private:
	QString m_name;
	bool m_collapsed;
	int m_id;
};

struct LinearViewBlock
{
	enum Type
	{
		Data,
		Header,
		HeaderCollapsed,
	};

	LinearViewBlock(void);
	LinearViewBlock(Type t, QSharedPointer<Delegate> d, int id);
	LinearViewBlock(const LinearViewBlock &r);

	bool operator==(const LinearViewBlock &r) const;
	LinearViewBlock &operator+=(const LinearViewBlock &r);

	Type type;
	QSharedPointer<Delegate> delegate;
	int id;	///< Key when in m_hidden
};

class LinearViewContext : public QObject
{
	Q_OBJECT
	Q_PROPERTY(qreal columnWidth READ columnWidth WRITE setColumnWidth NOTIFY columnWidthChanged)

public:
	LinearViewContext(QObject *parent = 0);

	qreal columnWidth(void) const;
	void setColumnWidth(qreal);

signals:
	void columnWidthChanged(void);

private:
	qreal m_columnWidth;
};

class LinearView : public QQuickItem
{
	Q_OBJECT
	Q_PROPERTY(Session* session READ session WRITE setSession NOTIFY sessionChanged)

public:
	using rowIndex = int;

	LinearView(QQuickItem *parent = 0);
	virtual ~LinearView(void);

	Session *session(void);
	void setSession(Session *);

public slots:
	void scrollViewport(float delta = 0);
	void delegateModified(void);
	void selected(po::bound b);

signals:
	void sessionChanged(void);

protected:
	virtual void wheelEvent(QWheelEvent *event);
	virtual void mouseMoveEvent(QMouseEvent *event);
	virtual void mousePressEvent(QMouseEvent *event);
	virtual void geometryChanged(const QRectF &newGeometry, const QRectF &oldGeometry);

private:
	QQmlEngine _engine;
	LinearViewContext _context;
	Session* _session;
	boost::icl::split_interval_map<rowIndex,std::shared_ptr<Delegate>> _delegates;
	rowIndex _globalRowIndex;
	int _yOffset;
	std::map<rowIndex,QQuickItem*> _visibleRows;
	std::map<rowIndex,std::tuple<rowIndex,bool>> _references;

	void insertRows(float y, rowIndex gri, bool up);

private slots:
	void rowHeightChanged(void);
};

#include <map>
#include <QtQuick>
#include "session.hh"
#include "delegate.hh"

#pragma once

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
	void selected(boost::optional<po::offset>, bool);

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
	std::unordered_map<const Delegate*,size_t> _ordinals;
	boost::icl::split_interval_map<rowIndex,std::shared_ptr<Delegate>> _delegates;
	rowIndex _globalRowIndex;
	int _yOffset;
	std::map<rowIndex,QQuickItem*> _visibleRows;
	std::map<rowIndex,std::tuple<rowIndex,bool>> _references;
	boost::optional<DelegateSelection> _cursor;

	void insertRows(float y, rowIndex gri, bool up);
	size_t regionIndex(const Delegate *del) const;

private slots:
	void rowHeightChanged(void);
};

#include <QObject>
#include <QQmlContext>
#include <QQuickItem>
#include <QQuickPaintedItem>
#include <QImage>
#include <QVariant>
#include <QLineF>
#include <QPointF>
#include <vector>
#include <memory>
#include <mutex>
#include <cstdint>

#include "glue.h"
#include "qbasicblockline.h"

#pragma once

class QControlFlowGraph : public QQuickPaintedItem {
	Q_OBJECT

public:
	QControlFlowGraph(QQuickItem* parent = 0);
	virtual ~QControlFlowGraph();

	Q_PROPERTY(QString uuid READ getUuid WRITE setUuid NOTIFY uuidChanged)
	Q_PROPERTY(QVariant delegate READ getDelegate WRITE setDelegate NOTIFY delegateChanged)
	Q_PROPERTY(QVariant edgeDelegate READ getEdgeDelegate WRITE setEdgeDelegate NOTIFY edgeDelegateChanged)
	Q_PROPERTY(QVariantList preview READ getPreview NOTIFY previewChanged)

	QString getUuid(void) const;
	QVariant getDelegate(void) const;
	QVariant getEdgeDelegate(void) const;
	QVariantList getPreview(void) const;

	void setUuid(QString& s);
	void setDelegate(QVariant& v);
	void setEdgeDelegate(QVariant& v);

	virtual void paint(QPainter* painter = nullptr) override;

	static std::mutex allInstancesLock;
	static std::vector<QControlFlowGraph*> allInstances;

	using node_tuple = std::tuple<unsigned int,float,float,bool,std::vector<std::shared_ptr<QBasicBlockLine>>>;

public slots:
	void insertNode(QString uuid, unsigned int id, float x, float y, bool is_entry, QVector<QBasicBlockLine*> block);
	void insertEdges(QString uuid, QVector<unsigned int> ids, QVector<QString> label, QVector<QString> kind,
			             QVector<QPointF> head, QVector<QPointF> tail, QImage svg);
	void requestPreview(QString uuid);

signals:
	void uuidChanged(void);
	void delegateChanged(void);
	void edgeDelegateChanged(void);
	void previewChanged(void);

protected:
	void updateNodes(void);
	void updateEdges(void);
	void updateNode(unsigned int id, float x, float y, bool is_entry, const std::vector<std::shared_ptr<QBasicBlockLine>>& block, QQmlContext*);

	QString m_uuid;
	std::unique_ptr<QQmlComponent> m_delegate;
	std::unique_ptr<QQmlComponent> m_edgeDelegate;
	std::vector<std::pair<std::unique_ptr<QQuickItem>,QQmlContext*>> m_nodeItems;
	std::vector<std::pair<std::unique_ptr<QQuickItem>,QQmlContext*>> m_edgeItems;
	std::vector<node_tuple> m_nodes;
	std::tuple<std::string,std::vector<std::shared_ptr<QBasicBlockLine>>> m_preview;
	std::pair<std::vector<std::tuple<unsigned int,QString,QString,QPointF,QPointF>>,QImage> m_edges;
};

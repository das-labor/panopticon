#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlEngine>
#include <QPainter>
#include <QtQml/qqml.h>
#include <iostream>
#include <vector>

#include "qcontrolflowgraph.h"
#include "qpanopticon.h"

std::vector<QControlFlowGraph*> QControlFlowGraph::allInstances = {};
std::mutex QControlFlowGraph::allInstancesLock;

QControlFlowGraph::QControlFlowGraph(QQuickItem* parent)
: QQuickPaintedItem(parent), m_uuid(""), m_delegate(nullptr), m_edgeDelegate(nullptr) {
	std::lock_guard<std::mutex> guard(allInstancesLock);
	allInstances.push_back(this);
}

QControlFlowGraph::~QControlFlowGraph() {
	std::lock_guard<std::mutex> guard(allInstancesLock);
	for(auto i = allInstances.begin(); i != allInstances.end(); i++) {
		if(*i == this) {
			allInstances.erase(i);
			return;
		}
	}
}

QString QControlFlowGraph::getUuid(void) const { return m_uuid; }

QVariant QControlFlowGraph::getDelegate(void) const {
	QVariant v;
	v.setValue<QObject*>(static_cast<QObject*>(m_delegate.get()));
	return v;
}

QVariant QControlFlowGraph::getEdgeDelegate(void) const {
	QVariant v;
	v.setValue<QObject*>(static_cast<QObject*>(m_edgeDelegate.get()));
	return v;
}

QVariantList QControlFlowGraph::getPreview(void) const {
	QVariantList ret;

	for(const std::shared_ptr<QBasicBlockLine>& l: std::get<1>(m_preview)) {
		QVariant v;

		v.setValue(l.get());
		ret.append(v);
	}

	return ret;
}

bool QControlFlowGraph::getIsEmpty(void) const {
  return m_nodes.empty();
}

void QControlFlowGraph::setUuid(QString& s) {
	m_uuid = s;

	m_nodes.clear();
	m_edges.first.clear();
	m_edges.second = QImage();
	emit uuidChanged();
	emit isEmptyChanged();
  updateNodes();
  updateEdges();
  update();

	if(m_uuid != "" && m_delegate && QPanopticon::staticGetFunction) {
		std::string uuid = m_uuid.toStdString();
		QPanopticon::staticGetFunction(uuid.c_str(),false,true,true);
	}
}

void QControlFlowGraph::setDelegate(QVariant& v) {
	if(v.canConvert<QQmlComponent*>()) {
		QQmlComponent* item = v.value<QQmlComponent*>();
		m_delegate = std::unique_ptr<QQmlComponent>(item);

		m_nodes.clear();
		emit delegateChanged();
		emit isEmptyChanged();
    updateNodes();

		if(m_uuid != "" && m_delegate && QPanopticon::staticGetFunction) {
			std::string uuid = m_uuid.toStdString();
			QPanopticon::staticGetFunction(uuid.c_str(),false,true,true);
		}
	}
}

void QControlFlowGraph::setEdgeDelegate(QVariant& v) {
	if(v.canConvert<QQmlComponent*>()) {
		QQmlComponent* item = v.value<QQmlComponent*>();
		m_edgeDelegate = std::unique_ptr<QQmlComponent>(item);

		m_edges.first.clear();
		m_edges.second = QImage();
    updateEdges();
		emit edgeDelegateChanged();
    update();

		if(m_uuid != "" && m_edgeDelegate && QPanopticon::staticGetFunction) {
			std::string uuid = m_uuid.toStdString();
			QPanopticon::staticGetFunction(uuid.c_str(),false,false,true);
		}
	}
}

void QControlFlowGraph::insertEdges(QString uuid, QVector<unsigned int> ids, QVector<QString> label,
		                                QVector<QString> kind, QVector<QPointF> head, QVector<QPointF> tail,
																		QImage svg) {
	if(uuid == m_uuid) {
		std::vector<std::tuple<unsigned int,QString,QString,QPointF,QPointF>> edges;

		for(size_t idx = 0; idx < label.size(); ++idx) {
			edges.emplace_back(std::make_tuple(ids[idx],label[idx],kind[idx],head[idx],tail[idx]));
		}

		setWidth(svg.width());
		setHeight(svg.height());
		m_edges = std::make_pair(std::move(edges),std::move(svg));
		updateEdges();
	}
}

/*static QString svgPath(const QPointF& head, const QVariantList& segs, const QPointF& tail) {
	QVector<QPointF> points;
	QString svg;

	if(segs.size() > 0) {
		points.append(segs[0].toLineF().p1());
		for(const auto& v: segs) {
			points.append(v.toLineF().p2());
		}

		const QPointF* first = &points[0];
		const QPointF* prev = first;
			 switch(edge.kind) {
			 case "branch":
			 ctx.strokeStyle = "green";
			 ctx.fillStyle = "green";
			 break;

			 case "branch-backedge":
			 ctx.strokeStyle = "green";
			 ctx.fillStyle = "green";
			 break;

			 case "fallthru":
			 ctx.strokeStyle = "red";
			 ctx.fillStyle = "red";
			 break;

			 case "jump":
			 default:
			 ctx.strokeStyle = "black";
			 ctx.fillStyle = "black";
			 break;

			 case "jump-backedge":
			 ctx.strokeStyle = "black";
			 ctx.fillStyle = "black";
			 break;
			 }
		auto lerp = [](const QPointF& a,const QPointF& b,float p) {
			return QPointF(a.x() + p * (b.x() - a.x()),a.y() + p * (b.y() - a.y()));
		};
		QString path = QString("M %1 %2").arg(first->x()).arg(first->y());

		for(size_t i = 0; i < points.size() - 1; ++i) {
			const QPointF* cur = &points[i];
			const QPointF* next = &points[i+1];
			QPointF a = lerp(*cur,*prev,.2);
			QPointF b = lerp(*cur,*next,.2);

			path += QString(" L %1 %2").arg(a.x()).arg(a.y());
			path += QString(" C %1 %2 %3 %4 %5 %6")
				.arg(cur->x()).arg(cur->y())
				.arg(cur->x()).arg(cur->y())
				.arg(b.x()).arg(b.y());

			prev = cur;
		}

		const QPointF& last = points[points.size() - 1];

		path += QString(" L %1 %2").arg(last.x()).arg(last.y());

		svg = QString("<path d='%1' style='fill:none; stroke:black; stroke-width:1' />").arg(path);
	} else {
		svg = QString("");
	}
	QString ret = QString("<svg xmlns='http://www.w3.org/2000/svg'>%1</svg>").arg(svg);
	return QString("data:image/svg+xml;charset=utf-8;base64,%1").arg(QString(ret.toLatin1().toBase64()));
}*/

void QControlFlowGraph::updateEdges(void) {
	if(!m_edgeDelegate) return;

	const auto& edges = m_edges.first;

	while(m_edgeItems.size() < edges.size()) {
		QQmlContext *ctx = new QQmlContext(QQmlEngine::contextForObject(this));

		ctx->setContextProperty("edgeId",QVariant::fromValue(-1));
		ctx->setContextProperty("edgeLabel",QVariant::fromValue(QString("")));
		ctx->setContextProperty("edgeKind",QVariant::fromValue(QString("")));
		ctx->setContextProperty("edgeHead",QVariant::fromValue(QPointF()));
		ctx->setContextProperty("edgeTail",QVariant::fromValue(QPointF()));

		QObject* obj = m_edgeDelegate->create(ctx);
		if(!obj) return;

		ctx->setParent(obj);
		obj->setParent(m_edgeDelegate.get());

		QQuickItem* item = qobject_cast<QQuickItem*>(obj);
		if(!item) return;

		item->setParentItem(this);
		item->setVisible(false);

		m_edgeItems.push_back(std::make_pair(std::move(std::unique_ptr<QQuickItem>(item)),ctx));
	}

	size_t index = 0;
	QString path;
	for(; index < edges.size(); ++index) {
		auto item = m_edgeItems[index].first.get();
		auto ctx = m_edgeItems[index].second;
		const auto& edge = edges[index];

		ctx->setContextProperty("edgeId",QVariant::fromValue(std::get<0>(edge)));
		ctx->setContextProperty("edgeLabel",QVariant::fromValue(std::get<1>(edge)));
		ctx->setContextProperty("edgeKind",QVariant::fromValue(std::get<2>(edge)));
		ctx->setContextProperty("edgeHead",QVariant::fromValue(std::get<3>(edge)));
		ctx->setContextProperty("edgeTail",QVariant::fromValue(std::get<4>(edge)));
		item->setVisible(true);
	}

	for(; index < m_edgeItems.size(); ++index) {
		m_edgeItems[index].first.get()->setVisible(false);
	}
	update();
}

void QControlFlowGraph::insertNode(QString uuid, unsigned int id, float x, float y, bool is_entry, QVector<QBasicBlockLine*> lines) {
	std::vector<std::shared_ptr<QBasicBlockLine>> vec;

	for(auto bbl: lines) {
		vec.emplace_back(std::shared_ptr<QBasicBlockLine>(bbl));
	}

	// full control flow graph
	if(uuid == m_uuid) {
		for(size_t idx = 0; idx < m_nodes.size(); ++idx) {
			auto& tpl = m_nodes[idx];

			if(std::get<0>(tpl) == id) {
				tpl = std::make_tuple(id,x,y,is_entry,vec);
				updateNode(id,x,y,is_entry,vec,m_nodeItems.at(idx).second);

				return;
			}
		}

		m_nodes.emplace_back(std::make_tuple(id,x,y,is_entry,vec));
		emit isEmptyChanged();
		updateNodes();
	}

	// preview
	if(std::get<0>(m_preview) == uuid.toStdString() && is_entry) {
		std::get<1>(m_preview) = vec;
		emit previewChanged();
	}
}

void QControlFlowGraph::updateNodes(void) {
	if(!m_delegate) return;

	while(m_nodeItems.size() < m_nodes.size()) {
		QQmlContext *ctx = new QQmlContext(QQmlEngine::contextForObject(this));

		ctx->setContextProperty("blockContents",QVariantList());
		ctx->setContextProperty("blockX",QVariant::fromValue(0.0f));
		ctx->setContextProperty("blockY",QVariant::fromValue(0.0f));
		ctx->setContextProperty("blockId",QVariant::fromValue(0));
		ctx->setContextProperty("blockIsEntry",QVariant::fromValue(false));
		ctx->setContextProperty("blockIsBlock",QVariant::fromValue(false));

		QObject* obj = m_delegate->create(ctx);
		if(!obj) return;

		ctx->setParent(obj);
		obj->setParent(m_delegate.get());

		QQuickItem* item = qobject_cast<QQuickItem*>(obj);
		if(!item) return;

		item->setParentItem(this);
		item->setVisible(false);

		m_nodeItems.push_back(std::make_pair(std::move(std::unique_ptr<QQuickItem>(item)),ctx));
	}

	float min_x = std::numeric_limits<float>::infinity();
	float max_x = -std::numeric_limits<float>::infinity();
	float min_y = std::numeric_limits<float>::infinity();
	float max_y = -std::numeric_limits<float>::infinity();
	size_t index = 0;
	for(; index < m_nodes.size(); ++index) {
		auto item = m_nodeItems[index].first.get();
		auto ctx = m_nodeItems[index].second;
		const auto& line = m_nodes[index];

		min_x = std::min(min_x,std::get<1>(line));
		max_x = std::max(max_x,std::get<1>(line));
		min_y = std::min(min_y,std::get<2>(line));
		max_y = std::max(max_y,std::get<2>(line));

		updateNode(std::get<0>(line),std::get<1>(line),std::get<2>(line),std::get<3>(line),std::get<4>(line),ctx);
		item->setVisible(true);
	}

	for(; index < m_nodeItems.size(); ++index) {
		m_nodeItems[index].first.get()->setVisible(false);
	}
}

void QControlFlowGraph::updateNode(unsigned int id, float x, float y, bool is_entry, const std::vector<std::shared_ptr<QBasicBlockLine>>& block, QQmlContext* ctx) {
	QVariantList contents;

	for(const std::shared_ptr<QBasicBlockLine>& l: block) {
		QVariant v;

		v.setValue(l.get());
		contents.append(v);
	}

  bool is_block = block.size() != 1 || block[0]->getOpcode() != "";

	ctx->setContextProperty("blockContents",contents);
	ctx->setContextProperty("blockX",QVariant::fromValue(x));
	ctx->setContextProperty("blockY",QVariant::fromValue(y));
	ctx->setContextProperty("blockId",QVariant::fromValue(id));
	ctx->setContextProperty("blockIsEntry",QVariant::fromValue(is_entry));
	ctx->setContextProperty("blockIsBlock",QVariant::fromValue(is_block));
}

void QControlFlowGraph::paint(QPainter* painter) {
	if(!painter || m_edges.second.isNull()) return;

	painter->drawImage(0,0,m_edges.second);
}

void QControlFlowGraph::requestPreview(QString quuid) {
	std::string uuid = quuid.toStdString();

	if(std::get<0>(m_preview) != uuid && QPanopticon::staticGetFunction) {
		m_preview = std::make_pair(uuid,std::vector<std::shared_ptr<QBasicBlockLine>>());
		QPanopticon::staticGetFunction(uuid.c_str(),true,true,false);
	}
}

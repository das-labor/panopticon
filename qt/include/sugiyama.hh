/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Panopticon authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <mutex>
#include <memory>

#include <QtQml>
#include <QtQuick>
#include <QQuickItem>
#include <QQmlContext>

#include <boost/optional.hpp>

#include <panopticon/procedure.hh>

#include "session.hh"

#pragma once

struct node_proxy
{
	node_proxy(QQmlComponent* comp,QQuickItem* parent)
	: _item(), _context()
	{
		if(comp)
		{
			_context.reset(new QQmlContext(QQmlEngine::contextForObject(parent)),[](QQmlContext *q) { q->deleteLater(); });
			_context->setContextProperty("modelData",QVariant());
			_context->setContextProperty("incomingEdges",QVariantList());
			_context->setContextProperty("incomingNodes",QVariantList());
			_context->setContextProperty("outgoingNodes",QVariantList());
			_context->setContextProperty("outgoingEdges",QVariantList());
			_context->setContextProperty("firstRank",QVariant());
			_context->setContextProperty("lastRank",QVariant());
			_context->setContextProperty("computedX",QVariant());
			_context->setContextProperty("payload",QVariant());
			_item.reset(qobject_cast<QQuickItem*>(comp->create(_context.get())),[](QQuickItem *q) { q->deleteLater(); });
			_item->setParentItem(parent);
		}
	}

	std::shared_ptr<QQuickItem> const& item(void) const { return _item; }
	std::shared_ptr<QQmlContext> const& context(void) const { return _context; }

private:
	std::shared_ptr<QQuickItem> _item;
	std::shared_ptr<QQmlContext> _context;
};

struct edge_proxy
{
	edge_proxy(QQmlComponent* comp,QQuickItem* parent)
	: _edge(), _label(), _head(), _tail(),
	  _edge_cxt(), _label_cxt(), _head_cxt(), _tail_cxt()
	{
		if(comp)
		{
			_edge_cxt.reset(new QQmlContext(QQmlEngine::contextForObject(parent)),[](QQmlContext *q) { q->deleteLater(); });
			QObject* edge_obj = comp->create(_edge_cxt.get());

			QQuickItem* ed = qobject_cast<QQuickItem*>(edge_obj);
			if(!ed)
			{
				delete edge_obj;
				qWarning() << "Edge delegate needs to be an Item class";
			}
			else
			{
				_edge.reset(ed,[](QQuickItem* q) { q->deleteLater(); });
				_edge->setParent(parent);
				_edge->setParentItem(parent);
			}
		}
	}

	void replaceDecorations(boost::optional<std::pair<QQuickItem*,QQmlContext*>> lb,
			boost::optional<std::pair<QQuickItem*,QQmlContext*>> tl,
			boost::optional<std::pair<QQuickItem*,QQmlContext*>> hd)
	{
		if(lb)
		{
			_label.reset(lb->first,[](QQuickItem* q) { q->deleteLater(); });
			_label_cxt.reset(lb->second,[](QQmlContext* q) { q->deleteLater(); });
		}
		else
		{
			_label.reset();
			_label_cxt.reset();
		}

		if(tl)
		{
			_tail.reset(tl->first,[](QQuickItem* q) { q->deleteLater(); });
			_tail_cxt.reset(tl->second,[](QQmlContext* q) { q->deleteLater(); });
		}
		else
		{
			_tail.reset();
			_tail_cxt.reset();
		}

		if(hd)
		{
			_head.reset(hd->first,[](QQuickItem* q) { q->deleteLater(); });
			_head_cxt.reset(hd->second,[](QQmlContext* q) { q->deleteLater(); });
		}
		else
		{
			_head.reset();
			_head_cxt.reset();
		}
	}

	std::shared_ptr<QQuickItem> const& edge(void) const { return _edge; }
	std::shared_ptr<QQuickItem> const& label(void) const { return _label; }
	std::shared_ptr<QQuickItem> const& tail(void) const { return _tail; }
	std::shared_ptr<QQuickItem> const& head(void) const { return _head; }

private:
	std::shared_ptr<QQuickItem> _edge, _label, _head, _tail;
	std::shared_ptr<QQmlContext> _edge_cxt, _label_cxt, _head_cxt, _tail_cxt;
};

using itmgraph = po::digraph<node_proxy,edge_proxy>;

struct point
{
	enum Type : uint8_t
	{
		Entry,
		Exit,
		Corner,
		Center
	};

	bool operator==(point const& p) const { return p.node == node && p.x == x && p.y == y && type == p.type; }
	bool operator!=(point const& p) const { return !(p == *this); }

	itmgraph::vertex_descriptor node;
	Type type;
	int x, y;
};

using visgraph = std::unordered_multimap<point,point>;

namespace std
{
	template<>
	struct hash<point>
	{
		size_t operator()(struct point const& p) const
		{
			return po::hash_struct<int,int,int,uint8_t>(p.node.id,p.x,p.y,p.type);
		}
	};

	template<>
	struct hash<node_proxy>
	{
		size_t operator()(node_proxy const& p) const
		{
			return po::hash_struct(p.item().get(),p.context().get());
		}
	};
}

class Sugiyama : public QQuickPaintedItem
{
	Q_OBJECT

	Q_PROPERTY(QQmlComponent* vertex READ vertex WRITE setVertex NOTIFY vertexChanged)
	Q_PROPERTY(QQmlComponent* edge READ edge WRITE setEdge NOTIFY edgeChanged)
	Q_PROPERTY(QObject* procedure READ procedure WRITE setProcedure NOTIFY procedureChanged)

public:
	using layout_type = std::unordered_map<itmgraph::vertex_descriptor,std::tuple<unsigned int,unsigned int,unsigned int>>;
	using route_type = std::unordered_map<itmgraph::edge_descriptor,std::pair<QPainterPath,QPointF>>;
	using cache_type = std::tuple<itmgraph,boost::optional<layout_type>,boost::optional<route_type>>;

	static const int nodeBorderPadding;
	static const int edgeRadius;
	static const int nodePortPadding;

	Sugiyama(QQuickItem *parent = nullptr);
	virtual ~Sugiyama(void);

	QQmlComponent* vertex(void) const { return _vertex; }
	QQmlComponent* edge(void) const { return _edge; }
	QObject* procedure(void) const { return _procedure; }

	void setVertex(QQmlComponent* c);
	void setEdge(QQmlComponent* c);
	void setProcedure(QObject* o);

	virtual void paint(QPainter*) override;

public slots:
	void layout(void);
	void route(void);
	void updateEdge(QObject *);
	void processRoute(void);
	void processLayout(void);

signals:
	void vertexChanged(void);
	void edgeChanged(void);
	void procedureChanged(void);

	void layoutStart(void);
	void layoutDone(void);
	void routeStart(void);
	void routeDone(void);

private:
	// Properties
	QQmlComponent* _vertex;
	QQmlComponent* _edge;
	Procedure* _procedure;

	std::unordered_map<po::proc_wloc,cache_type> _cache;
	QSignalMapper _mapper;
	QFutureWatcher<std::pair<po::proc_wloc,layout_type>> _layoutWatcher;
	QFutureWatcher<std::pair<po::proc_wloc,route_type>> _routeWatcher;
	bool _routingNeeded;
	std::mutex _mutex;

	void positionNode(itmgraph::vertex_descriptor v, itmgraph const& graph, std::tuple<unsigned int,unsigned int,unsigned int> pos);
	void positionEdgeDecoration(itmgraph::edge_descriptor e, cache_type const& cache);
	void updateEdgeDecorations(itmgraph::edge_descriptor e, cache_type& cache);
	void redoAttached(void);
	void scheduleLayout(po::proc_loc proc);
};

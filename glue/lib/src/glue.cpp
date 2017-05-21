#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlEngine>
#include <QImage>
#include <QPainter>
#include <QSvgRenderer>
#include <QtQml/qqml.h>
#include <iostream>

#include "glue.h"
#include "qpanopticon.h"
#include "qcontrolflowgraph.h"

extern "C" void update_function_node(const char* uuid, uint32_t id, float x, float y, int8_t is_entry, const BasicBlockLine** lines) {
	QString uuid_str(uuid);
	std::lock_guard<std::mutex> guard(QControlFlowGraph::allInstancesLock);

	for(auto cfg: QControlFlowGraph::allInstances) {
		QVector<QBasicBlockLine*> qobjs;
		size_t idx = 0;

		while(lines && lines[idx]) {
			const BasicBlockLine *line = lines[idx];
			QBasicBlockLine* qobj = new QBasicBlockLine(*line);

			qobjs.append(qobj);
			++idx;
		}

		if(!qobjs.empty()) {
			cfg->metaObject()->invokeMethod(
					cfg,
					"insertNode",
					Qt::QueuedConnection,
					Q_ARG(QString,uuid_str),
					Q_ARG(unsigned int,id),
					Q_ARG(float,x),
					Q_ARG(float,y),
					Q_ARG(bool,is_entry != 0),
					Q_ARG(QVector<QBasicBlockLine*>,qobjs));
		}
	}
}

extern "C" void update_function_edges(const char* uuid, const uint32_t* ids,
                                      const char** labels,const char** kinds,
																			const float* head_xs,const float* head_ys,
																			const float* tail_xs,const float* tail_ys,
																		  const char* svg) {
	QString uuid_str(uuid);
	std::lock_guard<std::mutex> guard(QControlFlowGraph::allInstancesLock);

  for(auto cfg: QControlFlowGraph::allInstances) {
    QVector<QPointF> heads;
    QVector<QPointF> tails;
    QVector<QString> label_vec;
    QVector<QString> kind_vec;
    QVector<unsigned int> id_vec;
    size_t idx = 0;

    while(head_xs && head_ys && tail_xs && tail_ys && ids && labels &&
        labels[idx] && kinds && kinds[idx])
    {
      QPointF head(head_xs[idx],head_ys[idx]);
      QPointF tail(tail_xs[idx],tail_ys[idx]);
      QString label(labels[idx]);
      QString kind(kinds[idx]);
      unsigned int id = ids[idx];

      heads.append(head);
      tails.append(tail);
      label_vec.append(label);
      kind_vec.append(kind);
      id_vec.append(id);

      ++idx;
    }

    if(!id_vec.empty()) {
      QSvgRenderer renderer;
      renderer.load(QByteArray(svg));
      auto vp = renderer.viewBox();
      QImage img(vp.width(),vp.height(),QImage::Format_ARGB32_Premultiplied);
      img.fill(Qt::transparent);
      QPainter painter(&img);
      renderer.render(&painter);

      cfg->metaObject()->invokeMethod(
          cfg,
          "insertEdges",
          Qt::QueuedConnection,
          Q_ARG(QString,uuid_str),
          Q_ARG(QVector<unsigned int>,id_vec),
          Q_ARG(QVector<QString>,label_vec),
          Q_ARG(QVector<QString>,kind_vec),
          Q_ARG(QVector<QPointF>,heads),
          Q_ARG(QVector<QPointF>,tails),
          Q_ARG(QImage,img));
    }
  }
}

extern "C" void update_sidebar_items(const SidebarItem** items) {
	size_t idx = 0;
	QPanopticon *panop = QPanopticon::staticInstance;
	if(!panop) return;

	QSidebar *sidebar = panop->getSidebar();

	while(items && items[idx]) {
		const SidebarItem *item = items[idx];
		QString title(item->title);
		QString subtitle(item->subtitle);
		QString uuid(item->uuid);

		sidebar->metaObject()->invokeMethod(
				sidebar,
				"insert",
				Qt::QueuedConnection,
				Q_ARG(QString,title),
				Q_ARG(QString,subtitle),
				Q_ARG(QString,uuid));
		++idx;
	}
}

extern "C" void update_undo_redo(int8_t undo, int8_t redo) {
	QPanopticon *panop = QPanopticon::staticInstance;

	if(panop) {
		panop->metaObject()->invokeMethod(
				panop,
				"updateUndoRedo",
				Qt::QueuedConnection,
				Q_ARG(bool,undo != 0),
				Q_ARG(bool,redo != 0));
	}
}

extern "C" void update_current_session(const char* path) {
	QPanopticon *panop = QPanopticon::staticInstance;

	if(panop) {
		panop->metaObject()->invokeMethod(
				panop,
				"updateCurrentSession",
				Qt::QueuedConnection,
				Q_ARG(QString,QString(path)));
	}
}

extern "C" void start_gui_loop(const char *dir, const char* f, const RecentSession** sess,
															 GetFunctionNodesFunc gfn, GetFunctionEdgesFunc gfe,
															 OpenProgramFunc op, SaveSessionFunc ss,
															 CommentOnFunc co, RenameFunctionFunc rf, SetValueForFunc svf,
															 UndoFunc u, RedoFunc r) {
	int argc = 1;
	char *argv[1] = { "Panopticon" };
	QGuiApplication app(argc,argv);

	QPanopticon::staticGetFunctionNodes = gfn;
	QPanopticon::staticGetFunctionEdges = gfe;
	QPanopticon::staticOpenProgram = op;
	QPanopticon::staticSaveSession = ss;
	QPanopticon::staticCommentOn = co;
	QPanopticon::staticRenameFunction = rf;
	QPanopticon::staticSetValueFor = svf;
	QPanopticon::staticUndo = u;
	QPanopticon::staticRedo = r;
	QPanopticon::staticInitialFile = QString(f);

	for(size_t idx = 0; sess[idx]; ++idx) {
		QRecentSession *qobj = new QRecentSession(*sess[idx]);
		QPanopticon::staticRecentSessions.push_back(qobj);
	}

	qRegisterMetaType<QVector<QBasicBlockLine*>>();
	qRegisterMetaType<QVector<QPointF>>();
	qRegisterMetaType<QVector<unsigned int>>();
	qRegisterMetaType<QVector<QString>>();
	qmlRegisterType<QControlFlowGraph>("Panopticon", 1, 0, "ControlFlowGraph");
	qmlRegisterSingletonType<QPanopticon>("Panopticon", 1, 0, "Panopticon", qpanopticon_provider);

	QQmlApplicationEngine engine;
	QString qmlDir(dir);

	engine.addImportPath(qmlDir);
	engine.load(qmlDir + QString("/Panopticon/Window.qml"));

	app.exec();
}

/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2017  Panopticon authors
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

#include <QObject>
#include <QAbstractListModel>
#include <QModelIndex>
#include <QVariant>

#include <tuple>

#pragma once

class QSidebar : public QAbstractListModel {
	Q_OBJECT

public:
	QSidebar(QObject* parent = 0);
	virtual ~QSidebar();

	virtual int rowCount(const QModelIndex& parent = QModelIndex()) const override;
	virtual QVariant data(const QModelIndex& idx, int role = Qt::DisplayRole) const override;
	virtual QHash<int, QByteArray> roleNames(void) const override;

public slots:
	void insert(QString title,QString subtitle,QString uuid);

protected:
	std::vector<std::tuple<QString,QString,QString>> m_items;
};

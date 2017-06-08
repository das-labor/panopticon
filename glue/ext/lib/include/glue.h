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
#include <QQmlContext>
#include <QQuickItem>
#include <QVariant>
#include <vector>
#include <memory>
#include <mutex>
#include <cstdint>

#pragma once

struct BasicBlockOperand {
	const char* kind;
	const char* display;
	const char* alt;
	const char* data;
};

struct BasicBlockLine {
	const char* opcode;
	const char* region;
	uint64_t offset;
	const char* comment;
	const BasicBlockOperand** args;
};

struct BasicBlock {
	const BasicBlockLine** lines;
};

struct SidebarItem {
	const char* title;
	const char* subtitle;
	const char* uuid;
};

struct RecentSession {
	const char* title;
	const char* kind;
	const char* path;
	uint32_t timestamp;
};

typedef int32_t (*GetFunctionFunc)(const char* uuid, int8_t only_entry, int8_t do_nodes, int8_t do_edges);
typedef int32_t (*SubscribeToFunc)(const char* uuid, int8_t subscribe);

// session management
typedef int32_t (*OpenProgramFunc)(const char* path);
typedef int32_t (*SaveSessionFunc)(const char* path);

// actions
typedef int32_t (*CommentOnFunc)(uint64_t address, const char* comment);
typedef int32_t (*RenameFunctionFunc)(const char* uuid, const char* name);
typedef int32_t (*SetValueForFunc)(const char* uuid, const char* variable, const char* value);

// undo/redo
typedef int32_t (*UndoFunc)();
typedef int32_t (*RedoFunc)();

class QSideBarItem : public QObject {
	Q_OBJECT
public:
	QSideBarItem(QObject* parent = 0) : QObject(parent) {}
	virtual ~QSideBarItem() {}
};

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

#include "glue.h"

#pragma once

class QBasicBlockLine : public QObject {
	Q_OBJECT

public:
	QBasicBlockLine(const BasicBlockLine& op, QObject* parent = 0);
	virtual ~QBasicBlockLine();

	Q_PROPERTY(QString opcode READ getOpcode NOTIFY opcodeChanged)
	Q_PROPERTY(QString region READ getRegion NOTIFY regionChanged)
	Q_PROPERTY(quint64 offset READ getOffset NOTIFY offsetChanged)
	Q_PROPERTY(QString comment READ getComment NOTIFY commentChanged)
	Q_PROPERTY(QVariantList operandKind READ getOperandKind NOTIFY operandKindChanged)
	Q_PROPERTY(QVariantList operandDisplay READ getOperandDisplay NOTIFY operandDisplayChanged)
	Q_PROPERTY(QVariantList operandAlt READ getOperandAlt NOTIFY operandAltChanged)
	Q_PROPERTY(QVariantList operandData READ getOperandData NOTIFY operandDataChanged)

	QString getOpcode(void) const;
	QString getRegion(void) const;
	quint64 getOffset(void) const;
	QString getComment(void) const;
	QVariantList getOperandKind(void) const;
	QVariantList getOperandDisplay(void) const;
	QVariantList getOperandAlt(void) const;
	QVariantList getOperandData(void) const;

	void replace(const BasicBlockLine& line);

signals:
	void opcodeChanged(void);
	void regionChanged(void);
	void offsetChanged(void);
	void commentChanged(void);
	void operandKindChanged(void);
	void operandDisplayChanged(void);
	void operandAltChanged(void);
	void operandDataChanged(void);

protected:
	QString m_opcode;
	QString m_region;
	quint64 m_offset;
	QString m_comment;
	QVariantList m_operandKind;
	QVariantList m_operandDisplay;
	QVariantList m_operandAlt;
	QVariantList m_operandData;
};

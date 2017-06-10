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

#include "qbasicblockline.h"

QBasicBlockLine::QBasicBlockLine(const BasicBlockLine& line, QObject* parent)
: QObject(parent), m_opcode(line.opcode), m_region(line.region), m_offset(line.offset), m_comment(line.comment)
{
	for(size_t idx = 0; line.args[idx]; ++idx) {
		const BasicBlockOperand* op = line.args[idx];

		m_operandKind.append(QVariant(QString(op->kind)));
		m_operandDisplay.append(QVariant(QString(op->display)));
		m_operandAlt.append(QVariant(QString(op->alt)));
		m_operandData.append(QVariant(QString(op->data)));
	}
}

QBasicBlockLine::~QBasicBlockLine() {}

QString QBasicBlockLine::getOpcode(void) const { return m_opcode; }
QString QBasicBlockLine::getRegion(void) const { return m_region; }
quint64 QBasicBlockLine::getOffset(void) const { return m_offset; }
QString QBasicBlockLine::getComment(void) const { return m_comment; }
QVariantList QBasicBlockLine::getOperandKind(void) const { return m_operandKind; }
QVariantList QBasicBlockLine::getOperandDisplay(void) const { return m_operandDisplay; }
QVariantList QBasicBlockLine::getOperandAlt(void) const { return m_operandAlt; }
QVariantList QBasicBlockLine::getOperandData(void) const { return m_operandData; }

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

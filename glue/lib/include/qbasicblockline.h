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

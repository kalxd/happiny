#include "table.h"
#include <string>
#include <QKeyEvent>
#include <QDebug>

namespace XGApp {
	TableModel::TableModel(QObject *parent) : QAbstractTableModel(parent) {
        this->colors = std::move(XGFFI::readColorItem());
	}

    int TableModel::rowCount(const QModelIndex &idx) const {
        return this->colors.size();
    }

    int TableModel::columnCount(const QModelIndex &idx) const { return 7; }

    QVariant TableModel::headerData(int section, Qt::Orientation orientation,
                                    int role) const {

		if (role != Qt::DisplayRole) {
			return {};
        }

        if (orientation != Qt::Horizontal) {
			return {};
        }

        if (section == 0) {
			return "编号";
        } else if (section == 1) {
			return "名称";
        } else if (section == 2) {
			return "拼音";
        } else if (section == 3) {
			return "明亮亮的颜色";
        } else if (section == 4) {
			return "RGB";
        } else if (section == 5) {
			return "CMYK";
        } else if (section == 6) {
			return "HEX";
        }

        return {};
    }

    QVariant TableModel::data(const QModelIndex &idx, int role) const {
		if (!idx.isValid()) {
			return {};
        }

        const auto &data = this->colors[idx.row()];
		const auto col = idx.column();

        if (role == Qt::DisplayRole) {
            if (col == 0) {
                return data.id;
            } else if (col == 1) {
				return QString::fromStdString((std::string)data.name);
            } else if (col == 2) {
				return QString::fromStdString((std::string)data.pinyin);
            } else if (col == 4) {
				auto const &rgb = data.rgb;
				return QString("rgb(%1, %2, %3)").arg(rgb[0]).arg(rgb[1]).arg(rgb[2]);
            } else if (col == 5) {
				auto const &cmyk = data.cmyk;
				return QString("cmyk(%1, %2, %3, %4)").arg(cmyk[0]).arg(cmyk[1]).arg(cmyk[2]).arg(cmyk[3]);
            } else if (col == 6) {
				return QString::fromStdString(static_cast<std::string>(data.hex));
            }

            return {};
        } else if (role == Qt::BackgroundRole) {
			if (col == 3) {
				const auto &rgb = data.rgb;
				return QColor(rgb[0], rgb[1], rgb[2]);
			}
        }

        return {};
    }

    Table::Table(QWidget *parent) : QTableView(parent) {}

    void Table::keyPressEvent(QKeyEvent *event) {
		if (event->matches(QKeySequence::Copy)) {
			qDebug() << "do this?";
        }

		QTableView::keyPressEvent(event);
    }
}

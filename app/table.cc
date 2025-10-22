#include "table.h"
#include <qnamespace.h>

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

        if (role == Qt::DisplayRole) {
			const auto &data = this->colors[idx.row()];
			const auto col = idx.column();
            if (col == 0) {
                return data.getId();
            }

            return {};
        }

        return {};
    }

	Table::Table(QWidget *parent) : QTableView(parent) {}
}

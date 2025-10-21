#ifndef XGAPP_TABLE
#define XGAPP_TABLE

#include <QAbstractTableModel>
#include "lib.rs.h"

namespace XGApp {
	class TableModel : public QAbstractTableModel {
    private:
        rust::Vec<XGFFI::ColorItem> colors;

    public:
		TableModel(QObject *parent = nullptr);

        int rowCount(const QModelIndex &idx) const;
        int columnCount(const QModelIndex &idx) const;
        QVariant headerData(int section, Qt::Orientation orientation, int role) const;
	};

	class Table {};
}

#endif

#ifndef XGAPP_TABLE
#define XGAPP_TABLE

#include <QAbstractTableModel>
#include <QTableView>
#include "lib.rs.h"

namespace XGApp {
	class TableModel : public QAbstractTableModel {
    private:
        rust::Vec<XGFFI::ColorItem> colors;

    public:
		TableModel(QObject *parent = nullptr);

        int rowCount(const QModelIndex &idx) const override;
        int columnCount(const QModelIndex &idx) const override;
        QVariant headerData(int section, Qt::Orientation orientation,
                            int role) const override;
        QVariant data(const QModelIndex &idx, int role) const override;
	};

    class Table : public QTableView {
    public:
		explicit Table(QWidget *parent = nullptr);

    private:
        void keyPressEvent(QKeyEvent *event) override;
    };
}

#endif

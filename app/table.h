#ifndef XGAPP_TABLE
#define XGAPP_TABLE

#include <QAbstractTableModel>

namespace XGApp {
	class TableCellData {
	};


	class TableModel : public QAbstractTableModel {
    public:
		TableModel(QObject *parent = nullptr);
	};

	class Table {};
}

#endif

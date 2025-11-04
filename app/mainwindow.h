#ifndef XGAPP_MAINWINDOW
#define XGAPP_MAINWINDOW

#include "table.h"
#include "searchline.h"
#include <QSortFilterProxyModel>

namespace XGApp {
	class MainWindow : public QWidget {
    private:
		XGApp::Table *table;
        XGApp::TableModel *model;
        XGWidget::SearchLine *searchLine;
        QSortFilterProxyModel *proxyModel;

    private slots:
        void filterSearch(const QString &word);
    public:
		explicit MainWindow();
	};
}

#endif

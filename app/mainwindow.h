#ifndef XGAPP_MAINWINDOW
#define XGAPP_MAINWINDOW

#include "table.h"
#include <QWidget>
#include <QPushButton>

namespace XGApp {
	class MainWindow : public QWidget {
    private:
		XGApp::Table *table;
		XGApp::TableModel *model;
    public:
		explicit MainWindow();
	};
}

#endif

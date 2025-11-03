#ifndef XGAPP_MAINWINDOW
#define XGAPP_MAINWINDOW

#include "table.h"
#include <QWidget>
#include <QPushButton>
#include <QLineEdit>

namespace XGApp {
	class MainWindow : public QWidget {
    private:
		XGApp::Table *table;
        XGApp::TableModel *model;
        QLineEdit *searchEdit;
    public:
		explicit MainWindow();
	};
}

#endif

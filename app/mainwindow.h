#ifndef XGAPP_MAINWINDOW
#define XGAPP_MAINWINDOW

#include "table.h"
#include "searchline.h"

namespace XGApp {
	class MainWindow : public QWidget {
    private:
		XGApp::Table *table;
        XGApp::TableModel *model;
        XGWidget::SearchLine *searchLine;
    public:
		explicit MainWindow();
	};
}

#endif

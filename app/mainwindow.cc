#include "mainwindow.h"
#include <QFormLayout>

namespace XGApp {
	MainWindow::MainWindow() {
        auto mainLayout = new QVBoxLayout;

        auto searchLayout = new QFormLayout;

        this->searchLine = new XGWidget::SearchLine;
        searchLayout->addRow("搜索", this->searchLine);
        mainLayout->addLayout(searchLayout);

        this->model = new XGApp::TableModel;
        this->table = new XGApp::Table;

        this->table->setModel(this->model);
        mainLayout->addWidget(this->table);

        this->setLayout(mainLayout);
        this->resize(800, 800);
    }
}

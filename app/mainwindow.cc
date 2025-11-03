#include "mainwindow.h"
#include <QBoxLayout>
#include <QFormLayout>
#include <QDebug>

namespace XGApp {
	MainWindow::MainWindow() {
        auto mainLayout = new QVBoxLayout;

        auto searchLayout = new QFormLayout;

        this->searchEdit = new QLineEdit;
        searchLayout->addRow("搜索", this->searchEdit);
        mainLayout->addLayout(searchLayout);

        this->model = new XGApp::TableModel;
        this->table = new XGApp::Table;

        this->table->setModel(this->model);
        mainLayout->addWidget(this->table);

        this->setLayout(mainLayout);
        this->resize(800, 800);
    }
}

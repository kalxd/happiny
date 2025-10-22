#include "mainwindow.h"
#include <QBoxLayout>
#include <QDebug>

namespace XGApp {
	MainWindow::MainWindow() {
        auto mainLayout = new QVBoxLayout;

        this->model = new XGApp::TableModel;
        this->table = new XGApp::Table;

        this->table->setModel(this->model);
        mainLayout->addWidget(this->table);

        this->setLayout(mainLayout);
        this->resize(800, 800);
    }
}

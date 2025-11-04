#include "mainwindow.h"
#include "searchline.h"
#include <QFormLayout>
#include <QDebug>

namespace XGApp {
	MainWindow::MainWindow() {
        auto mainLayout = new QVBoxLayout;

        auto searchLayout = new QFormLayout;

        this->searchLine = new XGWidget::SearchLine;
        connect(this->searchLine, &XGWidget::SearchLine::search, this,
                [](auto word) {
					qDebug() << word;
				});

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

#include "mainwindow.h"
#include "searchline.h"
#include <QFormLayout>
#include <QDebug>

namespace XGApp {
	MainWindow::MainWindow() {
        auto mainLayout = new QVBoxLayout;

        auto searchLayout = new QFormLayout;

        this->searchLine = new XGWidget::SearchLine;

        searchLayout->addRow("搜索拼音", this->searchLine);
        mainLayout->addLayout(searchLayout);

        this->model = new XGApp::TableModel;
        this->proxyModel = new QSortFilterProxyModel(this);
        this->proxyModel->setSourceModel(this->model);
        this->proxyModel->setFilterCaseSensitivity(Qt::CaseInsensitive);
        this->proxyModel->setFilterKeyColumn(2);
        connect(this->searchLine, &XGWidget::SearchLine::search, this->proxyModel, &QSortFilterProxyModel::setFilterWildcard);

        this->table = new XGApp::Table;
        this->table->setModel(this->proxyModel);
        this->table->setSelectionMode(QTableView::SingleSelection);
        mainLayout->addWidget(this->table);

        this->setLayout(mainLayout);
        this->resize(800, 800);
    }
}

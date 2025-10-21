#include "mainwindow.h"
#include <QBoxLayout>
#include <qpushbutton.h>
#include "lib.rs.h"
#include <QDebug>

namespace XGApp {
	MainWindow::MainWindow() {
        auto mainLayout = new QVBoxLayout;

        this->btn = new QPushButton("hello sb");
        mainLayout->addWidget(this->btn);
        connect(this->btn, &QPushButton::clicked, this, &MainWindow::showHello);

        this->setLayout(mainLayout);
        this->resize(800, 800);
    }

    void MainWindow::showHello() const {
		auto colors = XGFFI::readColorItem();
		qDebug() << colors.size();
        for (const auto &item : colors) {
            qDebug() << item.getId();
        }
    }
}

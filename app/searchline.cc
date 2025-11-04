#include "searchline.h"

namespace XGWidget {
	SearchLine::SearchLine(QWidget *parent) : QHBoxLayout(parent) {
        this->searchLine = new QLineEdit;
        this->addWidget(this->searchLine, 1);

        this->searchBtn = new QPushButton("搜索");
        this->addWidget(this->searchBtn);
	}
}

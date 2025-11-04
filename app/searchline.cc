#include "searchline.h"

namespace XGWidget {
	SearchLine::SearchLine(QWidget *parent) : QHBoxLayout(parent) {
        this->searchLine = new QLineEdit;
        connect(this->searchLine, &QLineEdit::returnPressed, this, &SearchLine::startSearch);
        this->addWidget(this->searchLine, 1);

        this->searchBtn = new QPushButton("搜索");
        connect(this->searchBtn, &QPushButton::clicked, this, &SearchLine::startSearch);
        this->addWidget(this->searchBtn);
    }

    void SearchLine::startSearch() {
		auto text = this->searchLine->text().trimmed();
        emit this->search(text);
    }
}

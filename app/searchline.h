#ifndef XGWIDGET_SEARCHLINE
#define XGWIDGET_SEARCHLINE

#include <QLineEdit>
#include <QPushButton>
#include <QHBoxLayout>

namespace XGWidget {
	class SearchLine : public QHBoxLayout {
    private:
		QLineEdit *searchLine;
		QPushButton *searchBtn;

    public:
        explicit SearchLine(QWidget *parent = nullptr);
	};
}

#endif

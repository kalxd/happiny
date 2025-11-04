#ifndef XGWIDGET_SEARCHLINE
#define XGWIDGET_SEARCHLINE

#include <QLineEdit>
#include <QPushButton>
#include <QHBoxLayout>

namespace XGWidget {
	class SearchLine : public QHBoxLayout {
		Q_OBJECT
    private:
		QLineEdit *searchLine;
        QPushButton *searchBtn;

    private slots:
        void startSearch();

    public:
		explicit SearchLine(QWidget *parent = nullptr);

    signals:
        void search(const QString &word);
	};
}

#endif

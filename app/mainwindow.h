#ifndef XGAPP_MAINWINDOW
#define XGAPP_MAINWINDOW

#include <QWidget>
#include <QPushButton>

namespace XGApp {
	class MainWindow : public QWidget {
    private:
        QPushButton *btn;
    public:
		explicit MainWindow();
    public slots:
        void showHello() const;
	};
}

#endif

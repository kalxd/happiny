#include "mainwindow.h"
#include <gtkmm/scrolledwindow.h>

namespace XGApp {
	MainWindow::MainWindow() {
        Gtk::ScrolledWindow sw;

        this->set_child(sw);
        this->set_default_size(800, 800);
	}
}

#include "mainwindow.h"
#include <gtkmm/scrolledwindow.h>
#include <gtkmm/button.h>
#include <gtkmm/box.h>
#include <gtkmm/scrolledwindow.h>
#include "lib.rs.h"

namespace XGApp {
	MainWindow::MainWindow() {

        Gtk::Box main_layout;

        Gtk::Button btn("点击我");
        btn.signal_clicked().connect(sigc::ptr_fun(&say_hello));
        main_layout.append(btn);

        this->set_child(main_layout);
        this->set_default_size(800, 800);
	}
}

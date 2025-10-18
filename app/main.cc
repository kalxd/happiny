#include "mainwindow.h"

int main(int argc, char *argv[]) {
	auto app = Gtk::Application::create("person.xgley.happiny");
	return app->make_window_and_run<XGApp::MainWindow>(argc, argv);
}

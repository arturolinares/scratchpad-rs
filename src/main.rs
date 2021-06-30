#![windows_subsystem = "windows"]
/*!
    A very simple scratchpad that stays un the tray.
*/

pub mod settings;

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::MessageChoice;
use nwg::NativeUi;

use anyhow::Result;
use settings::load_scratchpad_contents;
use settings::preferences_read;
use settings::preferences_save;
use settings::save_scratchpad_contents;

#[derive(Default, NwgUi)]
pub struct MainWindow {
    #[nwg_control(
        size: (
            preferences_read("mw_width", "300").unwrap().parse::<i32>().unwrap_or(300),
            preferences_read("mw_height", "300").unwrap().parse::<i32>().unwrap_or(300)),
        position: (
            preferences_read("mw_x", "300").unwrap().parse::<i32>().unwrap_or(300),
            preferences_read("mw_y", "300").unwrap().parse::<i32>().unwrap_or(300),
        ),
        title: "Scratchpad",
        flags: "WINDOW|RESIZABLE",
        icon: Some(&data.icon),
    )]
    #[nwg_events(
        OnWindowClose: [MainWindow::on_close],
        OnResize: [MainWindow::save_win_pos],
        OnMove: [MainWindow::save_win_pos]
    )]
    window: nwg::Window,

    #[nwg_resource(family: "Segoe UI", size: 18)]
    font: nwg::Font,

    #[nwg_control(text: "", font: Some(&data.font), flags: "VSCROLL|AUTOVSCROLL|VISIBLE|TAB_STOP|SAVE_SELECTION")]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    buffer: nwg::RichTextBox,

    #[nwg_layout(parent: window, spacing: 1, margin:[5,5,25,5], min_size: [300,115])]
    grid: nwg::GridLayout,

    #[nwg_control(parent: window, text: "")]
    status: nwg::StatusBar,

    #[nwg_resource(source_file: Some("./resources/tray.ico"))]
    icon: nwg::Icon,

    // region Tray items
    #[nwg_control(icon: Some(&data.icon), tip: Some("Scratchpad"))]
    #[nwg_events(MousePressLeftUp: [MainWindow::show_editor], OnContextMenu: [MainWindow::show_menu])]
    tray: nwg::TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "Clear")]
    #[nwg_events(OnMenuItemSelected: [MainWindow::clear])]
    tray_item_clear_history: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "Show Editor")]
    #[nwg_events(OnMenuItemSelected: [MainWindow::show_editor])]
    tray_item_show_editor: nwg::MenuItem,

    #[nwg_control(parent: tray_menu)]
    tray_item_separator: nwg::MenuSeparator,

    #[nwg_control(parent: tray_menu, text: "Exit")]
    #[nwg_events(OnMenuItemSelected: [MainWindow::exit])]
    tray_item_exit: nwg::MenuItem,
    // endregion
}

impl MainWindow {
    pub fn on_close(&self) {
        self.status.set_text(0, "Saving contents...");
        let contents = self.buffer.text();

        match save_scratchpad_contents(&contents) {
            Ok(_) => {
                self.status.set_text(0, "Saved");
                self.window.set_visible(false);
            }
            Err(e) => {
                self.status.set_text(0, "Error saving contents!");
                nwg::modal_error_message(
                    &self.window,
                    "Error",
                    &format!("An error ocurred: {}", e),
                );
            }
        };
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn show_editor(&self) {
        let contents = match load_scratchpad_contents() {
            Ok(c) => c,
            Err(_) => String::from(""),
        };
        self.window.set_visible(true);
        self.buffer.set_text(&contents);
        self.buffer.set_focus();
    }

    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn clear(&self) {
        let result = nwg::modal_message(&self.window, &nwg::MessageParams {
            title: "Confirm deletion",
            content: "All the contents will be wiped. Are you sure?",
            buttons: nwg::MessageButtons::OkCancel,
            icons: nwg::MessageIcons::Warning,
        });

        if result == MessageChoice::Ok {
            self.buffer.set_text("");
            save_scratchpad_contents("").unwrap();
        }
    }

    fn save_win_pos(&self) {
        let (x, y) = self.window.position();
        let (width, height) = self.window.size();

        preferences_save("mw_x", &x.to_string()).unwrap();
        preferences_save("mw_y", &y.to_string()).unwrap();
        preferences_save("mw_width", &width.to_string()).unwrap();
        preferences_save("mw_height", &height.to_string()).unwrap();
    }
}

fn main() -> Result<()> {
    nwg::init().expect("Failed to init Native Windows GUI");
    unsafe {
        nwg::set_dpi_awareness();
    }
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = MainWindow::build_ui(Default::default())?;
    nwg::dispatch_thread_events();

    Ok(())
}

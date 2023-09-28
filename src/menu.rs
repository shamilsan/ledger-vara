mod page;

use nanos_sdk::buttons::ButtonEvent;
use nanos_ui::{
    bagls::{LEFT_S_ARROW, RIGHT_S_ARROW},
    layout::Draw,
};

pub use self::page::MenuPage;

pub trait Menu {
    fn page(&self) -> MenuPage;
    fn prev(&mut self);
    fn next(&mut self);
    fn action(&mut self) -> MenuAction;

    fn hide(&self) {
        self.page().hide();
    }

    fn show(&self) {
        self.page().show();
    }

    fn handle_button_event(&mut self, button: ButtonEvent) -> MenuAction {
        match button {
            ButtonEvent::LeftButtonPress => {
                LEFT_S_ARROW.instant_display();
                MenuAction::Nothing
            }
            ButtonEvent::LeftButtonRelease => {
                LEFT_S_ARROW.instant_erase();
                self.prev();
                MenuAction::Nothing
            }
            ButtonEvent::RightButtonPress => {
                RIGHT_S_ARROW.instant_display();
                MenuAction::Nothing
            }
            ButtonEvent::RightButtonRelease => {
                RIGHT_S_ARROW.instant_erase();
                self.next();
                MenuAction::Nothing
            }
            ButtonEvent::BothButtonsPress => {
                LEFT_S_ARROW.instant_display();
                RIGHT_S_ARROW.instant_display();
                MenuAction::Nothing
            }
            ButtonEvent::BothButtonsRelease => {
                LEFT_S_ARROW.instant_erase();
                RIGHT_S_ARROW.instant_erase();
                self.action()
            }
        }
    }
}

pub enum MenuAction {
    Nothing,
    Update,
    Exit,
}
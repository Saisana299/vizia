use vizia::*;

#[derive(Debug, Default, Lens)]
pub struct Options {
    pub option1: bool,
    pub option2: bool,
    pub option3: bool,
}

impl Model for Options {}

#[derive(Debug, Default, Lens)]
pub struct AppData {
    pub options: Options,
    pub exclusive_options: Options,
}

#[derive(Debug)]
pub enum AppEvent {
    ToggleOption(u32),
    ToggleExclusiveOption(u32),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleOption(index) => match *index {
                0 => self.options.option1 ^= true,
                1 => self.options.option2 ^= true,
                2 => self.options.option3 ^= true,
                _ => {}
            },

            AppEvent::ToggleExclusiveOption(index) => match *index {
                0 => {
                    self.exclusive_options.option1 = true;
                    self.exclusive_options.option2 = false;
                    self.exclusive_options.option3 = false;
                }

                1 => {
                    self.exclusive_options.option1 = false;
                    self.exclusive_options.option2 = true;
                    self.exclusive_options.option3 = false;
                }

                2 => {
                    self.exclusive_options.option1 = false;
                    self.exclusive_options.option2 = false;
                    self.exclusive_options.option3 = true;
                }
                _ => {}
            },
        });
    }
}

fn main() {
    Application::new(WindowDescription::new().with_title("Checkbox"), |cx| {
        AppData {
            options: Options { option1: true, option2: false, option3: false },

            exclusive_options: Options { option1: true, option2: false, option3: false },
        }
        .build(cx);

        Label::new(
            cx,
            "A Checkbox represents a boolean state and can trigger an action when toggled.",
        )
        .width(Stretch(1.0))
        .position_type(PositionType::SelfDirected)
        .space(Pixels(10.0));

        HStack::new(cx, |cx| {
            // Checkboxes with labels
            VStack::new(cx, |cx| {
                Label::new(cx, "Checkboxes with labels").class("h1");

                HStack::new(cx, |cx| {
                    //Binding::new(cx, AppData::options.then(Options::option1), |cx, option1| {
                    Checkbox::new(cx, AppData::options.then(Options::option1))
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleOption(0)));
                    //});
                    Label::new(cx, "Option 1");
                })
                .col_between(Pixels(5.0));

                HStack::new(cx, |cx| {
                    //Binding::new(cx, AppData::options.then(Options::option2), |cx, option2| {
                    Checkbox::new(cx, AppData::options.then(Options::option2))
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleOption(1)));
                    //});
                    Label::new(cx, "Option 2");
                })
                .col_between(Pixels(5.0));

                HStack::new(cx, |cx| {
                    //Binding::new(cx, AppData::options.then(Options::option3), |cx, option3| {
                    Checkbox::new(cx, AppData::options.then(Options::option3))
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleOption(2)));
                    //});
                    Label::new(cx, "Option 3");
                })
                .col_between(Pixels(5.0));
            })
            .child_space(Stretch(1.0));

            // Exclusive checkboxes with labels
            // Only one checkbox can be checked at a time and cannot be unchecked
            VStack::new(cx, |cx| {
                Label::new(cx, "Exclusive Check Boxes").class("h1");

                HStack::new(cx, |cx| {
                    Checkbox::new(cx, AppData::exclusive_options.then(Options::option1))
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleExclusiveOption(0)));

                    Label::new(cx, "Option 1");
                })
                .col_between(Pixels(5.0));

                HStack::new(cx, |cx| {
                    Checkbox::new(cx, AppData::exclusive_options.then(Options::option2))
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleExclusiveOption(1)));
                    Label::new(cx, "Option 2");
                })
                .col_between(Pixels(5.0));

                HStack::new(cx, |cx| {
                    Checkbox::new(cx, AppData::exclusive_options.then(Options::option3))
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleExclusiveOption(2)));
                    Label::new(cx, "Option 3");
                })
                .col_between(Pixels(5.0));
            })
            .child_space(Stretch(1.0));
        })
        .child_space(Stretch(1.0));
    })
    .run();
}
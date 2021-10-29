use std::rc::Rc;

use vizia::*;

struct CustomView {
    builder: Option<Rc<dyn Fn(&mut Context)>>,
}

impl CustomView {
    pub fn new<'a,F>(cx: &'a mut Context, f: F) -> Handle<'a, Self> 
    where F: 'static + Fn(&mut Context)
    {
        Self {
            builder: Some(Rc::new(f)),
        }.build(cx)
    }
}



impl View for CustomView {
    fn body(&mut self, cx: &mut Context) 
    {
        if let Some(builder) = self.builder.clone() {
            VStack::new(cx, move |cx| {
                (builder)(cx);
                Label::new(cx, "Three");
                Label::new(cx, "Four");
            });
        }
    }
}

fn main() {

    Application::new(|cx|{
        CustomView::new(cx, |cx|{
            VStack::new(cx, |cx| {
                Label::new(cx, "One");
                Label::new(cx, "Two");
            });
        });
    }).run();
}
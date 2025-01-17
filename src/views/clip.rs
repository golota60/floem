use kurbo::Size;

use crate::{
    id::Id,
    view::{View, ViewData},
};

pub struct Clip {
    data: ViewData,
    child: Box<dyn View>,
}

pub fn clip<V: View + 'static>(child: V) -> Clip {
    Clip {
        data: ViewData::new(Id::next()),
        child: Box::new(child),
    }
}

impl View for Clip {
    fn view_data(&self) -> &ViewData {
        &self.data
    }

    fn view_data_mut(&mut self) -> &mut ViewData {
        &mut self.data
    }

    fn for_each_child<'a>(&'a self, for_each: &mut dyn FnMut(&'a dyn View) -> bool) {
        for_each(&self.child);
    }

    fn for_each_child_mut<'a>(&'a mut self, for_each: &mut dyn FnMut(&'a mut dyn View) -> bool) {
        for_each(&mut self.child);
    }

    fn for_each_child_rev_mut<'a>(
        &'a mut self,
        for_each: &mut dyn FnMut(&'a mut dyn View) -> bool,
    ) {
        for_each(&mut self.child);
    }

    fn debug_name(&self) -> std::borrow::Cow<'static, str> {
        "Clip".into()
    }

    fn paint(&mut self, cx: &mut crate::context::PaintCx) {
        cx.save();
        let style = cx.get_builtin_style(self.id());
        let border_radius = style.border_radius();
        let size = cx
            .get_layout(self.id())
            .map(|layout| Size::new(layout.size.width as f64, layout.size.height as f64))
            .unwrap_or_default();

        let radius = match border_radius {
            crate::unit::PxPct::Px(px) => px,
            crate::unit::PxPct::Pct(pct) => size.min_side() * (pct / 100.),
        };
        if radius > 0.0 {
            let rect = size.to_rect().to_rounded_rect(radius);
            cx.clip(&rect);
        } else {
            cx.clip(&size.to_rect());
        }
        cx.paint_view(&mut self.child);
        cx.restore();
    }
}

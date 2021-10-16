use druid::{
    widget::{ClipBox, Label, LabelText},
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Point, Rect, Size, UnitPoint, UpdateCtx, Widget, WidgetPod,
};

pub struct RightAlignedLabel<T> {
    clip_box: WidgetPod<T, ClipBox<T, Label<T>>>,
}

impl<T: Data> RightAlignedLabel<T> {
    pub fn new(label: Label<T>) -> Self {
        let clip_box = WidgetPod::new(ClipBox::new(label));

        RightAlignedLabel { clip_box }
    }
}

impl<T: Data> Widget<T> for RightAlignedLabel<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.clip_box.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.clip_box.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.clip_box.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.clip_box.layout(ctx, &bc.loosen(), data, env);

        // Set the clipbox viewport
        let clipbox = self.clip_box.widget_mut();

        let label_size = clipbox.content_size();
        let viewport_size = clipbox.viewport_size();

        let x = if label_size.width > viewport_size.width {
            label_size.width - viewport_size.width
        } else {
            0.
        };
        let y = 0.;

        let view_point_origin = Point::new(x, y);
        clipbox.pan_to(view_point_origin);

        let size = self.clip_box.layout(ctx, &bc.loosen(), data, env);

        // Right align the clipbox
        let mut my_size = size;
        if bc.is_width_bounded() {
            my_size.width = bc.max().width;
        }
        if bc.is_height_bounded() {
            my_size.height = bc.max().height;
        }

        my_size = bc.constrain(my_size);
        let extra_width = (my_size.width - size.width).max(0.);
        let extra_height = (my_size.height - size.height).max(0.);
        let origin = UnitPoint::RIGHT
            .resolve(Rect::new(0., 0., extra_width, extra_height))
            .expand();
        self.clip_box.set_origin(ctx, data, env, origin);

        let my_insets = self.clip_box.compute_parent_paint_insets(my_size);
        ctx.set_paint_insets(my_insets);

        my_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.clip_box.paint(ctx, data, env);
    }
}

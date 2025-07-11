use core::time::Duration;

use crate::{
    environment::DefaultEnvironment,
    event::Event,
    primitives::Point,
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::RenderTarget,
    view::View,
};

#[derive(Debug)]
pub struct App<V, ViewFn, Tree, AppData, ViewState, Target> {
    pub view: V,
    pub view_fn: ViewFn,
    pub view_state: ViewState,
    pub data: AppData,
    pub target: Target,
    pub source_tree: Tree,
    pub target_tree: Tree,
}

impl<Color, V, ViewFn, AppData, Target> App<V, ViewFn, V::Renderables, AppData, V::State, Target>
where
    ViewFn: Fn(&AppData) -> V,
    V: View<Color, AppData>,
    Target: RenderTarget<ColorFormat = Color>,
    V::Renderables: Clone,
{
    pub fn new(
        view_fn: ViewFn,
        target: Target,
        mut data: AppData,
        time: core::time::Duration,
    ) -> Self {
        let env = DefaultEnvironment::new(time);
        let view = (view_fn)(&data);
        let mut view_state = view.build_state(&mut data);
        let layout = view.layout(&target.size().into(), &env, &mut data, &mut view_state);
        let source_tree =
            view.render_tree(&layout, Point::zero(), &env, &mut data, &mut view_state);
        let target_tree =
            view.render_tree(&layout, Point::zero(), &env, &mut data, &mut view_state);
        Self {
            view,
            view_fn,
            view_state,
            data,
            target,
            source_tree,
            target_tree,
        }
    }

    pub fn handle_events(
        &mut self,
        time: Duration,
        events: impl IntoIterator<Item = Event>,
    ) -> bool {
        let domain = AnimationDomain::top_level(time);
        let mut joined_tree =
            AnimatedJoin::join(self.source_tree.clone(), self.target_tree.clone(), &domain);
        let mut handled = false;
        for event in events {
            if event == Event::Exit {
                return true;
            }
            {
                extern crate std;
                std::println!("Event: {event:?}");
            }
            handled = handled
                || self.view.handle_event(
                    &event,
                    &mut joined_tree,
                    &mut self.data,
                    &mut self.view_state,
                );
        }
        self.source_tree = joined_tree;
        let env = DefaultEnvironment::new(time);
        self.view = (self.view_fn)(&self.data);
        let layout = self.view.layout(
            &self.target.size().into(),
            &env,
            &mut self.data,
            &mut self.view_state,
        );
        self.target_tree = self.view.render_tree(
            &layout,
            Point::zero(),
            &env,
            &mut self.data,
            &mut self.view_state,
        );
        false
    }

    pub fn render(&mut self, time: Duration, default_color: &Color) {
        Render::<Color>::render_animated(
            &mut self.target,
            &self.source_tree,
            &self.target_tree,
            default_color,
            Point::zero(),
            &AnimationDomain::top_level(time),
        );
    }
}

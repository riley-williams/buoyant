use core::time::Duration;

use crate::{
    environment::DefaultEnvironment,
    event::{Event, EventContext, EventResult},
    primitives::Point,
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::RenderTarget,
    view::{View, ViewLayout},
};

/// This is an experimental render loop
pub async fn render_loop<C, V, F, R, P, T, D, TimeFn>(
    target: &mut T,
    mut app_data: D,
    default_style: C,
    app_time: TimeFn,
    mut view_fn: F,
    mut flush_fn: R,
    mut poll_fn: P,
) where
    V: View<C, D>,
    F: FnMut(&mut D, Duration, Duration) -> V,
    R: AsyncFnMut(&mut T, Duration, Duration),
    P: for<'a> AsyncFnMut(&mut EventHandler<'a, V, D, TimeFn>),
    T: RenderTarget<ColorFormat = C>,
    TimeFn: Fn() -> Duration,
{
    let time = app_time();
    let mut render_duration = Duration::from_millis(1);
    let mut flush_duration = Duration::from_millis(1);
    let mut view = view_fn(&mut app_data, render_duration, flush_duration);
    let mut state = view.build_state(&mut app_data);
    let env = DefaultEnvironment::new(time);
    let layout = view.layout(&target.size().into(), &env, &mut app_data, &mut state);
    let mut source_tree =
        view.render_tree(&layout, Point::default(), &env, &mut app_data, &mut state);
    let mut target_tree =
        view.render_tree(&layout, Point::default(), &env, &mut app_data, &mut state);

    let mut source_tree = &mut source_tree;
    let mut target_tree = &mut target_tree;
    loop {
        let time = app_time();
        // render
        let domain = AnimationDomain::top_level(time);
        Render::render_animated(
            target,
            source_tree,
            target_tree,
            &default_style,
            Point::zero(),
            &domain,
        );
        render_duration = app_time() - time;

        let view_size = target.size();
        let mut should_exit = false;

        let mut events = async || {
            // Immediately attempt to yield to allow flush task to start
            // embassy_futures::yield_now().await;
            let time = app_time();
            let mut event_handler = EventHandler {
                view: &mut view,
                source_tree,
                target_tree,
                state: &mut state,
                app_data: &mut app_data,
                app_time: &app_time,
                result: EventResult::default(),
                should_exit: false,
            };
            poll_fn(&mut event_handler).await;

            should_exit = event_handler.should_exit;

            if event_handler.result.recompute_view {
                target_tree.join_from(source_tree, &domain);
                // Swap the references
                core::mem::swap(&mut source_tree, &mut target_tree);
                let env = DefaultEnvironment::new(time);
                view = view_fn(&mut app_data, render_duration, flush_duration);
                let layout = view.layout(&view_size.into(), &env, &mut app_data, &mut state);
                *target_tree =
                    view.render_tree(&layout, Point::default(), &env, &mut app_data, &mut state);
            }
        };
        // wait for display transfer to finish
        let flush_start = app_time();
        embassy_futures::join::join(flush_fn(target, render_duration, flush_duration), events())
            .await;
        flush_duration = app_time() - flush_start;
        if should_exit {
            break;
        }
    }
}

#[derive(Debug)]
pub struct EventHandler<'a, V: ViewLayout<D>, D, TimeFn: Fn() -> Duration> {
    view: &'a mut V,
    #[expect(unused)]
    source_tree: &'a mut V::Renderables,
    target_tree: &'a mut V::Renderables,
    state: &'a mut V::State,
    pub app_data: &'a mut D,
    app_time: &'a TimeFn,
    result: EventResult,
    should_exit: bool,
}

impl<V: ViewLayout<D>, D, TimeFn: Fn() -> Duration> EventHandler<'_, V, D, TimeFn> {
    pub fn handle_event(&mut self, event: &Event) {
        if *event == Event::Exit {
            self.should_exit = true;
            return;
        }
        if *event == Event::External {
            self.result.recompute_view = true;
            return;
        }
        let context = EventContext::new((self.app_time)());
        let result =
            self.view
                .handle_event(event, &context, self.target_tree, self.app_data, self.state);
        self.result.merge(result);
    }
}

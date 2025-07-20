use core::{ time::Duration};

use crate::{
    environment::DefaultEnvironment,
    event::Event,
    primitives::Point,
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::RenderTarget,
    view::{View, ViewLayout},
};

pub async fn render_loop<C, V, F, R, P, T, D>(
    target: &mut T,
    mut app_data: D,
    default_style: C,
    app_time: impl Fn() -> Duration,
    view_fn: F,
    mut flush_fn: R,
    mut poll_fn: P,
) where
    V: View<C, D>,
    F: Fn(&mut D) -> V,
    R: AsyncFnMut(&mut T),
    P: for<'a> AsyncFnMut(&mut EventHandler<'a, V, D>),
    T: RenderTarget<ColorFormat = C>,
{
    let time = app_time();
    let mut view = view_fn(&mut app_data);
    let mut state = view.build_state(&mut app_data);
    let env = DefaultEnvironment::new(time);
    let layout = view.layout(&target.size().into(), &env, &mut app_data, &mut state);
    let mut source_tree =
        view.render_tree(&layout, Point::default(), &env, &mut app_data, &mut state);
    let mut target_tree =
        view.render_tree(&layout, Point::default(), &env, &mut app_data, &mut state);

    loop {
        let time = app_time();
        // render
        let domain = AnimationDomain::top_level(time);
        Render::render_animated(
            target,
            &source_tree,
            &target_tree,
            &default_style,
            Point::zero(),
            &domain,
        );

        let view_size = target.size();
        let should_exit = &mut false;

        let mut events = async || {
            // Immediately attempt to yield to allow flush task to start
            embassy_futures::yield_now().await;
            let time = app_time();
            let mut event_handler = EventHandler {
                view: &mut view,
                _source_tree: &mut source_tree,
                target_tree: &mut target_tree,
                state: &mut state,
                app_data: &mut app_data,
                redraw: false,
                should_exit: false,
            };
            poll_fn(&mut event_handler).await;

            *should_exit = event_handler.should_exit;

            if event_handler.redraw {
                target_tree.join_from(&source_tree, &domain);
                source_tree = target_tree.clone(); // FIXME: A/B tree swap
                let env = DefaultEnvironment::new(time);
                view = view_fn(&mut app_data);
                let layout = view.layout(&view_size.into(), &env, &mut app_data, &mut state);
                target_tree =
                    view.render_tree(&layout, Point::default(), &env, &mut app_data, &mut state);
            }
        };
        // wait for display transfer to finish
        embassy_futures::join::join(flush_fn(target), events()).await;
        if *should_exit {
            break;
        }
    }
}

#[derive(Debug)]
pub struct EventHandler<'a, V: ViewLayout<D>, D> {
    view: &'a mut V,
    _source_tree: &'a mut V::Renderables,
    target_tree: &'a mut V::Renderables,
    state: &'a mut V::State,
    app_data: &'a mut D,
    redraw: bool,
    should_exit: bool,
}

impl<V: ViewLayout<D>, D> EventHandler<'_, V, D> {
    pub fn handle_event(&mut self, event: &Event) {
        if *event == Event::Exit {
            self.should_exit = true;
            return;
        }
        let needs_redraw =
            self.view
                .handle_event(event, self.target_tree, self.app_data, self.state);
        self.redraw = self.redraw || needs_redraw;
    }
}

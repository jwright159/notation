use std::any::Any;
use std::marker::PhantomData;
use std::sync::Arc;

use anyhow::{bail, Result};
use bevy::prelude::*;

use crate::prelude::{LayoutAnchor, LayoutConstraint, LayoutData, LayoutSize};

#[derive(Debug)]
pub struct ViewEntity<TE, T>
where
    TE: LayoutEnv,
    T: View<TE>,
{
    env: PhantomData<TE>,
    pub entity: Entity,
    pub view: Arc<T>,
}

#[derive(Debug)]
pub struct DoLayoutEvent<TE, T>
where
    TE: LayoutEnv,
    T: View<TE>,
{
    env: PhantomData<TE>,
    pub entity: Entity,
    pub view: Arc<T>,
    pub layout: LayoutData,
}
impl<TE, T> DoLayoutEvent<TE, T>
where
    TE: LayoutEnv + Send + Sync + 'static,
    T: View<TE>,
{
    pub fn new(entity: Entity, view: &Arc<T>, layout: &LayoutData) -> Self {
        Self {
            env: PhantomData,
            entity,
            view: view.clone(),
            layout: layout.clone(),
        }
    }
    pub fn on_layout_changed(query: LayoutChangedQuery<T>, mut evts: EventWriter<Self>) {
        for (entity, view, layout) in query.iter() {
            if layout.size.width > 0.0 && layout.size.height > 0.0 {
                if view.log_layout_changed() {
                    println!(
                        "<{}>::on_layout_changed({:#?})",
                        std::any::type_name::<T>(),
                        layout
                    );
                }
                evts.send(Self::new(entity, view, layout))
            }
        }
    }
    pub fn setup(app: &mut AppBuilder) {
        app.add_event::<Self>();
        app.add_system(Self::on_layout_changed.system());
    }
}

pub type LayoutQuery<'w, 'd, 't> = Query<'w, (&'d mut LayoutData, &'t mut Transform)>;
pub type LayoutChangedQuery<'w, 'v, 'd, T> =
    Query<'w, (Entity, &'v Arc<T>, &'d LayoutData), Changed<LayoutData>>;
pub type LayoutChangedWithChildrenQuery<'w, 'v, 'd, 'c, T> =
    Query<'w, (Entity, &'v Arc<T>, &'d LayoutData, &'c Children), Changed<LayoutData>>;
pub type ViewQuery<'w, 'p, 'v, T> = Query<'w, (&'p Parent, Entity, &'v Arc<T>)>;
pub type ViewAddedQuery<'w, 'p, 'v, T> = Query<'w, (&'p Parent, Entity, &'v Arc<T>), Added<Arc<T>>>;
pub type ViewRootQuery<'w, 'v, T> = Query<'w, (Entity, &'v Arc<T>)>;
pub type ViewRootAddedQuery<'w, 'v, T> = Query<'w, (Entity, &'v Arc<T>), Added<Arc<T>>>;

pub trait LayoutEnv {
    fn query_child<TE, T>(
        &self,
        view_query: &ViewQuery<T>,
        entity: Entity,
    ) -> Result<ViewEntity<TE, T>>
    where
        TE: LayoutEnv,
        T: View<TE>,
    {
        for (parent, child, view) in view_query.iter() {
            if parent.0 == entity {
                return Ok(ViewEntity {
                    env: PhantomData,
                    entity: child,
                    view: view.clone(),
                });
            }
        }
        bail!("View Not Found: <{}>", std::any::type_name::<T>());
    }
    fn get_child<TE, T>(
        &self,
        view_query: &ViewQuery<T>,
        entity: Entity,
    ) -> Option<ViewEntity<TE, T>>
    where
        TE: LayoutEnv,
        T: View<TE>,
    {
        let result = self.query_child(view_query, entity);
        if let Err(err) = result {
            println!(
                "<LayoutEnv>.get_child<{}>() Not Found: {:?}",
                std::any::type_name::<T>(),
                err
            );
            return None;
        }
        result.ok()
    }
    fn get_children<TE, T>(
        &self,
        view_query: &ViewQuery<T>,
        entity: Entity,
    ) -> Vec<ViewEntity<TE, T>>
    where
        TE: LayoutEnv,
        T: View<TE>,
    {
        let mut children = Vec::new();
        for (parent, child, view) in view_query.iter() {
            if parent.0 == entity {
                children.push(ViewEntity {
                    env: PhantomData,
                    entity: child,
                    view: view.clone(),
                })
            }
        }
        children
    }
}

pub trait View<TE: LayoutEnv>: Any + Send + Sync + ToString + 'static {
    fn is_root(&self) -> bool {
        false
    }
    fn log_set_layout(&self) -> bool {
        false
    }
    fn log_layout_changed(&self) -> bool {
        false
    }
    fn pivot(&self) -> LayoutAnchor {
        if self.is_root() {
            LayoutAnchor::ROOT
        } else {
            LayoutAnchor::default()
        }
    }
    #[allow(unused_variables)]
    fn calc_size(&self, engine: &TE, constraint: LayoutConstraint) -> LayoutSize {
        constraint.max
    }
    fn calc_root_layout(&self, engine: &TE, constraint: LayoutConstraint) -> LayoutData {
        let size = self.calc_size(engine, constraint);
        let pivot = LayoutAnchor::ROOT;
        LayoutData::new(0, size, pivot, pivot, Vec2::ZERO)
    }
    fn set_layout_data(&self, layout_query: &mut LayoutQuery, entity: Entity, data: LayoutData) {
        if self.is_root() {
            println!("Should NOT call set_layout_data() for root views! {}", data);
            return;
        }
        let pivot = self.pivot();
        let need_adjust = pivot != data.pivot;
        let adjusted = if need_adjust {
            data.change_pivot(pivot)
        } else {
            data
        };
        if self.log_set_layout() {
            if need_adjust {
                println!(
                    "{}.set_layout_data(\n\t{} {} ->\n\t{}\n)",
                    self.to_string(),
                    data.pivot,
                    data.offset,
                    adjusted
                );
            } else {
                println!("{}.set_layout_data(\n\t{}\n)", self.to_string(), adjusted);
            }
        }
        match layout_query.get_mut(entity) {
            Ok((mut layout_data, mut transform)) => {
                *layout_data = adjusted;
                *transform = adjusted.transform();
            }
            Err(err) => {
                println!(
                    "{}.set_layout_data() Query Failed: {:?}",
                    self.to_string(),
                    err
                );
            }
        }
    }
}

impl<TE: LayoutEnv, T: View<TE>> ViewEntity<TE, T> {
    pub fn set_layout_data(&self, layout_query: &mut LayoutQuery, data: LayoutData) {
        self.view.set_layout_data(layout_query, self.entity, data)
    }
}
use std::{marker::PhantomData, sync::Arc};

use bevy::prelude::*;
use anyhow::{bail, Result};

use crate::prelude::{LayoutAnchor, LayoutConstraint, LayoutData, LayoutSize};

#[derive(Debug)]
pub struct ViewEntity<TE, T> where TE: LayoutEnv, T: View<TE> {
    env: PhantomData<TE>,
    pub entity: Entity,
    pub view: Arc<T>,
}

pub type LayoutQuery<'w, 'd, 't> = Query<'w, (&'d mut LayoutData, &'t mut Transform)>;
pub type ViewQuery<'w, 'p, 'v, T> = Query<'w, (&'p Parent, Entity, &'v Arc<T>)>;
pub type ViewRootQuery<'w, 'v, T> = Query<'w, (Entity, &'v Arc<T>)>;
pub type ViewRootAddedQuery<'w, 'v, T> = Query<'w, (Entity, &'v Arc<T>), Added<Arc<T>>>;

pub trait LayoutEnv {
    fn query_child<TE, T>(&self,
        view_query: &ViewQuery<T>,
        entity: Entity,
    ) -> Result<ViewEntity<TE, T>> where TE: LayoutEnv, T: View<TE> {
        for (parent, child, view) in view_query.iter() {
            if parent.0 == entity {
                    return Ok(ViewEntity{
                        env: PhantomData,
                        entity: child,
                        view: view.clone(),
                    })
            }
        }
        bail!("View Not Found: <{}>", stringify!(T));
    }
    fn get_child<TE, T>(&self,
        view_query: &ViewQuery<T>,
        entity: Entity,
    ) -> Option<ViewEntity<TE, T>> where TE: LayoutEnv, T: View<TE> {
        let result = self.query_child(view_query, entity);
        if let Err(err) = result {
            println!("<LayoutEnv>.get_child<{}>() Not Found: {:?}", stringify!(T), err);
            return None;
        }
        result.ok()
    }
    /*
    fn query_get<'w, Q: WorldQuery>(&self, world: &'w mut World, entity: Entity
    ) -> Result<<Q::Fetch as Fetch<'w>>::Item, QueryEntityError>
    where <Q as WorldQuery>::Fetch: ReadOnlyFetch;
    fn query_child<'w, T>(&self, world: &'w mut World, entity: Entity) -> Result<ViewEntity<T>> where T: Send + Sync + 'static {
        let _children = self.query_get::<&Children>(world, entity)?;
        let children: Vec<Entity> = _children.iter().map(|x| x.clone()).collect();
        for child in children.iter() {
            if let Ok((entity, view))
                = self.query_get::<(Entity, &Arc<T>)>(world, *child) {
                    return Ok(ViewEntity{
                        entity,
                        view: view.clone(),
                    })
            }
        }
        bail!("Not Found: [{}]", children.len());
    }
     */
}

pub trait View<TE: LayoutEnv>: Send + Sync + ToString + 'static {
    fn pivot(&self) -> LayoutAnchor {
        LayoutAnchor::default()
    }
    fn calc_size(&self, _engine: &TE, constraint: LayoutConstraint) -> LayoutSize {
        constraint.max
    }
    fn calc_root_layout(&self, engine: &TE, constraint: LayoutConstraint, pivot: LayoutAnchor) -> LayoutData {
        let size = self.calc_size(engine, constraint);
        LayoutData::new(0, pivot, LayoutAnchor::default(), Vec2::ZERO, size)
    }
    fn set_layout_data(&self, layout_query: &mut LayoutQuery, entity: Entity, data: LayoutData) {
        let pivot = self.pivot();
        let need_adjust = pivot != data.pivot;
        let adjusted = if need_adjust {
            data.change_pivot(pivot)
        } else {
            data
        };
        if need_adjust {
            println!("{}.set_layout_data(\n\t{} {} ->\n\t{:?}\n)", self.to_string(),
                data.pivot, data.offset,
                adjusted);
        } else {
            println!("{}.set_layout_data(\n\t{:?}\n)", self.to_string(), adjusted);
        }
        match layout_query.get_mut(entity) {
            Ok((mut layout_data, mut transform)) => {
                *layout_data = adjusted;
                *transform = adjusted.transform();
            },
            Err(err) => {
                println!("{}.set_layout_data() Query Failed: {:?}", self.to_string(), err);
            },
        }
    }
}

impl<TE: LayoutEnv, T: View<TE>> ViewEntity<TE, T> {
    pub fn set_layout_data(&self, layout_query: &mut LayoutQuery, data: LayoutData) {
        self.view.set_layout_data(layout_query, self.entity, data)
    }
}

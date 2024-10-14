use std::sync::Arc;

use fj_math::Transform;

use crate::{
    geometry::SurfaceGeom, operations::insert::Insert, storage::Handle,
    topology::Surface, Core,
};

use super::{TransformCache, TransformObject};

impl TransformObject for &Handle<Surface> {
    type Transformed = Handle<Surface>;

    fn transform_with_cache(
        self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self::Transformed {
        cache
            .entry(self)
            .or_insert_with(|| {
                let surface = Surface::new().insert(core);

                let geometry =
                    core.layers.geometry.of_surface(self).transform(transform);
                core.layers
                    .geometry
                    .define_surface(surface.clone(), geometry);
                core.layers.geometry.define_surface_2(
                    surface.clone(),
                    SurfaceGeom {
                        geometry: Arc::new(geometry),
                    },
                );

                surface
            })
            .clone()
    }
}

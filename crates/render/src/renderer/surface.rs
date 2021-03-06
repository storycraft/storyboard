use std::fmt::Debug;

use storyboard_core::observable::Observable;
use wgpu::{
    self, Color, CommandBuffer, CommandEncoderDescriptor, LoadOp, Operations, PresentMode,
    RenderPassColorAttachment, Surface, SurfaceTexture, TextureUsages, TextureViewDescriptor,
};

use crate::{component::Drawable, shared::RenderScope};

use super::{ScreenRect, StoryboardRenderer};

#[derive(Debug)]
pub struct StoryboardSurfaceRenderer {
    surface: Surface,
    configuration: Observable<SurfaceConfiguration>,

    renderer: StoryboardRenderer,
}

impl StoryboardSurfaceRenderer {
    pub fn new(surface: Surface, configuration: SurfaceConfiguration) -> Self {
        let renderer = StoryboardRenderer::new();

        Self {
            surface,
            configuration: configuration.into(),
            renderer,
        }
    }

    pub fn configuration(&self) -> SurfaceConfiguration {
        *self.configuration
    }

    pub fn set_configuration(&mut self, configuration: SurfaceConfiguration) {
        if self.configuration.ne(&configuration) {
            self.configuration = configuration.into();
        }
    }

    pub fn render<'a>(
        &mut self,
        scope: RenderScope,
        drawables: impl ExactSizeIterator<Item = &'a dyn Drawable>,
    ) -> Option<SurfaceRenderResult> {
        let backend = scope.backend();

        if Observable::invalidate(&mut self.configuration)
            && self.configuration.screen.rect.size.area() > 0
        {
            self.surface.configure(
                backend.device(),
                &wgpu::SurfaceConfiguration {
                    usage: TextureUsages::RENDER_ATTACHMENT,
                    format: scope.pipeline().texture_format,
                    width: self.configuration.screen.rect.size.width,
                    height: self.configuration.screen.rect.size.height,
                    present_mode: self.configuration.present_mode,
                },
            );
        }

        if let Ok(surface_texture) = self.surface.get_current_texture() {
            let mut encoder = backend
                .device()
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("StoryboardSurfaceRenderer command encoder"),
                });

            self.renderer.render(
                scope,
                self.configuration.screen,
                drawables,
                Some(RenderPassColorAttachment {
                    view: &surface_texture
                        .texture
                        .create_view(&TextureViewDescriptor::default()),
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: true,
                    },
                }),
                &mut encoder,
            );

            return Some(SurfaceRenderResult {
                surface_texture,
                command_buffer: encoder.finish(),
            });
        }

        None
    }

    pub fn into_inner(self) -> Surface {
        self.surface
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceConfiguration {
    pub present_mode: PresentMode,
    pub screen: ScreenRect,
}

#[derive(Debug)]
pub struct SurfaceRenderResult {
    pub surface_texture: SurfaceTexture,
    pub command_buffer: CommandBuffer,
}

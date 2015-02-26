use glium::*;
use render::Draw;
use std::marker;

pub trait Stage<T, U> {
    fn update(&mut self, dt: f32);
    fn render(&mut self, dt: f32);
    fn get_vertex_buffer(&self) -> VertexBuffer<T>;
    fn get_index_buffer(&self) -> IndexBuffer;
    fn get_program(&self) -> Program;
    fn get_uniforms(&self) -> U;
    fn get_draw_params(&self) -> DrawParameters;
}

pub struct StageContainer<V, U, T: Stage<V, U>> {
    pub obj: T,
    marker1: marker::PhantomData<U>,
    marker2: marker::PhantomData<V>,
}

impl <V, U, T: Stage<V, U>> StageContainer <V, U, T> {
    pub fn new(obj: T) -> StageContainer<V, U, T> {
        StageContainer {
            obj: obj,
            marker1: marker::PhantomData,
            marker2: marker::PhantomData,
        }
    }

    #[allow(unused_variables)]
    pub fn update(&mut self, dt: f32) {
    // Here T must be mutable so we can do:
    // self.obj.update(dt);
    }
}

impl <V: vertex::Vertex, U: uniforms::Uniforms + Copy, T: Stage<V, U>> Draw for StageContainer<V, U, T> {
    #[inline]
    #[allow(unused_variables)]
    fn draw(&mut self, frame: &mut Frame, dt: f32) {
        let obj = &mut self.obj;
        obj.render(dt);
        frame.draw(
            &obj.get_vertex_buffer(),
            &obj.get_index_buffer(),
            &obj.get_program(),
            &obj.get_uniforms(),
            &obj.get_draw_params()
        ).unwrap();
    }
}

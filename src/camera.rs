use std::collections::HashMap;

use winit::{event::{WindowEvent, ElementState, KeyEvent, MouseScrollDelta, DeviceId, MouseButton, TouchPhase}, keyboard::{PhysicalKey, KeyCode}, dpi::PhysicalPosition};
use cgmath::{prelude::*, Matrix4, Vector2};

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub light: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        Camera {
            eye: (0.0, 5.0, 10.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            light: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: aspect_ratio,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        proj * view
    }

    pub fn update_aspect_ratio(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }
}


#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = (OPENGL_TO_WGPU_MATRIX * camera.build_view_projection_matrix()).into();
    }
}

struct CursorData {
    pos: Option<Vector2<f32>>,
    pressed: bool,
}

pub struct CameraController {
    pub key_speed: f32,
    pub zoom_speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    movement: Vector2<f32>,
    zoom: f32,
    cursors: HashMap<DeviceId, CursorData>,
    touchs: HashMap<u64, Vector2<f32>>,
}

impl CameraController {
    pub fn new(key_speed: f32) -> Self {
        Self {
            key_speed,
            zoom_speed: 3.0,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            zoom: 1.0,
            movement: Vector2::zero(),
            cursors: HashMap::new(),
            touchs: HashMap::new(),
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        let var_name = match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(keycode),
                    state,
                    ..
                },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    KeyCode::KeyW | KeyCode::ArrowUp => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyA | KeyCode::ArrowLeft => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyS | KeyCode::ArrowDown => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyD | KeyCode::ArrowRight => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            },
            WindowEvent::MouseWheel { delta, .. } => {
                let speed = match delta {
                    MouseScrollDelta::LineDelta(_x, y) => *y,
                    MouseScrollDelta::PixelDelta(x) => x.y as _,
                };
                self.zoom *= (1.1_f32).powf(-speed);
                true
            },
            WindowEvent::MouseInput { device_id, state, button } => {
                if *button == MouseButton::Left {
                    if let Some(cursor) = self.cursors.get_mut(device_id) {
                        cursor.pressed = *state == ElementState::Pressed;
                        cursor.pos = None;
                    }
                    true
                } else { false }
            },
            WindowEvent::CursorEntered { device_id } => {
                self.cursors.insert(*device_id, CursorData { pos: None, pressed: false });
                true
            },
            WindowEvent::CursorLeft { device_id } => {
                self.cursors.remove(device_id);
                true
            }
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => {
                let cdata = match self.cursors.get_mut(device_id) {
                    Some(x) if x.pressed => x,
                    _ => return false
                };

                let new_pos = position.to_cgmath();
                let diff = cdata.pos.map(|pos| new_pos - pos);
                cdata.pos = Some(new_pos);
                if let Some(diff) = diff {
                    self.movement += diff * 0.2;
                }
                true
            }
            WindowEvent::Touch(touch) => {
                match touch.phase {
                    TouchPhase::Started => {
                        self.touchs.insert(touch.id, touch.location.to_cgmath());
                    }
                    TouchPhase::Moved => {
                        let stored_pos = match self.touchs.get_mut(&touch.id) {
                            Some(x) => x,
                            None => return false,
                        };
                        let last_pos = *stored_pos;
                        let curr_pos  = touch.location.to_cgmath();
                        *stored_pos = curr_pos;
                        if self.touchs.len() == 1 {
                            let diff = curr_pos - last_pos;
                            self.movement += diff * 0.1;
                        } else if self.touchs.len() == 2 {
                            let other_pos = *self.touchs.iter()
                                .filter(|x| *x.0 != touch.id)
                                .next()
                                .unwrap().1;

                            let old_dist = last_pos.distance(other_pos);
                            let new_dist = curr_pos.distance(other_pos);
                            self.zoom *= old_dist / new_dist;
                        }
                    }
                    TouchPhase::Ended | TouchPhase::Cancelled => {
                        self.touchs.remove(&touch.id);
                    }
                }
                true
            }
            _ => false,
        };
        var_name
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        let forward = (camera.target - camera.eye).normalize();
        if self.zoom != 1.0 {
            let distance = camera.target.distance(camera.eye);
            camera.eye = camera.target + (camera.eye - camera.target).normalize() * distance * self.zoom;
            self.zoom = 1.0;
        }


        if self.is_forward_pressed {
            camera.eye += forward * self.key_speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward * self.key_speed;
        }

        // Ok, zoom adjusted, now we need to rotate the model
        let right = forward.cross(camera.up);
        let distance = camera.target.distance(camera.eye);

        if self.is_right_pressed {
            camera.eye += right * self.key_speed;
        }
        if self.is_left_pressed {
            camera.eye -= right * self.key_speed;
        }

        camera.eye -= right * self.movement.x as f32 * distance / 20.0;
        camera.eye += camera.up * self.movement.y as f32 * distance / 20.0;
        self.movement = Vector2::zero();
        camera.eye = camera.target + (camera.eye - camera.target).normalize() * distance;
    }
}


pub trait ToCgMath {
    fn to_cgmath(self) -> Vector2<f32>;
}

impl ToCgMath for PhysicalPosition<f64> {
    fn to_cgmath(self) -> Vector2<f32> {
        Vector2::new(self.x as _, self.y as _)
    }
}

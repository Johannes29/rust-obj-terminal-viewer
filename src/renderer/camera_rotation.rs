use super::interface::Camera;
use crate::general::positions_3d::Point as Point3;
use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};

/// Rotates a camera around a point (0,0,0) based on input events.
/// Hold left mouse button (or the middle mouse button) and drag to spin the camera.
/// You can also press the c key and then move the mouse to spin the camera.
pub struct CameraInputHelper {
    drag_key_is_down: bool,
    mouse_column: u16,
    mouse_row: u16,
    drag_rotation: DragRotation,
    camera_distance: f32,
}

impl CameraInputHelper {
    pub fn new(terminal_height: u16, terminal_width: u16, camera_distance: f32) -> Self {
        CameraInputHelper {
            drag_key_is_down: false,
            mouse_column: 0,
            mouse_row: 0,
            drag_rotation: DragRotation::new(terminal_height, terminal_width, 2.0, 0.5),
            camera_distance,
        }
    }

    pub fn process_input_events(&mut self, events: Vec<Event>) {
        for event in events {
            match event {
                Event::Mouse(mouse_event) => self.process_mouse_event(mouse_event),
                Event::Key(key_event) => self.process_key_event(key_event),
                Event::Resize(columns, rows) => self
                    .drag_rotation
                    .update_terminal_dimensions((columns, rows)),
                _ => (),
            }
        }
    }

    pub fn apply_to_camera(&self, camera: &mut Camera) {
        self.drag_rotation
            .apply_to_camera(camera, self.camera_distance);
    }

    fn process_mouse_event(&mut self, mouse_event: MouseEvent) {
        (self.mouse_column, self.mouse_row) = (mouse_event.column, mouse_event.row);
        match mouse_event.kind {
            MouseEventKind::Drag(MouseButton::Middle) | MouseEventKind::Drag(MouseButton::Left) => {
                if mouse_event.modifiers == KeyModifiers::NONE {
                    self.drag_rotation
                        .handle_drag(mouse_event.column, mouse_event.row);
                }
            }
            MouseEventKind::Moved => {
                if self.drag_key_is_down {
                    self.drag_rotation
                        .handle_drag(mouse_event.column, mouse_event.row);
                }
            }
            MouseEventKind::Down(MouseButton::Middle) | MouseEventKind::Down(MouseButton::Left) => {
                self.drag_rotation
                    .handle_drag_start(mouse_event.column, mouse_event.row)
            }
            _ => (),
        }
    }

    fn process_key_event(&mut self, key_event: KeyEvent) {
        if key_event.code == KeyCode::Char('c') {
            self.drag_key_is_down = !self.drag_key_is_down;
        }
        if self.drag_key_is_down {
            self.drag_rotation
                .handle_drag_start(self.mouse_column, self.mouse_row)
        }
    }
}

#[derive(Debug)]
struct Rotation {
    around_x: f32,
    around_y: f32,
}

impl Rotation {
    fn new() -> Self {
        Rotation {
            around_x: 0.0,
            around_y: 0.0,
        }
    }

    fn sum(rotation_1: &Rotation, rotation_2: &Rotation) -> Self {
        Rotation {
            around_x: rotation_1.around_x + rotation_2.around_x,
            around_y: rotation_1.around_y + rotation_2.around_y,
        }
    }
}

#[derive(Debug)]
struct CellPosition {
    row: u16,
    column: u16,
}

impl CellPosition {
    fn new() -> Self {
        CellPosition { row: 0, column: 0 }
    }

    fn relative_xy_to(&self, other: &Self) -> (i16, i16) {
        (
            self.column as i16 - other.column as i16,
            self.row as i16 - other.row as i16,
        )
    }
}

/// drag refers to when you move the mouse with a special modifier pressed
#[derive(Debug)]
struct DragRotation {
    drag_start_pos: CellPosition,
    rotation_before_drag: Rotation, // rotation when drag started
    drag_rotation: Rotation,        // rotation since drag started
    terminal_height: u16,
    terminal_width: u16,
    rotation_speed: f32,
    char_aspect_ratio: f32,
}

impl DragRotation {
    fn new(
        terminal_height: u16,
        terminal_width: u16,
        rotation_speed: f32,
        char_aspect_ratio: f32,
    ) -> Self {
        DragRotation {
            drag_start_pos: CellPosition::new(),
            rotation_before_drag: Rotation::new(),
            drag_rotation: Rotation::new(),
            terminal_height,
            terminal_width,
            rotation_speed,
            char_aspect_ratio,
        }
    }

    fn handle_drag_start(&mut self, current_column: u16, current_row: u16) {
        self.rotation_before_drag = self.get_rotation();
        self.drag_start_pos = CellPosition {
            column: current_column,
            row: current_row,
        };
        self.update_drag_rotation(0, 0);
    }

    fn handle_drag(&mut self, current_column: u16, current_row: u16) {
        let current_pos = CellPosition {
            column: current_column,
            row: current_row,
        };
        let (relative_x, relative_y) = current_pos.relative_xy_to(&self.drag_start_pos);
        self.update_drag_rotation(relative_x, relative_y);
    }

    /// Uses the right hand coordinate system.
    fn apply_to_camera(&self, camera: &mut Camera, distance: f32) {
        // Assumes that camera is pointing towards -Z when rotation is 0.
        let (rotation_around_x, rotation_around_y) = self.get_rotation_xy();
        let x = rotation_around_y.sin() * rotation_around_x.cos() * distance;
        let y = rotation_around_x.sin() * distance;
        let z = rotation_around_y.cos() * rotation_around_x.cos() * distance;
        camera.position = Point3 { x, y, z };
        let (rotation_x, rotation_y) = self.get_rotation_xy();
        camera.rotation_around_x = -rotation_x;
        camera.rotation_around_y = rotation_y;
    }

    fn update_terminal_dimensions(&mut self, new_dimensions: (u16, u16)) {
        self.terminal_height = new_dimensions.1;
        self.terminal_width = new_dimensions.0;
    }

    fn get_rotation(&self) -> Rotation {
        Rotation::sum(&self.rotation_before_drag, &self.drag_rotation)
    }

    fn get_rotation_xy(&self) -> (f32, f32) {
        let rotation = self.get_rotation();
        (rotation.around_x, rotation.around_y)
    }

    fn update_drag_rotation(&mut self, relative_x: i16, relative_y: i16) {
        let x_movement = relative_x as f32 / self.terminal_width as f32;
        let y_movement = relative_y as f32 / self.terminal_height as f32;
        let window_aspect_ratio = self.terminal_width as f32 / self.terminal_height as f32;
        let rotation_around_x = y_movement as f32 * self.rotation_speed / self.char_aspect_ratio;
        let rotation_around_y = -x_movement as f32 * self.rotation_speed * window_aspect_ratio;

        self.drag_rotation = Rotation {
            around_x: rotation_around_x,
            around_y: rotation_around_y,
        }
    }
}

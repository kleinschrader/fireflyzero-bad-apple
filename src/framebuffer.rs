use crate::prelude::*;

const FULL_SCREEN_DATA_SIZE: usize = (size_of::<u8>() * 240 * 160) / 2;
//                                                      Magic Number       Width               Transparency Color
const BUFFER_DATA_SIZE: usize =
    FULL_SCREEN_DATA_SIZE + size_of::<u8>() + size_of::<u16>() + size_of::<u8>();

pub struct Framebuffer {
    frame_count: usize,
    buffer: [u8; BUFFER_DATA_SIZE],
}

const ANIM_DATA: &[u8] = include_bytes!("../out.bin");
const ANIM_DATA_FRAME_SIZE: usize = (240 * 160) / 8;

impl Framebuffer {
    pub const fn new() -> Self {
        let mut buffer = [0; BUFFER_DATA_SIZE];
        buffer[0] = 0x22;

        let size: [u8; 2] = 240u16.to_le_bytes();

        buffer[1] = size[0];
        buffer[2] = size[1];
        buffer[3] = 0xff;

        Framebuffer {
            frame_count: 0,
            buffer: buffer,
        }
    }

    pub fn render(&mut self) {
        let current_frame = self.frame_count / 2;
        let current_frame_beginning = current_frame * ANIM_DATA_FRAME_SIZE;
        let current_frame_end = current_frame_beginning + ANIM_DATA_FRAME_SIZE;

        let mut current_pixel = 0;

        for byte in &ANIM_DATA[current_frame_beginning..current_frame_end] {
            for bit in 0..8 {
                let v = (byte >> (7-bit)) & 1;

                let new_color: u8 =  match v {
                    0 => 0b0001,
                    _ => 0b0010
                };

                let current_index = (current_pixel/2) + 4;

                let current_val = self.buffer[current_index];

                if (current_pixel & 1) == 0 {
                    let current_val = current_val & 0b00001111;
                    self.buffer[current_index] = current_val | (new_color << 4);
                }
                else {
                    let current_val = current_val & 0b11110000;
                    self.buffer[current_index] = current_val | new_color;
                }

                current_pixel += 1;
            }
        };


        let image = unsafe { ff::Image::from_bytes(&self.buffer) };

        ff::graphics::draw_image(&image, ff::Point { x: 0, y: 0 });
        self.frame_count += 1;
    }

    pub fn set_full(&mut self, value: u8) {
        for x in 0..240 {
            for y in 0..160 {
                self.set_at(x, y, value);
            }
        }
    }

    #[inline]
    fn set_at(&mut self, x: usize, y: usize, value: u8) {
        let is_odd = x & 0b000001;
        let x = x & 0b11111110;

        let pos = (x / 2) + (y * (240 / 2));

        let pos_offset = pos + 4;

        let old_value = self.buffer[pos_offset];

        if is_odd == 0 {
            let new_value = old_value & 0b00001111;
            let new_value = new_value | ((value << 4) & 0b11110000);
            self.buffer[pos_offset] = new_value;
        } else {
            let new_value = old_value & 0b11110000;
            let new_value = new_value | value;
            self.buffer[pos_offset] = new_value;
        }
    }
}

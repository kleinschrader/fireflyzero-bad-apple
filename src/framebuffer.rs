use crate::prelude::*;

mod binary_color;


const FULL_SCREEN_DATA_SIZE: usize = (size_of::<u8>() * 240 * 160) / 2;
//                                                      Magic Number       Width               Transparency Color
const BUFFER_DATA_SIZE: usize =
    FULL_SCREEN_DATA_SIZE + size_of::<u8>() + size_of::<u16>() + size_of::<u8>();

const TOTAL_PIXEL_COUNT: usize = 240*160;


pub struct Framebuffer {
    frame_count: usize,
    buffer: [u8; BUFFER_DATA_SIZE],
    current_color: binary_color::BColor,
    anim_data: ff::FileBuf,
    anim_data_reader: core::slice::Iter<'static, u8>,
    remaining_run: u8
}


impl Framebuffer {
    pub fn new() -> Self {
        let mut buffer = [0; BUFFER_DATA_SIZE];
        buffer[0] = 0x22;

        let size: [u8; 2] = 240u16.to_le_bytes();

        buffer[1] = size[0];
        buffer[2] = size[1];
        buffer[3] = 0xff;

        let anim_data = ff::load_file_buf("video").unwrap();

        let mut anim_data_reader: core::slice::Iter<'static, u8> = unsafe {
            core::mem::transmute(anim_data.as_bytes().iter())
        };


        let remaining_run = unsafe { *anim_data_reader.next().unwrap_unchecked() };

        Framebuffer {
            frame_count: 0,
            buffer: buffer,
            current_color: binary_color::BColor::Off,
            anim_data,
            anim_data_reader,
            remaining_run
        }
    }

    pub fn render(&mut self) {
        if (self.frame_count & 1) == 1 {
            self.frame_count += 1;
            return;
        }

        for pixel_count in 0..TOTAL_PIXEL_COUNT {
            let odd_pixel = match pixel_count & 1 {
                0 => false,
                _ => true
            };

            let idx_pos = pixel_count / 2;

            let old_value = self.buffer[idx_pos + 4];

            while self.remaining_run == 0 {
                self.remaining_run = match self.anim_data_reader.next() {
                    Some(x) => *x,
                    None => {
                        ff::quit();
                        return;
                    },
                };
                self.current_color = self.current_color.flip();
            }

            let new_value = match odd_pixel {
                false => {
                    (0b00001111 & old_value) | self.current_color.to_bits_lhs()
                },
                true => {
                    (0b11110000 & old_value) | self.current_color.to_bits_rhs()
                }
            };

            self.buffer[idx_pos + 4] = new_value;

            self.remaining_run -= 1;
        }


        let image = unsafe { ff::Image::from_bytes(&self.buffer) };

        ff::graphics::draw_image(&image, ff::Point { x: 0, y: 0 });
        self.frame_count += 1;
    }

}

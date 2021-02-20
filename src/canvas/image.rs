pub type ImageDataArray = Vec<u8>;

pub trait ImageDataInterface {
    fn get_data(&self) -> ImageDataArray;
    fn set_data(&mut self, data: ImageDataArray);

    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
}

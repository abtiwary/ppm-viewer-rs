use std::error::Error;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;
use std::io::{BufReader, Read};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct PPMReader {
    is_valid_ppm: bool,
    header_bytes: Option<Rc<RefCell<Vec<u8>>>>,
    comments: Option<Rc<RefCell<Vec<String>>>>,
    metadata: Option<Rc<RefCell<Vec<String>>>>,
    width: usize,
    height: usize,
    bit_depth: usize,
    image_data: Option<Rc<RefCell<Vec<u8>>>>,
}

impl PPMReader {
    pub fn new() -> Self {
        PPMReader { 
            is_valid_ppm: false, 
            header_bytes: None, 
            comments: None,
            metadata: None,
            width: 0, 
            height: 0, 
            bit_depth: 0,
            image_data: None,
        }
    }

    fn is_valid_ppm(magic_vec: &[u8]) -> bool {
        if magic_vec.len() != 2 {
            return false;
        }

        let ret = match magic_vec {
            [80, 54] => true,
            _ => false,
        };
        ret
    }

    fn read_header_data(
        ppm_file: &str,
        header_bytes: &mut Vec<u8>,
        comments: &mut Vec<String>,
        metadata: &mut Vec<String>,
    ) -> Result<(usize, usize, usize), Box<dyn Error>> {
        let mut ppm_file_path = String::from(ppm_file);
        let mut f = File::open(ppm_file_path)?;
        let mut reader = BufReader::new(&f);
        reader.seek(SeekFrom::Start((3)));

        let mut pos: usize= 0;
        let mut metadata_lines_read: usize = 0;
        let mut reading_comment = false;
        let mut reading_metadata = true;
        let mut temp_buffer: Vec<u8> = Vec::new();
        loop {
            let mut byt = vec![0; 1];
            reader.read_exact(&mut byt)?;
            let byt = byt[0];

            match byt {
                0x23 => {
                    reading_comment = true;
                    &temp_buffer.push(byt);
                    pos += 1;
                },
                0x0A => {
                    if reading_comment {
                        let mut comment_str = String::from_utf8_lossy(&temp_buffer).to_string();
                        //println!("{:?}", &comment_str);
                        &comments.push(String::from(&comment_str));
                        reading_comment = false;
                        &temp_buffer.clear();
                    } else if reading_metadata {
                        let mut metadata_str = String::from_utf8_lossy(&temp_buffer).to_string();
                        //println!("{:?}", &metadata_str);
                        &metadata.push(String::from(&metadata_str));
                        metadata_lines_read += 1;
                    }
                    pos += 1;
                },
                _ => {
                    &temp_buffer.push(byt);
                    pos += 1;
                },
            }

            if metadata_lines_read == 2 {
                break;
            }
        }

        let mut width: usize = 0;
        let mut height: usize = 0;
        let dims: Vec<&str> = metadata[0].split_whitespace().collect();
        width = dims[0].parse::<usize>()?;
        height = dims[1].parse::<usize>()?;

        Ok((pos, width, height))
    }

    pub fn from_file(ppm_file: &str) -> Result<(Self, usize, usize), Box<dyn Error>> {
        let mut ppm_reader = PPMReader::new();

        // attempt to open the file
        let ppm_file_path = String::from(ppm_file);
        let mut f = File::open(&ppm_file_path)?;

        // if that succeeded, read the magic - for now we only support P6
        let mut magic = vec![0; 2];
        f.read_exact(&mut magic)?;
        let valid_ppm = Self::is_valid_ppm(&magic);

        // read the contents of the header
        let mut header_vec: Vec<u8> = Vec::new();
        let mut comment_vec: Vec<String> = Vec::new();
        let mut metadata_vec: Vec<String> = Vec::new();

        let (pos, width, height) = Self::read_header_data(
            ppm_file, 
            &mut header_vec, 
            &mut comment_vec, 
            &mut metadata_vec,
        )?;

        ppm_reader.header_bytes = Some(Rc::new(RefCell::new(header_vec)));
        ppm_reader.comments = Some(Rc::new(RefCell::new(comment_vec)));
        ppm_reader.metadata = Some(Rc::new(RefCell::new(metadata_vec)));

        let mut image_data: Vec<u8> = vec![0; 3 * width * height];
        println!("{:?}", &image_data.len());

        f.seek(SeekFrom::Start((3 + pos as u64)));
        let _ = f.read_exact(&mut image_data)?;

        ppm_reader.image_data = Some(Rc::new(RefCell::new(image_data)));

        Ok((ppm_reader, width, height))
    }

}

impl PPMReader {
    pub fn get_image_data(self) -> Option<Rc<RefCell<Vec<u8>>>> {
        self.image_data
    }
}


#[cfg(test)]
mod tests {
    use crate::PPMReader;

    #[test]
    fn can_create_using_new() {
        let new_ppm_reader = PPMReader::new();
        assert_eq!(new_ppm_reader.width, 0);
        assert_eq!(new_ppm_reader.height, 0);
        assert_eq!(new_ppm_reader.is_valid_ppm, false);
    }
}

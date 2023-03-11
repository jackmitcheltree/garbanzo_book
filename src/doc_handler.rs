use std::env;
use std::path::Path;
use std::path::PathBuf;

//modules
use crate::iomod;

pub struct DocHandler {
    pub text: Vec<String>,     //text contents of doc
    pub ptr_x: usize, //increases left going to right
    pub ptr_y: usize  //increases top going to bottom
}

impl DocHandler {
    //for initializing a completely new doc
    pub fn new() -> Self {
        let mut text : Vec<String> = vec![String::new()];
        let mut ptr_x : usize = 0;
        let mut ptr_y : usize = 0;

        Self {
            text,
            ptr_x,
            ptr_y
        }
    }//end new def

    //for loading in data from an existing .txt
    pub fn load(path : &PathBuf) -> Self {
        //read in text
        let mut text : Vec<String> = match iomod::load_file_txt(&path) {
            Ok(text) => text,
            Err(e) => panic!("Failed to load {:?} due to {:?}", path, e)
        };

        let num_lines = text.len() - 1;

        //default cursor to last position in doc
        let mut ptr_x = text[num_lines].chars().count(); //cannot use String.len() as it returns number of bytes 
        let mut ptr_y = num_lines;

        Self {
            text,
            ptr_x,
            ptr_y
        }

    }//end load def

    pub fn backspace(&mut self) {
        let line = self.text[self.ptr_y].clone();
        let line = remove_char(&line, self.ptr_x);
        
        match (self.ptr_x, self.ptr_y) {
            (0, 0) => {}, //do nothing

            (0, _) => {   //append to line above
                self.ptr_y -= 1;
                self.ptr_x = self.text[self.ptr_y].chars().count();
                self.text[self.ptr_y].push_str(&line); 
                self.text.remove(self.ptr_y + 1);
            }, 

            (_, _) => {   //update line
                self.ptr_x -= 1;
                self.text[self.ptr_y] = String::from(line);
            } 
        };

    }//end backspace

    pub fn update(&mut self, x : char) {
        let line = self.text[self.ptr_y].clone();
        let line = add_char(&line, self.ptr_x, x);
        self.text[self.ptr_y] = line;
        self.ptr_x += 1;
    }

    pub fn newline(&mut self) {
        let line = &self.text[self.ptr_y][self.ptr_x..];
        self.text.insert(self.ptr_y + 1, String::from(line) );
        self.text[self.ptr_y].truncate(self.ptr_x);
        self.ptr_y += 1;
        self.ptr_x = 0;
    }

    pub fn update_cursor(&mut self, code : &str) {//todo add functionality to keep pressing right or left onto a new line above or below
        match (self.ptr_x, self.ptr_y, code) {
            (_, 0, "up") => {}, //do nothing
            (0, _, "left") => {}, //do nothing

            (_, _, "up") => self.ptr_y -= 1,
            (_, _, "down") => self.ptr_y += 1,
            (_, _, "left") => self.ptr_x -= 1,
            (_, _, "right") => self.ptr_x += 1,
            _ => println!("Invalid code passed to doc_handler.update_cursor(code).")
        };

        if self.ptr_y > self.text.len() - 1 {
            self.ptr_y = self.text.len() - 1 
        };

        if self.ptr_x > self.text[self.ptr_y].chars().count() {
            self.ptr_x = self.text[self.ptr_y].chars().count()
        };
    }
}

fn remove_char(line : &str, index : usize) -> String {
    match index {
        0 => line.to_string(),
        _ => {
            let (p1, p2) = line.split_at(index);
            let (p3, p4) = p1.split_at(index - 1);
            format!("{}{}", p3, p2)
        }
    }
}

fn add_char(line : &str, index : usize, x : char) -> String {
    let (p1, p2) = line.split_at(index);
    format!("{}{}{}", p1, x.to_string(), p2)
}


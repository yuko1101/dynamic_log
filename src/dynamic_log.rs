use std::io::{stdout, Stdout, Write};

pub struct DynamicLog {
    stdout: Stdout,
    chunks: Vec<LogChunk>,
    pub rendered: bool,
    last_rendered_line_count: usize,
}

impl DynamicLog {
    pub fn new() -> DynamicLog {
        DynamicLog {
            stdout: stdout(),
            chunks: vec![],
            rendered: false,
            last_rendered_line_count: 0,
        }
    }

    pub fn render(&mut self) {
        if self.rendered {
            self.clear();
        }
        let lines = self.get_lines();
        let len = lines.len();
        for i in 0..len {
            if i == len - 1 {
                print!("{}", lines[i]);
                self.stdout.flush().expect("Failed to flush");
            } else {
                println!("{}", lines[i]);
            }
        }
        self.rendered = true;
        self.last_rendered_line_count = len;
    }

    pub fn clear(&mut self) {
        // Move the cursor up to the start of the last render
        print!("\x1B[{}A", self.last_rendered_line_count - 1);
        // Clear the lines
        print!("\x1B[J");
        self.rendered = false;
        self.last_rendered_line_count = 0;
    }

    pub fn push_chunk(&mut self, id: Option<String>) -> &mut LogChunk {
        let chunk = LogChunk::new(id);
        self.chunks.push(chunk);

        self.chunks.last_mut().unwrap()
    }

    pub fn push_line(&mut self, line: String, render: bool) {
        let last_chunk = self.chunks.last_mut();
        if last_chunk.is_none() {
            panic!("No chunk to push line to");
        }
        last_chunk.unwrap().push_line(line);

        if render && self.rendered {
            self.render();
        }
    }

    pub fn push_lines(&mut self, lines: Vec<String>, render: bool) {
        for line in lines {
            self.push_line(line, false);
        }

        if render && self.rendered {
            self.render();
        }
    }

    pub fn pop_chunk(&mut self, render: bool) -> Option<LogChunk> {
        let popped = self.chunks.pop();

        if render && self.rendered {
            self.render();
        }

        popped
    }

    pub fn pop_line(&mut self, render: bool) -> Option<String> {
        let last_chunk = self.chunks.last_mut();
        if last_chunk.is_none() {
            return None;
        }
        let popped = last_chunk.unwrap().lines.pop();

        if render && self.rendered {
            self.render();
        }

        popped
    }

    pub fn get_chunk(&mut self, id: &str) -> &mut LogChunk {
        self.chunks
            .iter_mut()
            .find(|c| c.id.to_owned().is_some_and(|s| s == id))
            .expect("Could not find the chunk")
    }

    pub fn get_chunk_by_index(&mut self, index: i32) -> &mut LogChunk {
        if index < 0 {
            let len = self.chunks.len();
            return &mut self.chunks[(len as i32 + index) as usize];
        }
        &mut self.chunks[index as usize]
    }

    pub fn get_lines(&self) -> Vec<String> {
        self.chunks
            .iter()
            .map(|c| c.lines.clone())
            .collect::<Vec<Vec<String>>>()
            .concat()
    }
}

pub struct LogChunk {
    pub lines: Vec<String>,
    pub id: Option<String>,
}

impl LogChunk {
    pub fn new(id: Option<String>) -> LogChunk {
        LogChunk { lines: vec![], id }
    }

    pub fn push_line(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn push_lines(&mut self, lines: Vec<String>) {
        for line in lines {
            self.push_line(line);
        }
    }

    pub fn pop_line(&mut self) -> Option<String> {
        self.lines.pop()
    }

    pub fn clear_lines(&mut self) {
        self.lines.clear();
    }

    pub fn set_line(&mut self, index: usize, line: String) {
        self.lines[index] = line;
    }
}

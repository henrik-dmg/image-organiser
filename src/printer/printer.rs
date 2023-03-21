use crate::cli::action::Action;
use anyhow::Result;
use std::io::Write;
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

pub struct Printer {
    out_writer: BufferWriter,
    out_buffer: Buffer,
    error_writer: BufferWriter,
    error_buffer: Buffer,
}

impl Default for Printer {
    fn default() -> Self {
        let out_writer = BufferWriter::stdout(ColorChoice::Auto);
        let out_buffer = out_writer.buffer();
        let error_writer = BufferWriter::stderr(ColorChoice::Auto);
        let error_buffer = error_writer.buffer();

        return Self {
            out_writer,
            out_buffer,
            error_writer,
            error_buffer,
        };
    }
}

impl Printer {
    pub fn regular_text(&mut self, string: &String) -> Result<()> {
        self.print_in_color(&string, Color::White)?;
        self.print_in_color("\n", Color::White)?;
        self.out_writer.print(&self.out_buffer)?;
        Ok(())
    }
    pub fn notify_path_handled_successfully(
        &mut self,
        file_name: &str,
        action: Action,
    ) -> Result<()> {
        match action {
            Action::Copy => {
                self.print_in_color("Successfully copied ", Color::White)?;
            }
            Action::Move => {
                self.print_in_color("Successfully moved ", Color::White)?;
            }
        }
        self.print_in_color(&file_name, Color::Green)?;
        self.print_in_color("\n", Color::White)?;
        self.out_writer.print(&self.out_buffer)?;
        Ok(())
    }
    pub fn notify_path_handling_failed(&mut self, file_name: &str) -> Result<()> {
        self.error_buffer
            .set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
        writeln!(
            &mut self.error_buffer,
            "Failed to handle file {}",
            file_name
        )?;
        self.error_writer.print(&self.error_buffer)?;
        Ok(())
    }
    fn print_in_color(&mut self, string: &str, color: Color) -> Result<()> {
        self.out_buffer
            .set_color(ColorSpec::new().set_fg(Some(color)))?;
        write!(self.out_buffer, "{}", string)?;
        Ok(())
    }
}


// Simple shell implementation.

use crate::peripheral::drivers::{uart::{uart_write_byte, uart_write_str}, watchdog};

const BUFFER_SIZE: usize = 128;
const PROMPT: &str = "$ ";

struct ShellState {
  // Buffer for command input
  // TODO: Use a dynamic structure, when dynamic memory allocation is implemented
  command_buffer: [u8; BUFFER_SIZE],
  buf_pos: usize,
}

pub fn shell_main() -> () {
  uart_write_str("Entering shell mode. Primitive commands supported: echo, help\nCommand input restricted to 1024 characters.\n");

  let mut state = ShellState {
    command_buffer: [0; BUFFER_SIZE],
    buf_pos: 0,
  };

  loop {
    // Clear buffer
    for i in 0..BUFFER_SIZE {
      state.command_buffer[i] = 0;
    }
    state.buf_pos = 0;

    uart_write_str(PROMPT);

    'read_loop:
    loop {
      let byte = crate::peripheral::drivers::uart::uart_read_blocking().data();
      match byte {
        b'\r' | b'\n' => {
          uart_write_str("\r\n");
          break 'read_loop;
        }
        8 | 127 => { // Backspace or DEL
          // Find last non-zero character in buffer
          if state.buf_pos > 0 {
            state.buf_pos -= 1;
            // Move cursor back, print space, move cursor back again
            uart_write_str("\x08 \x08");
          }
        }
        b if b.is_ascii_graphic() || b == b' ' => {
          if state.buf_pos < BUFFER_SIZE {
            state.command_buffer[state.buf_pos] = b;
            state.buf_pos = state.buf_pos + 1;
            // Echo the character
            uart_write_byte(b);
          }
        }
        _ => {
          // Ignore.
        }
      }
    }

    // Process command
    process_command(unsafe { core::mem::transmute::<&[u8], &Command>(&state.command_buffer[0..state.buf_pos]) });
  }
}

#[repr(transparent)]
struct Command(str);

impl Command {
  pub fn from_u8_slice(slice: &[u8]) -> &Self {
    unsafe { core::mem::transmute::<&[u8], &Command>(slice) }
  }

  fn as_str(&self) -> &str {
    &self.0
  }

  pub fn command(&self) -> &str {
    // Find first space or end of string
    for (i, c) in self.as_str().chars().enumerate() {
      if c.is_whitespace() {
        return &self.as_str()[0..i];
      }
    }
    self.as_str()
  }

  pub fn argument(&self, index: usize) -> Option<&str> {
    let mut start = 0;
    // Find n-th space or return None if no space found
    let mut nth = -1;
    for c in self.as_str().chars() {
      if c.is_whitespace() {
        nth += 1;
        if nth == index as isize {
          start += c.len_utf8();
          break;
        }
      }
      start += c.len_utf8();
    }

    if nth < index as isize {
      return None;
    }

    // Find next space or end of string
    let mut end = start;
    for c in self.as_str()[start..].chars() {
      if c.is_whitespace() {
        break;
      }
      end += c.len_utf8();
    }
    Some(&self.as_str()[start..end])
  }
}

fn process_command(command: &Command) -> () {
  match command.command() {
    "echo" => {
      let mut i = 0;
      while let Some(arg) = command.argument(i) {
        uart_write_str(arg);
        uart_write_byte(b' ');
        i += 1;
      }
      uart_write_str("\r\n");
    }
    "help" => {
      uart_write_str("Supported commands:\r\n");
      uart_write_str("  echo [text] - prints the text back to the terminal\r\n");
      uart_write_str("  help - prints this help message\r\n");
      uart_write_str("  shutdown - shuts down the system\r\n");
    }
    "shutdown" => {
      watchdog::power_off();
    }
    "" => {
      // Do nothing for empty command
    }
    _ => {
      uart_write_str("Unknown command \"");
      uart_write_str(command.command());
      uart_write_str("\". Type 'help' for a list of commands.\r\n");
    }
  }
}
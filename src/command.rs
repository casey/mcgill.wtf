use super::*;

pub(crate) trait Command {
  fn build(&mut self, arguments: &str) -> &mut Cmd;
}

impl Command for Cmd {
  fn build(&mut self, arguments: &str) -> &mut Self {
    arguments
      .trim()
      .split(' ')
      .filter(|argument| !argument.is_empty())
      .map(|argument| argument.trim())
      .for_each(|argument| *self = self.arg(argument).to_owned());
    self
  }
}

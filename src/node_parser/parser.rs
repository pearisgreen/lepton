
use env::Env;
use crate::node::Node;
use crate::node_parser::NodeParser;

trait Parser {
  fn parse(input: &mut NodeParser, env: &mut Env) -> Option<Node>;
}

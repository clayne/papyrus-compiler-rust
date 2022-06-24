use crate::ast::event::Event;
use crate::ast::function::Function;
use crate::ast::identifier::Identifier;
use crate::ast::node::{display_optional_nodes, Node};
use crate::choose_optional;
use crate::parser::{Parse, Parser, ParserResult};
use papyrus_compiler_lexer::syntax::keyword_kind::KeywordKind;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct State<'source> {
    pub is_auto: bool,
    pub name: Node<Identifier<'source>>,
    pub contents: Option<Vec<Node<StateContent<'source>>>>,
}

impl<'source> State<'source> {
    pub fn new(
        is_auto: bool,
        name: Node<Identifier<'source>>,
        contents: Option<Vec<Node<StateContent<'source>>>>,
    ) -> Self {
        Self {
            is_auto,
            name,
            contents,
        }
    }
}

impl<'source> Display for State<'source> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_auto {
            write!(f, "Auto ")?;
        }

        write!(f, "State {}", self.name)?;

        display_optional_nodes(&self.contents, "\n", f)?;

        write!(f, "\nEndState")?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum StateContent<'source> {
    Function(Function<'source>),
    Event(Event<'source>),
}

impl<'source> Display for StateContent<'source> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StateContent::Function(function) => write!(f, "{}", function),
            StateContent::Event(event) => write!(f, "{}", event),
        }
    }
}

/// ```ebnf
/// <state content> = (<function> | <event>)
/// ```
impl<'source> Parse<'source> for StateContent<'source> {
    fn parse(parser: &mut Parser<'source>) -> ParserResult<'source, Self> {
        choose_optional!(
            parser,
            "State Contents",
            parser.parse_optional::<Event>().map(StateContent::Event),
            parser
                .parse_optional::<Function>()
                .map(StateContent::Function)
        )
    }
}

/// ```ebnf
/// <state> ::= ['Auto'] 'State' <identifier> <state content>* 'EndState'
/// ```
impl<'source> Parse<'source> for State<'source> {
    fn parse(parser: &mut Parser<'source>) -> ParserResult<'source, Self> {
        let is_auto = parser
            .optional(|parser| parser.expect_keyword(KeywordKind::Auto))
            .is_some();

        parser.expect_keyword(KeywordKind::State)?;

        let state_name = parser.parse_node::<Identifier>()?;

        let state_content = parser.parse_node_optional_repeated::<StateContent>();

        parser.expect_keyword(KeywordKind::EndState)?;

        Ok(State::new(is_auto, state_name, state_content))
    }
}

#[cfg(test)]
mod test {
    use crate::ast::node::Node;
    use crate::ast::state::State;
    use crate::parser::test_utils::run_test;

    #[test]
    fn test_state_parser() {
        let src = "Auto State MyState EndState";
        let expected = State::new(true, Node::new("MyState", 11..18), None);

        run_test(src, expected);
    }
}

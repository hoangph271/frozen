use html_parser::{Dom, DomVariant, Node};
use iced::{Column, Element as IcedElement, Row, Text};

fn node_to_element(node: Node) -> Option<IcedElement<'static, ()>> {
    match node {
        html_parser::Node::Element(element) => match element.name.as_str() {
            "div" => {
                let mut col = Row::new();

                let children = element
                    .children
                    .iter()
                    .filter_map(|child| node_to_element(child.clone()));

                for child in children {
                    col = col.push(child);
                }

                let col: IcedElement<()> = col.into();
                Some(col)
            }
            "h4" => {
                let mut col = Row::new();

                let children = element
                    .children
                    .iter()
                    .filter_map(|child| node_to_element(child.clone()));

                for child in children {
                    col = col.push(child);
                }

                let col: IcedElement<()> = col.into();
                Some(col)
            }
            "quote" => {
                let mut col = Row::new();

                let children = element
                    .children
                    .iter()
                    .filter_map(|child| node_to_element(child.clone()));

                for child in children {
                    col = col.push(child);
                }

                let col: IcedElement<()> = col.into();
                Some(col)
            }
            _ => {
                todo!("element NOT matched: {:?}", element.name);
            }
        },
        html_parser::Node::Text(content) => {
            let text = Text::new(content).size(16);

            let text: IcedElement<()> = text.into();
            Some(text)
        }
        _ => {
            todo!("node NOT matched: {:?}", node);
        }
    }
}
fn dom_to_element(dom: Dom) -> Option<Vec<IcedElement<'static, ()>>> {
    match dom.tree_type {
        DomVariant::DocumentFragment => {
            let children = dom
                .children
                .iter()
                .filter_map(|child| node_to_element(child.clone()))
                .collect::<Vec<IcedElement<()>>>();

            Some(children)
        }
        DomVariant::Empty => None,
        _ => {
            todo!("dom_to_element NOT matched: {:?}", dom.tree_type);
        }
    }
}

#[allow(dead_code)]
pub fn parse_dom(html: &str) -> Result<IcedElement<()>, html_parser::Error> {
    let mut container = Column::new();
    let children = {
        let dom = Dom::parse(html)?;
        dom_to_element(dom).unwrap_or_default()
    };

    for child in children {
        container = container.push(child);
    }

    Ok(container.into())
}

#[cfg(test)]
mod tests {
    use iced::{Element, Row};

    use super::parse_dom;

    struct AssertWrapper<'a, T>(Element<'a, T>);

    impl<'a, T> PartialEq for AssertWrapper<'a, T> {
        fn eq(&self, other: &Self) -> bool {
            let el_1 = &self.0;
            let el_2 = &other.0;

            if el_1.width() != el_2.width() {
                return false;
            }

            // TODO: More assert here...!

            true
        }
    }

    #[test]
    fn test_parse_empty_dom() {
        let dom = parse_dom("").unwrap();
        let expected: Element<()> = Row::new().into();

        assert!(AssertWrapper(dom) == AssertWrapper(expected));
    }

    #[test]
    fn test_dummy_app_index() {
        let dom = parse_dom(
            "
            <div>
                <h4>
                    `RC` - The `iced-rs` content focus browser
                </h4>
                <button>
                    Exit app
                </button>
            </div>
        ",
        )
        .unwrap();

        let expected: Element<()> = Row::new().into();

        assert!(AssertWrapper(dom) == AssertWrapper(expected))
    }
}

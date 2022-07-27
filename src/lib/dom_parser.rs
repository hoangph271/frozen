use html_parser::{Dom, DomVariant};
use iced::{Column, Element as IcedElement, Row};

pub fn dom_to_element<'a, T: 'a>(dom: &Dom) -> Option<Vec<IcedElement<'a, T>>> {
    match dom.tree_type {
        DomVariant::DocumentFragment => {
            let children = dom
                .children
                .iter()
                .map(|node| match node {
                    html_parser::Node::Element(element) => match element.name.as_str() {
                        "div" => {
                            let col = Column::new();
                            let col: IcedElement<T> = col.into();

                            col
                        }
                        _ => {
                            todo!("element NOT matched: {:?}", element.name);
                        }
                    },
                    _ => {
                        todo!("node NOT matched: {:?}", node);
                    }
                })
                .collect::<Vec<IcedElement<'a, T>>>();

            Some(children)
        }
        DomVariant::Empty => {
            println!("{:?}", dom);
            None
        }
        _ => {
            todo!("dom_to_element NOT matched: {:?}", dom.tree_type);
        }
    }
}

#[allow(dead_code)]
pub fn parse_dom<'a, T: 'a>(html: &str) -> Result<IcedElement<'a, T>, html_parser::Error> {
    let container = Row::new();
    let dom = Dom::parse(html)?;

    if let Some(_children) = dom_to_element::<()>(&dom) {
        // TODO: children
        // for child_el in root_el {
        //     container.push(child_el);
        // }
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
        let dom = parse_dom::<()>("").unwrap();
        let expected: Element<()> = Row::new().into();

        assert!(AssertWrapper(dom) == AssertWrapper(expected));
    }

    #[test]
    fn test_dummy_app_index() {
        let dom = parse_dom::<()>(
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

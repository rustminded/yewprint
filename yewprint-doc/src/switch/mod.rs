mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Switch, H1, H5};

pub struct SwitchDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for SwitchDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        SwitchDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                disabled: false,
                inline: false,
                large: false,
                align_right: false,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.state = msg;
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let example_props = self.state.clone();
        let source = crate::include_raw_html!(
            concat!(env!("OUT_DIR"), "/", file!(), ".html"),
            "bp3-code-block"
        );

        html! {
            <div>
                <H1 class=classes!("docs-title")>{"Switch"}</H1>
                <SourceCodeUrl />
                <ExampleContainer
                    source=source
                    props=Some(html! {
                        <SwitchProps
                            callback={self.callback.clone()}
                            props=example_props.clone()
                        >
                        </SwitchProps>
                    })
                >
                    <Example with example_props />
                </ExampleContainer>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    SwitchProps for ExampleProps =>
    fn view(&self) -> Html {
        html! {
            <div>
                <H5>{"Props"}</H5>
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        disabled: !props.disabled,
                        ..props
                    })
                    checked=self.props.disabled
                    label=html!("Disabled")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        inline: !props.inline,
                        ..props
                    })
                    checked=self.props.inline
                    label=html!("Inline")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        large: !props.large,
                        ..props
                    })
                    checked=self.props.large
                    label=html!("Large")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        align_right: !props.align_right,
                        ..props
                    })
                    checked=self.props.align_right
                    label=html!("Align right")
                />
            </div>
        }
    }
}

crate::build_source_code_component!();

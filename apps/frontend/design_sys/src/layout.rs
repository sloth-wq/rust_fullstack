pub mod layout {
    use stylist::style;
    use yew::{
        prelude::{function_component, html, Html},
        Children, Properties,
    };

    #[derive(Properties, PartialEq)]
    pub struct Props {
        #[prop_or_default]
        pub children: Children,
    }

    #[function_component(DefaultLayout)]
    pub fn default_layout(Props { children }: &Props) -> Html {
        let link_items = [
            "AppButton",
            "AppTypography",
            "Stack",
            "AppBox",
            "AppInput",
            "AppTextarea",
            "AppCheckbox",
        ];

        let sidebar_style = style!(
            r#"
            position: fixed;
            top: 0;
            left: 0;
            height: 100%;
            width: 240px;
            background-color: #FFFFFF;
            box-shadow: 0 0 10px rgba(0, 0, 0, 0.3);
            z-index: 1;
            overflow-y: auto;
            padding-top: 64px;
        "#
        )
        .expect("Failed to mount style");

        let list_style = style!(
            r#"
            list-style: none;
            margin: 0;
            padding: 0;
        "#
        )
        .expect("Failed to mount style");

        let list_item_style = style!(
            r#"
            padding: 12px 24px;
            border-bottom: 1px solid #e0e0e0;
        "#
        )
        .expect("Failed to mount style");

        let link_style = style!(
            r#"
            color: #333;
            text-decoration: none;
            display: block;
            font-size: 14px;
            line-height: 24px;
            transition: all 0.3s ease;
        "#
        )
        .expect("Failed to mount style");

        let logo_style = style!(
            r#"
            position: absolute;
            top: 0;
            left: 0;
            background-color: #003EE5;
            color: #fff;
            height: 64px;
            width: 100%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 24px;
            font-weight: bold;
        "#
        )
        .expect("Failed to mount style");

        let content_style = style!(
            r#"
            margin-left: 240px;
            padding: 40px 80px;
        "#
        )
        .expect("Failed to mount style");

        html!(
            <>
                <div class={sidebar_style}>
                    <div><a class={logo_style} href="/">{"Design System"}</a></div>
                    <ul class={list_style}>
                        {link_items.iter().map(|item| html!(
                            <li class={list_item_style.clone()}><a class={link_style.clone()} href={format!("/ui/{}", &item)}>{&item}</a></li>
                        )).collect::<Vec<Html>>()}
                    </ul>
                </div>

                <div class={content_style}>
                    { for children.iter() }
                </div>
            </>
        )
    }
}

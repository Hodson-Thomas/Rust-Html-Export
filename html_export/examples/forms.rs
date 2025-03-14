use html_export::{
    composed::forms::{
        fields::{
            checkbox_input::create_labeled_checkboxes,
            color_input,
            date_input::{self, DateInputConfig},
            email_field::{create_labeled_email_input, EmailInputConfig},
            field::InputFieldConfig,
            file_input::{create_labeled_file_input, FileInputConfig},
            hidden_inputs::create_hidden_input,
            month_field::{create_labeled_mount_input, MonthInputConfig},
            number_input::{create_labeled_number_input, NumberInputConfigs},
            password_field::{create_labeled_password_input, PasswordInputConfig},
            radio_input::create_labeled_radios,
            range_input::{create_labeled_range_input, RangeInputConfigs},
            reset_input::create_labeled_reset_input,
            select_input::{
                create_labeled_select, Options, SelectInputConfigs, SelectOption, SelectOptionGroup,
            },
            submit_input::{create_submit_input, SubmitInputConfigs},
            tel_input::{create_labeled_tel_input, TelInputConfigs},
            text_input::{create_label_text_field, TextInputConfig},
            textaera_input::{create_labeled_text_area, TextAreaInputConfigs},
            time_input::{create_labeled_time_input, TimeInputConfigs},
            url_input::{create_labeled_url_input, UrlInputConfigs},
            week_input::{create_labeled_week_input, WeekInputConfigs},
        },
        form::{create_form, wrap_fields_in_fieldset, AsForm, FormConfig},
    },
    element::{Element, HtmlElementConfig},
    export_to_file,
    head::Head,
};

pub struct Car {
    pub tyres: Tyres,
    pub description: String,
    pub power: f64,
    pub model_name: String,
    pub brand: CarBrand,
    pub motor: Motor,
    pub color: String,
    pub bought_the: String,
    pub dealership: DealerShip,
    pub hidden_id: String,
    pub length: f64,
    pub last_start_time: String,
    pub benchmark_week: String,
}

pub enum Tyres {
    Winter,
    Summer,
    Hybrid,
    Hard,
    Medium,
    Slick,
}

pub enum CarBrand {
    Toyota,
    Xiaomi,
    Byd,
    Ferrari,
    MercedesBenz,
    Porsche,
    Volkswagen,
    Bmw,
}

pub struct Motor {
    is_thermic: bool,
    is_electric: bool,
}

pub struct DealerShip {
    warranty: String,
    email: String,
    _bill: String,
    secret_id: String,
    phone: String,
    website: String,
}

impl AsForm for Car {
    fn as_form_field(&self) -> Option<Element> {
        let model_name_field = create_label_text_field(
            InputFieldConfig::new(
                "model-name".to_string(),
                "model-name".to_string(),
                "Model name".to_string(),
            ),
            TextInputConfig::new().with_place_holder("Enter model name".to_string()),
            Some(self.model_name.clone()),
        );
        let brand_field = self.brand.as_form_field().unwrap();
        let vehicul_fieldset = wrap_fields_in_fieldset(
            vec![model_name_field, brand_field],
            "Vehicule model".to_string(),
            HtmlElementConfig::new_empty(),
            HtmlElementConfig::new_empty(),
        );

        let motor_fieldset = wrap_fields_in_fieldset(
            vec![self.motor.as_form_field().unwrap()],
            "Motor".to_string(),
            HtmlElementConfig::new_empty(),
            HtmlElementConfig::new_empty(),
        );

        let color_input = color_input::create_labeled_color_input(
            InputFieldConfig::new(
                "color".to_string(),
                "color".to_string(),
                "color".to_string(),
            ),
            Some(self.color.clone()),
        );

        let date_input = date_input::create_date_input_with_label(
            InputFieldConfig::new(
                "bought_the".to_string(),
                "bought_the".to_string(),
                "Bought the".to_string(),
            ),
            DateInputConfig::new(false),
            Some(self.bought_the.clone()),
        );
        let dealership = self.dealership.as_form_field().unwrap();
        let hidden = create_hidden_input(
            InputFieldConfig::new(
                "hidden-id".to_string(),
                "hidden-id".to_string(),
                "".to_string(),
            ),
            Some(self.hidden_id.clone()),
        );

        let power = create_labeled_number_input(
            InputFieldConfig::new(
                "hidden-id".to_string(),
                "hidden-id".to_string(),
                "".to_string(),
            ),
            NumberInputConfigs::new(),
            Some(self.power),
        );

        let size = create_labeled_range_input(
            InputFieldConfig::new(
                "size".to_string(),
                "size".to_string(),
                "Size filter".to_string(),
            ),
            RangeInputConfigs::new(3.0, 6.0),
            Some(self.length),
        );

        let start = create_labeled_time_input(
            InputFieldConfig::new(
                "reset".to_string(),
                "reset".to_string(),
                "Reset form".to_string(),
            ),
            TimeInputConfigs::new(),
            Some(self.last_start_time.clone()),
        );

        let benchmark = create_labeled_week_input(
            InputFieldConfig::new(
                "bencmark".to_string(),
                "bencmark".to_string(),
                "Benchmark week".to_string(),
            ),
            WeekInputConfigs::new(),
            Some(self.benchmark_week.clone()),
        );

        let description = create_labeled_text_area(
            InputFieldConfig::new(
                "description".to_string(),
                "description".to_string(),
                "Description".to_string(),
            ),
            TextAreaInputConfigs::new(),
            Some(self.description.clone()),
        );

        let tires = self.tyres.as_form_field().unwrap();

        let reset = create_labeled_reset_input(
            InputFieldConfig::new(
                "reset".to_string(),
                "reset".to_string(),
                "Reset form".to_string(),
            ),
            Some("reset".to_string()),
        );

        let submit = create_submit_input(
            InputFieldConfig::new(
                "submit".to_string(),
                "submit".to_string(),
                "submit".to_string(),
            ),
            SubmitInputConfigs::new(),
        );

        create_form(
            vec![
                vehicul_fieldset,
                color_input,
                motor_fieldset,
                date_input,
                dealership,
                hidden,
                power,
                size,
                benchmark,
                description,
                tires,
                start,
                reset,
                submit,
            ],
            FormConfig::new().with_method("POST".to_string()),
            HtmlElementConfig::new_empty(),
        )
    }
}

impl AsForm for CarBrand {
    fn as_form_field(&self) -> Option<Element> {
        let checked = Some(match self {
            CarBrand::Toyota => 0usize,
            CarBrand::Xiaomi => 1,
            CarBrand::Byd => 2,
            CarBrand::Ferrari => 3,
            CarBrand::MercedesBenz => 4,
            CarBrand::Porsche => 5,
            CarBrand::Volkswagen => 6,
            CarBrand::Bmw => 7,
        });

        let field_config =
            InputFieldConfig::new("car_brand".to_string(), String::new(), String::new());
        let radios = vec![
            (
                "toyota".to_string(),
                "Toyota".to_string(),
                "toyota".to_string(),
            ),
            (
                "xiaomi".to_string(),
                "Xiaomi".to_string(),
                "xiaomi".to_string(),
            ),
            ("byd".to_string(), "Byd".to_string(), "byd".to_string()),
            (
                "ferrari".to_string(),
                "Ferrari".to_string(),
                "ferrari".to_string(),
            ),
            (
                "mercedesBenz".to_string(),
                "MercedesBenz".to_string(),
                "mercedesBenz".to_string(),
            ),
            (
                "porsche".to_string(),
                "Porsche".to_string(),
                "porsche".to_string(),
            ),
            (
                "volkswagen".to_string(),
                "Volkswagen".to_string(),
                "volkswagen".to_string(),
            ),
            ("bmw".to_string(), "Bmw".to_string(), "bmw".to_string()),
        ];
        create_labeled_radios(field_config, checked, radios).ok()
    }
}

impl AsForm for Motor {
    fn as_form_field(&self) -> Option<Element> {
        Some(create_labeled_checkboxes(
            InputFieldConfig::new("motor".to_string(), "".to_string(), "".to_string()),
            vec![
                (
                    "electric-motor".to_string(),
                    "Electric".to_string(),
                    "is_electric".to_string(),
                    self.is_electric,
                ),
                (
                    "thermic-motor".to_string(),
                    "Thermic".to_string(),
                    "thermic".to_string(),
                    self.is_thermic,
                ),
            ],
        ))
    }
}

impl AsForm for DealerShip {
    fn as_form_field(&self) -> Option<Element> {
        let email = create_labeled_email_input(
            InputFieldConfig::new(
                "email".to_string(),
                "email".to_string(),
                "Email".to_string(),
            ),
            EmailInputConfig::new(),
            Some(self.email.to_string()),
        );
        let bill = create_labeled_file_input(
            InputFieldConfig::new("bill".to_string(), "bill".to_string(), "Bill".to_string()),
            FileInputConfig::new(),
        );

        let warranty = create_labeled_mount_input(
            InputFieldConfig::new(
                "warranty".to_string(),
                "warranty".to_string(),
                "Warranty ends at".to_string(),
            ),
            MonthInputConfig::new(),
            Some(self.warranty.clone()),
        );

        let secret_id = create_labeled_password_input(
            InputFieldConfig::new(
                "secret id".to_string(),
                "secret id".to_string(),
                "Secret id".to_string(),
            ),
            PasswordInputConfig::new(),
            Some(self.secret_id.clone()),
        );

        let phone = create_labeled_tel_input(
            InputFieldConfig::new(
                "phone".to_string(),
                "phone".to_string(),
                "Phone".to_string(),
            ),
            TelInputConfigs::new(),
            Some(self.phone.clone()),
        );

        let url = create_labeled_url_input(
            InputFieldConfig::new("url".to_string(), "url".to_string(), "Url".to_string()),
            UrlInputConfigs::new(),
            Some(self.website.clone()),
        );

        Some(wrap_fields_in_fieldset(
            vec![email, bill, warranty, secret_id, phone, url],
            "Dealership".to_string(),
            HtmlElementConfig::new_empty(),
            HtmlElementConfig::new_empty(),
        ))
    }
}

impl AsForm for Tyres {
    fn as_form_field(&self) -> Option<Element> {
        let options1 = vec![
            (
                SelectOption::new("winter".to_string(), "Winter".to_string()),
                HtmlElementConfig::new_empty(),
            ),
            (
                SelectOption::new("summer".to_string(), "Summer".to_string()),
                HtmlElementConfig::new_empty(),
            ),
            (
                SelectOption::new("hybrid".to_string(), "Hybrid".to_string()),
                HtmlElementConfig::new_empty(),
            ),
        ];
        let options2 = vec![
            (
                SelectOption::new("hard".to_string(), "Hard".to_string()),
                HtmlElementConfig::new_empty(),
            ),
            (
                SelectOption::new("medium".to_string(), "Medium".to_string()),
                HtmlElementConfig::new_empty(),
            ),
            (
                SelectOption::new("slick".to_string(), "Slick".to_string()),
                HtmlElementConfig::new_empty(),
            ),
        ];

        let group1 = SelectOptionGroup::new()
            .with_content(options1.into_iter())
            .with_label("Normal".to_string());
        let group2 = SelectOptionGroup::new()
            .with_content(options2.into_iter())
            .with_label("Race".to_string());
        Some(create_labeled_select(
            InputFieldConfig::new(
                "tyres".to_string(),
                "tyres".to_string(),
                "Tyres".to_string(),
            ),
            SelectInputConfigs::new(),
            vec![
                Options::OptionGroup(group1, HtmlElementConfig::new_empty()),
                Options::OptionGroup(group2, HtmlElementConfig::new_empty()),
            ],
        ))
    }
}

fn main() {
    let car = Car {
        tyres: Tyres::Winter,
        benchmark_week: "2025-W18".to_string(),
        length: 4.5,
        power: 600.0,
        hidden_id: "abcdef123".to_string(),
        brand: CarBrand::Ferrari,
        model_name: "Ferrari F80".to_string(),
        last_start_time: "13:45".to_string(),
        motor: Motor {
            is_electric: true,
            is_thermic: true,
        },
        color: "#ff0000".to_string(),
        bought_the: "2018-07-22".to_string(),
        dealership: DealerShip {
            website: "https://www.website.com".to_string(),
            phone: "123-456-7890".to_string(),
            secret_id: "abc123".to_string(),
            email: "car@cars.com".to_string(),
            _bill: String::new(),
            warranty: "2025-06".to_string(),
        },
        description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string()
    };
    let head = Head::new().with_title("Form".to_string());
    export_to_file(
        "examples_output".to_string(),
        "form.html".to_string(),
        head,
        vec![car.as_form_field().unwrap()],
    )
    .unwrap();
}
